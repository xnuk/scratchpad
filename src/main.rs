use cab::Cabinet;
use msi::{open, Column, Expr, Package, Select, Value};
use std::borrow::Cow;
use std::env::args_os;
use std::fmt::{self, Debug, Display, Formatter};
use std::io::{self, Read, Seek};

trait ToExpr {
	fn from(&self) -> Expr;
}

impl ToExpr for bool {
	#[inline]
	fn from(&self) -> Expr {
		Expr::boolean(*self)
	}
}

impl ToExpr for i32 {
	#[inline]
	fn from(&self) -> Expr {
		Expr::integer(*self)
	}
}

impl ToExpr for str {
	#[inline]
	fn from(&self) -> Expr {
		Expr::string(self)
	}
}

impl ToExpr for &str {
	#[inline]
	fn from(&self) -> Expr {
		Expr::string(*self)
	}
}

impl ToExpr for () {
	#[inline]
	fn from(&self) -> Expr {
		Expr::null()
	}
}

impl ToExpr for String {
	#[inline]
	fn from(&self) -> Expr {
		Expr::string(self)
	}
}

#[inline]
fn expr<T: ToExpr>(item: T) -> Expr {
	ToExpr::from(&item)
}

#[inline]
fn col<T: Into<String>>(item: T) -> Expr {
	Expr::col(item)
}

#[inline]
fn null() -> Expr {
	Expr::null()
}

macro_rules! expr {
	($a:ident $(. $ap:ident)?) => {
		col(
			concat!(
				stringify!($a)
				$(, ".", stringify!($ap))?
			)
		)
	};
	($a:literal) => {expr($a)};
	($a:ident $(. $ap:ident)? $s:ident $($b:tt)+) => {
		expr!($a $(. $ap)?).$s(expr!($($b)+))
	};
	($a:ident $(. $ap:ident)? == $($b:tt)+) => {
		expr!($a $(. $ap)? eq $($b)+)
	};
}

macro_rules! select {
	(
		@assign $row:ident, $v:expr =>
			$first_cont:ident ($first_key:ident),
			$( $cont:ident ($key:ident) ),* $(,)?
	) => {
		#[allow(non_snake_case)]
		let $first_key = select!(@lamb &$row[$v], $first_cont);
		select!(@assign $row, $v + 1 => $($cont($key),)*);
	};

	(@assign $row:ident, $v:expr => $(,)?) => {};

	(@lamb $col:expr, $cont:ident) => {
		select!(@ques $cont, match $col {
			select!(@val x $cont) => Some(select!(@lamb $cont(x))),
			_ => None,
		})
	};
	(@lamb Str($x:ident)) => { $x.to_owned() };
	(@lamb Int($x:ident)) => { $x };
	(@lamb MayStr($x:ident)) => { $x.to_owned() };
	(@lamb MayInt($x:ident)) => { $x };

	(@ques Str, $t:expr) => {$t?};
	(@ques Int, $t:expr) => {$t?};
	(@ques MayStr, $t:expr) => {$t};
	(@ques MayInt, $t:expr) => {$t};

	(@val $x:ident Str) => {Value::Str($x)};
	(@val $x:ident Int) => {Value::Int($x)};
	(@val $x:ident MayStr) => {Value::Str($x)};
	(@val $x:ident MayInt) => {Value::Int($x)};

	(
		$db:ident, $table:ident (
			| $( $cont:ident($a:ident) ),* $(,)? |
			$body:expr
		)
		$(,)?
	) => {{
		$db.select_rows(
			Select::table(stringify!($table)).columns(&[$(stringify!($a)),*])
		)?.filter_map(move |row| {
			select!(@assign row, 0 => $($cont($a),)*);
			$body
		})
	}}
}

struct Show<T>(T);

impl Debug for Show<&[Column]> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let mut list = f.debug_list();

		for item in self.0.iter() {
			let coltype: Box<dyn Display> = match item.category() {
				Some(category) => Box::new(category),
				None => Box::new(item.coltype()),
			};

			let nullable = item.is_nullable();
			let localizable = item.is_localizable();
			let primary = item.is_primary_key();

			let name = item.name();

			list.entry(&format_args!(
				"{name:?} /* {primary}{localizable}{coltype}{nullable} */",
				name = name,
				coltype = coltype,
				nullable = if nullable { "?" } else { "" },
				primary = if primary { "#" } else { "" },
				localizable = if localizable { "%" } else { "" }
			));
		}

		list.finish()
	}
}

#[inline]
fn long_name<'a>(text: &'a str) -> &'a str {
	match text.split_once('|') {
		Some((_, x)) => x,
		None => text,
	}
}

fn main() -> io::Result<()> {
	let path = args_os().nth(1).unwrap();
	let mut file = open(path)?;

	// for table in file.tables() {
	// 	let columns = table
	// 		.columns()
	// 		.iter()
	// 		// .filter(|x| {
	// 		// 	x.category().is_none()
	// 		// 		== match x.coltype() {
	// 		// 			ColumnType::Str(_) => true,
	// 		// 			_ => false,
	// 		// 		}
	// 		// })
	// 		.map(|x| (x.coltype(), x.category(), x.name()))
	// 		.collect::<Vec<_>>();

	// 	println!("{}: {:?}", table.name(), columns);
	// }

	// if let Some(table) = file.get_table("Component") {
	// 	println!("{:?}", Show(table.columns()))
	// }

	// for stream in file.streams() {
	// 	println!("{}", stream);
	// }

	// .columns(&[
	// 	"Directory.Directory",        /* #Identifier */
	// 	"Directory.Directory_Parent", /* Identifier? */
	// 	"Directory.DefaultDir",       /* %DefaultDir */
	// ]);

	// const COLUMN_FILE: &[&str] = &[
	// 	"File",       /* #Identifier */
	// 	"Component_", /* Identifier */
	// 	"FileName",   /* %Filename */
	// 	"FileSize",   /* Int32 */
	// 	"Version",    /* Version? */
	// 	"Language",   /* Language? */
	// 	"Attributes", /* Int16? */
	// 	"Sequence",   /* Int16 */
	// ];

	// "Component.Component",   /* #Identifier */
	// "Component.ComponentId", /* Guid? */
	// "Component.Directory_",  /* Identifier */
	// "Component.Attributes",  /* Int16 */
	// "Component.Condition",   /* Condition? */
	// "Component.KeyPath",     /* Identifier? */
	let directories = select!(
		file,
		Directory(
			|Str(Directory), MayStr(Directory_Parent), Str(DefaultDir)| {
				Some((
					Directory,
					Directory_Parent,
					long_name(&DefaultDir).to_owned(),
				))
			}
		)
	)
	.collect::<Vec<_>>();

	let components = select!(
		file,
		Component(|Str(Component), MayStr(ComponentId), Str(Directory_)| {
			Some((Component, ComponentId, Directory_))
		})
	)
	.collect::<Vec<_>>();

	let files = select!(
		file,
		File(|Str(File), Str(Component_), Str(FileName)| {
			Some((File, Component_, long_name(&FileName).to_owned()))
		})
	)
	.collect::<Vec<_>>();

	let media = select!(
		file,
		Media(|Str(Cabinet)| Cabinet.strip_prefix('#').map(|v| v.to_owned()))
	)
	.collect::<Vec<_>>();

	for id in media {
		let mut cabinet = Cabinet::new(file.read_stream(&id)?)?;


		let folder = &cabinet
			.folder_entries()
			.flat_map(|x| {
				println!("{:?}", x.compression_type());
				x.file_entries()
			})
			.map(|v| (v.name().to_owned(), v.uncompressed_size()))
			.collect::<Vec<_>>();

		for entry @ (name, size) in folder {
			println!("{:?}", entry);
			let mut buf = Vec::with_capacity(*size as usize);
			cabinet.read_file(name)?.read_to_end(&mut buf)?;
			println!("{:?}", buf.len());
		}
	}

	// let col = [
	// 	"DiskId",       /* #SMALLINT */
	// 	"LastSequence", /* SMALLINT */
	// 	"DiskPrompt",   /* %Text? */
	// 	"Cabinet",      /* Cabinet? */
	// 	"VolumeLabel",  /* Text? */
	// 	"Source",       /* Property? */
	// ];

	// let components = select! { Component:
	// 	Component ComponentId Directory_ Attributes Condition KeyPath |
	// 	query => {
	// 		file.select_rows(query)?.map(|row| {
	// 			row
	// 		})
	// 	}
	// };
	// let query = select("File")
	// 	.inner_join(
	// 		select("Component"),
	// 		expr!(Component.Component == File.Component_)
	// 	)
	// 	.inner_join(
	// 		select("Directory"),
	// 		expr!(Directory.Directory == Component.Directory_),
	// 	)
	// .columns(&[
	// 	"File.File",
	// 	//"Directory.DefaultDir",
	// 	"File.FileName",
	// 	"File.Component_",
	// 	"Component.Component",
	// ]);

	// println!("{}", query);

	// for row in file.select_rows(query)? {
	// 	println!("{:?}", Show(row.columns()));
	// 	let values = (0..row.len()).map(|i| &row[i]).collect::<Vec<_>>();
	// 	println!("{:?}", values);
	// }

	// for table in file.tables() {
	// 	println!("{} {:#?}", table.name(), Show(table.columns()));
	// }

	/*
	let streams = file
		.read_storage("/")?
		.filter_map(|storage| {
			if storage.is_stream() {
				Some(storage.path().as_os_str().to_owned())
			} else {
				None
			}
		})
		.collect::<Vec<_>>();

	for path in streams {
		let file = &mut file;
		let mut stream = file.open_stream(&path)?;
		let capacity = stream.len();
		println!("{:?} {}", path, capacity);

		// if path.to_string_lossy().contains("SummaryInformation") {
		let mut line = Vec::with_capacity(capacity as usize);
		stream.read_to_end(&mut line)?;
		line.shrink_to_fit();
		println!("{:?}", line);
		// }
	}
	*/

	Ok(())
}
