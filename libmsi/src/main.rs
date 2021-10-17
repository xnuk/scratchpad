#![cfg(unix)]

use libmsi::{DBFlags, Database};
use std::collections::HashMap;
use std::env::args_os;
use std::io::{self, ErrorKind};

#[derive(Debug)]
struct Directory<T> {
	id: T,
	parent: Option<T>,
	name: T,
}

fn main() -> io::Result<()> {
	let path = {
		let mut args = args_os().skip(1);

		let path = args
			.next()
			.ok_or::<io::Error>(ErrorKind::InvalidInput.into())?;

		path
	};

	let database = Database::new(&path, DBFlags::READONLY).unwrap();

	let directories = database.query_iter(
		"select `Directory`, `Directory_Parent`, `DefaultDir` from `Directory`",
	).map(|record| {
		let id = record.string(1);
		let parent = {
				let st = record.string(2);
				if st.is_empty() { None } else { Some(st) }
			};
		let directory = Directory {
			id: id.clone(),
			parent,
			name: record.string(3),
		};
		(id, directory)
	}).collect::<HashMap<_, _>>();

	println!("{:#?}", directories);

	// .query_iter("select `FileName`, `Component_` from `File`")
	// .query_iter(r#"select * from `_Columns` where .`Table` = 'File'"#)
	// .query_iter("select `Directory`, `Directory_Parent`, `DefaultDir` from `Directory`")
	// .query_iter("select `Component`, `ComponentId`, `Directory_`, `Attributes`, `KeyPath` from `Component`")

	Ok(())
}
