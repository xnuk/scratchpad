use sse_watch::watch_router;

use std::net::SocketAddr;

use axum::Router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let (watcher, service) = watch_router("./dick", 10);

	// watcher should outlive
	let _ = watcher;

	let app = Router::new().route("/dick", service);

	axum::Server::bind(&SocketAddr::new([127, 0, 0, 1].into(), 8080))
		.serve(app.into_make_service())
		.await?;

	Ok(())
}
