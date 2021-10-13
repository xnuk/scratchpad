use std::{
	convert::AsRef,
	ffi::{CStr, CString, OsStr, OsString},
	fmt,
	os::unix::ffi::OsStrExt,
	path::Path,
	ptr::{null, null_mut},
};

use crate::bindings::*;
use glib::ffi::{g_free, gpointer};
use glib::gobject_ffi::{g_object_unref, GObject};

macro_rules! gpointer {
	($v:vis $name:ident => $rel:ty $(, $ptr:ident => $body:expr)?) => {
		$v struct $name {
			ptr: $rel
		}

		impl Drop for $name {
			fn drop(&mut self) {
				$({
					let $ptr = self.ptr;
					{ $body };
				})?;
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
		if ptr.is_null() {
			None
		} else {
			Some($i { ptr })
		}
	}};
}

gpointer!(pub Database => *mut LibmsiDatabase);
gpointer!(pub Query => *mut LibmsiQuery, ptr => unsafe {
	libmsi_query_close(ptr, null_mut());
});
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

	#[must_use]
	pub fn query_iter<S: AsRef<str>>(&self, query: S) -> Option<FetchIter> {
		let query = Query::new(&self, query)?;

		Some(FetchIter(query))
	}

	pub fn is_readonly(&self) -> bool {
		unsafe { libmsi_database_is_readonly(self.ptr) }
	}
}

impl Query {
	fn new<S: AsRef<str>>(database: &Database, query: S) -> Option<Query> {
		let query = CString::new(query.as_ref().as_bytes()).ok()?;

		ptr_check!(Query unsafe {
			libmsi_query_new(database.ptr, query.as_ptr(), null_mut())
		})
	}

	fn fetch(&self) -> Option<Record> {
		ptr_check!(Record unsafe {
			libmsi_query_fetch(self.ptr, null_mut())
		})
	}

	fn column_info(&self, col: LibmsiColInfo) -> Option<Record> {
		ptr_check!(Record unsafe {
			libmsi_query_get_column_info(self.ptr, col, null_mut())
		})
	}
}

pub struct FetchIter(Query);

impl Iterator for FetchIter {
	type Item = Record;

	fn next(&mut self) -> Option<Record> {
		self.0.fetch()
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

	#[must_use]
	pub fn string(&self, field: u32) -> OsString {
		let ptr = unsafe { libmsi_record_get_string(self.ptr, field) };
		let string = unsafe { CStr::from_ptr(ptr) };

		let string = OsStr::from_bytes(string.to_bytes()).to_os_string();
		unsafe { g_free(ptr as gpointer) };
		string
	}
}

impl fmt::Debug for Record {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let count = self.field_count();

		let mut list = f.debug_list();

		for i in 1..=count {
			list.entry(&self.string(i));
		}

		list.finish()
	}
}
