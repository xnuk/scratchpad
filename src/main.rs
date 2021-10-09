#![cfg(unix)]

use libmsi::{
	libmsi_database_get_primary_keys, libmsi_database_new, libmsi_query_close,
	libmsi_query_execute, libmsi_query_fetch, libmsi_query_new,
	libmsi_record_get_string, LibmsiDbFlags_LIBMSI_DB_FLAGS_READONLY,
};
use std::env::args_os;
use std::ffi::{CStr, CString};
use std::io::{self, ErrorKind};
use std::os::raw::c_char;
use std::os::unix::ffi::OsStringExt;
use std::ptr::{null, null_mut};

fn main() -> io::Result<()> {
	let path = args_os()
		.nth(1)
		.ok_or::<io::Error>(ErrorKind::InvalidInput.into())?;

	let ccst = CString::new(path.into_vec())?;

	// let sexmsg = [0 as c_char; 100];

	// let mut foo = _GError {
	// 	domain: 0, code: 0, message: sexmsg.as_mut_ptr()
	// };

	let database = unsafe {
		libmsi_database_new(
			ccst.as_ptr(),
			LibmsiDbFlags_LIBMSI_DB_FLAGS_READONLY,
			null(),
			null_mut(),
		)
	};

	let foo = unsafe {
		let query = {
			let q = CString::new("SELECT * FROM `Directory`")?;
			libmsi_query_new(database, q.as_ptr(), null_mut())
		};

		let mut ptr = libmsi_query_fetch(query, null_mut());
		while ptr != null_mut() {
			println!("{:?}", CStr::from_ptr(libmsi_record_get_string(ptr, 1)));
			ptr = libmsi_query_fetch(query, null_mut());
		}
	};

	Ok(())
}
