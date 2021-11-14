use msi::{Column, Expr, Package, Select, Value};
use std::collections::{HashMap, VecDeque};
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hash;
use std::io::{self, Error, ErrorKind, Read, Seek};

pub use msi::{open, StreamReader};

#[inline]
fn col<T: Into<String>>(item: T) -> Expr {
	Expr::col(item)
}

#[inline]
fn select<T: Into<String>>(item: T) -> Select {
	Select::table(item)
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

fn dir_resolve<
	I: Iterator<Item = (K, Option<K>, V)>,
	K: Hash + Eq + Clone + Debug,
	V: Clone + Debug,
>(
	entries: &mut I,
) -> HashMap<K, Vec<V>> {
	// let mut roots: VecDeque<(K, Vec<V>)> = VecDeque::new();
	let mut res: HashMap<K, Vec<V>> = HashMap::new();
	let mut tree: HashMap<K, Vec<(K, V)>> = HashMap::new();

	for (key, parent, name) in entries {
		if let Some(parent) = parent {
			let item = (key, name);

			if let Some(children) = tree.get_mut(&parent) {
				children.push(item);
			} else {
				tree.insert(parent, vec![item]);
			}
		} else {
			res.insert(key.clone(), vec![name]);

			if tree.get(&key).is_none() {
				tree.insert(key, vec![]);
			}
		}
	}

	let mut roots: VecDeque<K> = res.keys().map(|v| v.clone()).collect();

	while let Some(key) = roots.pop_front() {
		if let Some(path) = res.get(&key) {
			if let Some(children) = tree.get(&key) {
				// Ownership problem
				let mut temp = vec![];

				for (key, name) in children {
					let path = {
						let mut path = path.clone();
						path.push(name.clone());
						path
					};

					roots.push_back(key.clone());
					temp.push((key, path));
				}

				for (key, path) in temp {
					res.insert(key.clone(), path);
				}
			}
		}
	}

	res
}

pub fn entries<T: Read + Seek>(
	file: &mut Package<T>,
) -> io::Result<HashMap<String, Vec<String>>> {
	let directories = dir_resolve(&mut select!(
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
	));

	let compfiles = file
		.select_rows(
			select("File")
				.inner_join(
					select("Component"),
					col("Component.Component").eq(col("File.Component_")),
				)
				.columns(&[
					"File.File",
					"File.FileName",
					"Component.Directory_",
				]),
		)?
		.filter_map(|row| {
			let file = select!(@lamb &row[0], Str);
			let name = select!(@lamb &row[1], Str);
			let dir = select!(@lamb &row[2], Str);

			let mut path = directories.get(&dir)?.clone();
			path.push(long_name(&name).to_owned());

			Some((file, path))
		})
		.collect::<HashMap<_, _>>();

	Ok(compfiles)
}

pub fn get_cab<T: Read + Seek>(
	file: &mut Package<T>,
) -> io::Result<msi::StreamReader<T>> {
	let media = select!(
		file,
		Media(|Str(Cabinet)| Cabinet.strip_prefix('#').map(|v| v.to_owned()))
	)
	.nth(0);

	if let Some(id) = media {
		file.read_stream(&id)
	} else {
		Err(Error::from(ErrorKind::NotFound))
	}
}
