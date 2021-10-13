#![cfg(unix)]

use libmsi::{DBFlags, Database};
use std::env::args_os;
use std::io::{self, ErrorKind};

fn main() -> io::Result<()> {
	let (path, count) = {
		let mut args = args_os().skip(1);

		let path = args
			.next()
			.ok_or::<io::Error>(ErrorKind::InvalidInput.into())?;

		let count = args
			.next()
			.and_then(|x| x.to_string_lossy().parse().ok())
			.unwrap_or(1u32);

		(path, count)
	};

	// loop for memory leak detect
	for _ in 0..count {
		let database = Database::new(&path, DBFlags::READONLY).unwrap();

		{
			let query = database
				.query_iter("select `FileName`, `Component_` from `File`")
				.unwrap();

			let mut i = 0u32;
			for _record in query {
				// println!("{:?}", record);
				i += 1;
			}

			println!("{:?}", i);
		}
	}

	Ok(())
}
