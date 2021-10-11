use std::{
	convert::AsRef,
	ffi::{CStr, CString, OsStr, OsString},
	os::unix::ffi::OsStrExt,
	path::Path,
	ptr::{null, null_mut},
};

use crate::bindings::*;
use glib::gobject_ffi::{g_object_unref, GObject};

macro_rules! gpointer {
	($v:vis $name:ident => $rel:ty) => {
		$v struct $name {
			ptr: $rel
		}

		impl Drop for $name {
			fn drop(&mut self) {
				unsafe {
					g_object_unref(self.ptr as *mut GObject);
				}
			}
		}
	}
}

macro_rules! ptr_check {
	($i:ident $ptr:expr) => {{
		let ptr = $ptr;
		if ptr != null_mut() {
			Some($i { ptr })
		} else {
			None
		}
	}};
}

gpointer!(pub Database => *mut LibmsiDatabase);
gpointer!(pub Query => *mut LibmsiQuery);
gpointer!(pub Record => *mut LibmsiRecord);

impl Database {
	#[must_use]
	pub fn new<P: AsRef<Path>>(
		path: P,
		flags: LibmsiDbFlags,
	) -> Option<Database> {
		let path = CString::new(path.as_ref().as_os_str().as_bytes()).ok()?;

		ptr_check!(Database unsafe {
			libmsi_database_new(path.as_ptr(), flags.0, null(), null_mut())
		})
	}
}

impl Query {
	#[must_use]
	pub fn new<S: AsRef<str>>(database: &Database, query: S) -> Option<Query> {
		let query = CString::new(query.as_ref().as_bytes()).ok()?;

		ptr_check!(Query unsafe {
			libmsi_query_new(database.ptr, query.as_ptr(), null_mut())
		})
	}

	#[must_use]
	pub fn fetch(&mut self) -> Option<Record> {
		ptr_check!(Record unsafe {
			libmsi_query_fetch(self.ptr, null_mut())
		})
	}
}

impl Record {
	#[must_use]
	pub fn new(count: u32) -> Option<Record> {
		ptr_check!(Record unsafe {
			libmsi_record_new(count)
		})
	}

	#[must_use]
	pub fn field_count(&self) -> u32 {
		unsafe { libmsi_record_get_field_count(self.ptr) }
	}

	#[must_use]
	pub fn null(&self, field: u32) -> bool {
		unsafe { libmsi_record_is_null(self.ptr, field) }
	}

	pub fn string(&self, field: u32) -> OsString {
		let string = unsafe {
			CStr::from_ptr(libmsi_record_get_string(self.ptr, field))
		};

		let string = OsStr::from_bytes(string.to_bytes());

		string.to_os_string()
	}
}
