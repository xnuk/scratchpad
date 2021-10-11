#![cfg(unix)]

use libmsi::{DBFlags, Database, Query};
use std::env::args_os;
use std::io::{self, ErrorKind};

fn main() -> io::Result<()> {
	let path = args_os()
		.nth(1)
		.ok_or::<io::Error>(ErrorKind::InvalidInput.into())?;

	let database = Database::new(path, DBFlags::READONLY).unwrap();
	let mut query = Query::new(&database, "SELECT * FROM `Directory`").unwrap();

	let foo = {
		while let Some(record) = query.fetch() {
			println!("{:?}", record.string(1));
		}
	};

	Ok(())
}
