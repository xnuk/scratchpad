use super::mime::{media_type, prefered};
use super::scan::Scanable;
use super::watch::{
	watch_state, ChangeDataReceiver, ChangeDataSender, RecommendedWatcher,
};

use std::convert::Infallible;
use std::path::Path;

use axum::body::{HttpBody, StreamBody};
use axum::extract::State;
use axum::handler::Handler;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::routing::method_routing::{get_service, MethodRouter};

use tokio::time::{interval, Duration};
use tokio_stream::wrappers::{BroadcastStream, IntervalStream};
use tokio_stream::{Stream, StreamExt};

fn streamed_body(
	rx: ChangeDataReceiver,
	tick: Duration,
) -> StreamBody<impl Stream<Item = Result<String, Infallible>>> {
	enum Either<L, R> {
		Left(L),
		Right(R),
	}

	let interval =
		IntervalStream::new(interval(tick)).map(|_| Either::Left(()));

	let rx = BroadcastStream::new(rx).filter_map(|v| v.ok().map(Either::Right));

	let b = interval.merge(rx).fused_scan(false, |changed, x| {
		if *changed {
			None
		} else {
			Some(Ok(if let Either::Right(time) = x {
				*changed = true;
				format!("event: change\ndata: {}\n\n", time.to_rfc3339())
			} else {
				": ping\n\n".to_string()
			}))
		}
	});

	StreamBody::new(b)
}

static EVENT_STREAM_CONTENT_TYPE: header::HeaderValue =
	header::HeaderValue::from_static("text/event-stream; charset=utf-8");

async fn route(
	State((tx, tick)): State<(ChangeDataSender, Duration)>,
	headers: HeaderMap,
) -> impl IntoResponse {
	let sse_ok = headers
		.get(header::ACCEPT)
		.and_then(|x| {
			prefered(x.to_str().ok()?, &[media_type!(TEXT / EVENT_STREAM)])
				.map(|_| ())
		})
		.is_some();

	eprintln!("{:?}", headers.get(header::ACCEPT));

	if sse_ok {
		let mut header = HeaderMap::new();
		header.insert(header::CONTENT_TYPE, EVENT_STREAM_CONTENT_TYPE.clone());
		(StatusCode::OK, streamed_body(tx.subscribe(), tick)).into_response()
	} else {
		(StatusCode::NOT_FOUND).into_response()
	}
}

pub fn watch_router<S, B>(
	path: impl AsRef<Path>,
	ping_sec: u64,
) -> (RecommendedWatcher, MethodRouter<S, B>)
where
	S: Clone,
	B: HttpBody + Send + 'static,
{
	let (watcher, tx) = watch_state(path);
	let service = route.with_state((tx, Duration::from_secs(ping_sec)));
	(watcher, get_service(service))
}
