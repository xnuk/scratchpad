use hyper::header::ACCEPT;
use pin_project::pin_project;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::Poll;
use std::{io, task, thread};

use chrono::SecondsFormat;
use chrono::{offset::Utc, DateTime};
use hyper::body::{Body, Bytes, Frame, Incoming};
use hyper::{Method, Request, Response};
use notify::{recommended_watcher, RecursiveMode, Watcher};
use tokio::sync::broadcast;
use tokio::time::{sleep, Duration, Instant, Sleep};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Message {
	Ping,
	Changed,
}

// async fn ping(mut sender: broadcast::Sender<Message>) {
// 	loop {
// 		sleep(Duration::from_secs(3)).await;
// 		let _ = sender.send(Message::Ping);
// 	}
// }

#[pin_project]
struct Dick {
	rx: Option<broadcast::Receiver<DateTime<Utc>>>,

	#[pin]
	timeout: Sleep,
}

impl Dick {
	fn from_rx(rx: broadcast::Receiver<DateTime<Utc>>) -> Self {
		Dick {
			rx: Some(rx),
			timeout: sleep(Duration::from_secs(3)),
		}
	}
	fn empty() -> Self {
		Dick {
			rx: None,
			timeout: sleep(Duration::from_secs(3)),
		}
	}
}

impl Body for Dick {
	type Data = Bytes;
	type Error = ();

	fn poll_frame(
		self: Pin<&mut Self>,
		cx: &mut task::Context<'_>,
	) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
		let this = self.project();
		if let Some(rx) = this.rx {
			if let Ok(t) = rx.try_recv() {
				*this.rx = None;
				Poll::Ready(Some(Ok(Frame::data(Bytes::from(format!(
					"event: change\ndata: {}\n\n",
					t.to_rfc3339()
				))))))
			} else if this.timeout.is_elapsed() {
				this.timeout.reset(Instant::now() + Duration::from_secs(3));
				Poll::Ready(Some(Ok(Frame::data(Bytes::from(": ping\n\n")))))
			} else {
				Poll::Pending
			}
		} else {
			Poll::Ready(None)
		}
	}

	fn is_end_stream(&self) -> bool {
		self.rx.is_none()
	}
}

async fn handle_request(
	req: Request<Incoming>,
) -> hyper::Result<Response<Dick>> {
	if req.method() != Method::GET || req.uri().path() != "/dick" {
		return Ok(Response::builder()
			.status(404)
			.body(Dick::empty())
			.unwrap());
	}
	let headers = req.headers();
	let val = headers.get(ACCEPT);
	println!("{val:?}");

	Ok(todo!())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let (tx, _) = broadcast::channel(10);
	let mut watcher = {
		let tx = tx.clone();
		recommended_watcher(move |_| {
			let _ = tx.send(Instant::now());
		})?
	};

	watcher.watch(
		PathBuf::from("./dick").as_path(),
		RecursiveMode::NonRecursive,
	)?;

	let mut line = String::new();
	while io::stdin().read_line(&mut line).is_ok() {
		if line.trim() == "stop" {
			break;
		} else {
			let mut rx = tx.subscribe();
			match rx.recv().await {
				Err(e) => {
					eprintln!("{e}");
					break;
				}

				Ok(t) => {
					println!("{}ms", t.elapsed().as_millis());
				}
			}
		}
	}

	Ok(())
}
