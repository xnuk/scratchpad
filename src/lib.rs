#[allow(dead_code)]
pub(crate) mod bindings {
	include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod wrapper;
pub use crate::wrapper::*;

pub use crate::bindings::LibmsiDbFlags as DBFlags;
