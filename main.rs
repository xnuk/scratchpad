use axum::body::StreamBody;
use axum::extract::State;
use axum::handler::Handler;
use axum::http::{header::ACCEPT, HeaderMap};
use axum::{routing, Router};
use futures::future::{self, Either};
use futures::StreamExt as FutureStreamExt;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio_stream::wrappers::{BroadcastStream, IntervalStream};
use tokio_stream::{Stream, StreamExt as TokioStreamExt};

use chrono::{offset::Utc, DateTime};
use notify::{recommended_watcher, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::broadcast;
use tokio::time::{interval, Duration};

type ChangeData = DateTime<Utc>;
type ChangeDataSender = broadcast::Sender<ChangeData>;

fn streamed_body(
	rx: broadcast::Receiver<ChangeData>,
	tick: Duration,
) -> StreamBody<impl Stream<Item = Result<String, Infallible>>> {
	let b = TokioStreamExt::fuse(FutureStreamExt::scan(
		TokioStreamExt::merge(
			TokioStreamExt::map(IntervalStream::new(interval(tick)), |_| {
				Either::Left(())
			}),
			TokioStreamExt::filter_map(BroadcastStream::new(rx), |v| {
				v.ok().map(Either::Right)
			}),
		),
		false,
		|changed, x| {
			future::ready(if *changed {
				None
			} else {
				Some(Ok(if let Either::Right(time) = x {
					*changed = true;
					format!("event: change\ndata: {}\n\n", time.to_rfc3339())
				} else {
					": ping\n\n".to_string()
				}))
			})
		},
	));

	StreamBody::new(b)
}

fn watch_state() -> (RecommendedWatcher, ChangeDataSender) {
	let (tx, _) = broadcast::channel(10);
	let mut watcher = {
		let tx = tx.clone();
		recommended_watcher(move |_| {
			let _ = tx.send(Utc::now());
		})
		.unwrap()
	};

	watcher
		.watch(
			PathBuf::from("./dick").as_path(),
			RecursiveMode::NonRecursive,
		)
		.unwrap();

	(watcher, tx)
}

async fn dick(
	State((tx, tick)): State<(ChangeDataSender, Duration)>,
	headers: HeaderMap,
) -> StreamBody<impl Stream<Item = Result<String, Infallible>>> {
	eprintln!("{:?}", headers.get(ACCEPT));
	streamed_body(tx.subscribe(), tick)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let (watcher, tx) = watch_state();

	// watcher should outlive
	let _ = watcher;

	let app = Router::new().route(
		"/dick",
		routing::get_service(dick.with_state((tx, Duration::from_secs(10)))),
	);

	axum::Server::bind(&SocketAddr::new([127, 0, 0, 1].into(), 8080))
		.serve(app.into_make_service())
		.await?;

	Ok(())
}
