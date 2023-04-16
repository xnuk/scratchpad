use std::{
	pin::Pin,
	task::{Context, Poll},
};

use futures_core::stream::{FusedStream, Stream};

use pin_project_lite::pin_project;

struct ScanFn<S, F> {
	state: S,
	func: F,
}

pin_project! {
	pub struct FusedScan<St, S, T, F>
	where
		St: Stream,
		F: FnMut(&mut S, St::Item) -> Option<T>,
	{
		#[pin] original: St,
		func: Option<ScanFn<S, F>>,
	}
}

impl<St, S, T, F> Stream for FusedScan<St, S, T, F>
where
	St: Stream,
	F: FnMut(&mut S, St::Item) -> Option<T>,
{
	type Item = T;

	fn poll_next(
		self: Pin<&mut Self>,
		cx: &mut Context<'_>,
	) -> Poll<Option<Self::Item>> {
		let this = self.project();

		match this.original.poll_next(cx) {
			Poll::Pending => Poll::Pending,
			Poll::Ready(None) => {
				*this.func = None;
				Poll::Ready(None)
			}
			Poll::Ready(Some(item)) => Poll::Ready({
				match this.func.as_mut() {
					None => None,
					Some(func) => {
						let ret = (func.func)(&mut func.state, item);
						if ret.is_none() {
							*this.func = None;
						}
						ret
					}
				}
			}),
		}
	}
}

impl<St, S, T, F> FusedStream for FusedScan<St, S, T, F>
where
	St: Stream,
	F: FnMut(&mut S, St::Item) -> Option<T>,
{
	fn is_terminated(&self) -> bool {
		self.func.is_none()
	}
}

pub trait Scanable: Stream {
	fn fused_scan<S, T, F>(self, initial: S, f: F) -> FusedScan<Self, S, T, F>
	where
		F: FnMut(&mut S, Self::Item) -> Option<T>,
		Self: Sized,
	{
		FusedScan {
			original: self,
			func: Some(ScanFn {
				state: initial,
				func: f,
			}),
		}
	}
}

impl<T: Stream> Scanable for T {}
