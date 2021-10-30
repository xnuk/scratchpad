use msi::{open, Column, Expr, Package, Select, Value};
use std::env::args_os;
use std::fmt::{self, Debug, Formatter};
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

#[inline]
fn select<T: Into<String>>(item: T) -> Select {
	Select::table(item)
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
	(@assign: $v:expr =>) => {};
	(@assign: $v:expr => $first:ident $($rest:ident)*) => {
		#[allow(non_snake_case)]
		let $first = $v;
		select!(@assign: $v + 1 => $($rest)*);
	};
	($table:ident $(,)? $(:)? $(;)? $($a:ident $(,)? $(;)?)*) => {{
		let columns = [$(stringify!($a)),*];
		Select::table(stringify!($table)).columns(&columns)
	}};
	($table:ident $(,)? $(:)? $(;)? $($a:ident $(,)? $(;)?)* | $query:ident => $body:expr) => {{
		let columns = [$(stringify!($a)),*];
		select!(@assign: 0 => $($a)*);

		let $query = Select::table(stringify!($table)).columns(&columns);
		$body
	}}
}

struct Show<T>(T);

impl Debug for Show<&[Column]> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let mut list = f.debug_list();

		for item in self.0.iter() {
			let coltype = match item.category() {
				Some(category) => format!("{:?}", category),
				None => format!("{:?}", item.coltype()),
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
	let directories = select! {
		Directory: Directory Directory_Parent DefaultDir | query => {
			file.select_rows(query)?.filter_map(move |row| {
				if let (
					Value::Str(dir),
					Value::Str(parent),
					Value::Str(name),
				) = (
					&row[Directory],
					&row[Directory_Parent],
					&row[DefaultDir]
				) {
					Some((dir.clone(), parent.clone(), name.clone()))
				} else {
					None
				}
			})
		}
	}
	.collect::<Vec<_>>();

	println!("{:?}", directories);

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
