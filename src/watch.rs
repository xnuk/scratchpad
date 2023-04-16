use std::path::Path;

use chrono::{offset::Utc, DateTime};
use notify::{recommended_watcher, RecursiveMode, Watcher};
use tokio::sync::broadcast;

pub(super) type ChangeData = DateTime<Utc>;
pub(super) type ChangeDataSender = broadcast::Sender<ChangeData>;
pub(super) type ChangeDataReceiver = broadcast::Receiver<ChangeData>;
pub(super) use notify::RecommendedWatcher;

pub(super) fn watch_state(
	path: impl AsRef<Path>,
) -> (RecommendedWatcher, ChangeDataSender) {
	let (tx, _) = broadcast::channel(10);
	let mut watcher = {
		let tx = tx.clone();
		recommended_watcher(move |_| {
			let _ = tx.send(Utc::now());
		})
		.unwrap()
	};

	watcher
		.watch(path.as_ref(), RecursiveMode::NonRecursive)
		.unwrap();

	(watcher, tx)
}
