use bindgen::{
	builder,
	callbacks::{EnumVariantValue, ParseCallbacks},
};

use std::{
	convert::AsRef,
	fs::OpenOptions,
	io::{self, ErrorKind, Write},
	iter::{FromIterator, IntoIterator},
};

use regex::Regex;

macro_rules! reserved {
	($i:ident) => {concat!(reserved!(@), stringify!($i))};
	(@) => {"___REMOVE_HERE___"};
}

const LIBMSI_REGEX: &'static str = "(?i).*libmsi.*";
const BINDING: &'static str = "./src/bindings.rs";

#[derive(Debug)]
struct Callback;

struct Lines(String);

impl<T: AsRef<str>> FromIterator<T> for Lines {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Lines {
		let mut iter = iter.into_iter();

		let mut string = match iter.next() {
			Some(v) => String::from(v.as_ref()),
			None => String::new(),
		};

		for item in iter {
			string.push('\n');
			string += item.as_ref();
		}

		Lines(string)
	}
}

impl ParseCallbacks for Callback {
	fn enum_variant_name(
		&self,
		enum_name: Option<&str>,
		original: &str,
		_: EnumVariantValue,
	) -> Option<String> {
		let name = original.strip_prefix(match enum_name? {
			"enum LibmsiProperty" => "LIBMSI_PROPERTY_",
			"enum LibmsiPropertyType" => "LIBMSI_PROPERTY_TYPE_",
			"enum LibmsiResultError" => "LIBMSI_RESULT_",
			"enum LibmsiDBError" => "LIBMSI_DB_ERROR_",
			"enum LibmsiDbFlags" => "LIBMSI_DB_FLAGS_",
			"enum LibmsiColInfo" => "LIBMSI_COL_INFO_",
			"GParamFlags" => "G_PARAM_",
			_ => None?,
		})?;

		Some(name.to_owned())
	}

	fn item_name(&self, original: &str) -> Option<String> {
		let name = match original {
			"gboolean" => reserved!(bool),

			"guint32" => reserved!(u32),
			"guint64" => reserved!(u64),
			"guint" => reserved!(u32),

			"gint32" => reserved!(i32),
			"gint64" => reserved!(i64),
			"gint" => reserved!(i64),
			"glong" => reserved!(i64),

			"gsize" => reserved!(u64),
			"gulong" => reserved!(u64),

			"gchar" => reserved!(c_char),
			_ => None?,
		};

		Some(name.to_owned())
	}
}

fn compile() -> Option<String> {
	builder()
		.header_contents("wrapper.h", "#include <libmsi.h>")
		.clang_args(&[
			"--include-directory=/usr/lib/glib-2.0/include/",
			"--include-directory=/usr/include/glib-2.0/",
			"--include-directory=/usr/include/libmsi-1.0/",
		])
		.allowlist_function(LIBMSI_REGEX)
		.allowlist_type(LIBMSI_REGEX)
		.allowlist_var(LIBMSI_REGEX)
		.blocklist_function("libmsi_.*_get_type")
		.blocklist_function("libmsi_.*_error_quark")
		.blocklist_type("_?Libmsi.*Class")
		.blocklist_type("_?GObject.*")
		.blocklist_type("_?GValue")
		.blocklist_type("_?GError")
		.blocklist_type("_?GQuark")
		.blocklist_type("_?GParamSpec")
		.blocklist_type("_?GArray")
		.blocklist_type("_?GInputStream(?:Private)?")
		.blocklist_type("_?GCancellable(?:Private)?")
		.blocklist_type("_?GParamFlags")
		.blocklist_type("_?GType.*")
		.blocklist_type("_?GValue.*")
		.blocklist_type("_?GData")
		.blocklist_type("_?GSList")
		.blocklist_item("LIBMSI_NULL_INT")
		.blocklist_item("gfloat|gdouble|gpointer")
		.newtype_enum("LibmsiProperty")
		.newtype_enum("LibmsiPropertyType")
		.newtype_enum("LibmsiResultError")
		.newtype_enum("LibmsiDBError")
		.newtype_enum("LibmsiColInfo")
		.bitfield_enum("LibmsiDbFlags")
		.layout_tests(false)
		.translate_enum_integer_types(true)
		.use_core()
		.parse_callbacks(Box::new(Callback))
		.generate()
		.ok()
		.map(|v| v.to_string())
}

fn post_compile() -> Option<String> {
	let Lines(source) = {
		let pubtype =
			Regex::new(concat!("^pub type ", reserved!(@), r"\w+ = [^{};]+;$"))
				.ok()?;

		compile()?
			.lines()
			.filter(|x| !pubtype.is_match(*x))
			.map(|x| x.replace(reserved!(@), ""))
			.chain(Some(reserved!(@).to_owned()).into_iter())
			.scan((None::<String>, false), |state, line| {
				let (start, closed) = state;

				if line == reserved!(@) {
					let mut ret = vec![String::new()];

					if *closed {
						ret.insert(0, '}'.into());
					}

					return Some(ret);
				}

				let trimmed = line.trim();
				if trimmed.is_empty() || line.starts_with("\t") {
					return Some(vec![line]);
				}

				if start.is_some() && trimmed == "}" {
					state.1 = true;
					return Some(vec![]);
				}

				if trimmed.ends_with("{") {
					println!("{:?} / {}", start, trimmed);
					if let Some(start) = start {
						if start == trimmed {
							state.1 = false;
							return Some(vec![]);
						}
					}

					state.0 = Some(line.clone());
				}

				let mut ret = vec![line];

				if *closed {
					ret.insert(0, '}'.into());
					state.1 = false;
				}

				Some(ret)
			})
			.flatten()
			.flat_map(|line| {
				if line.trim() == r#"extern "C" {"# {
					vec![r#"#[link(name = "msi")]"#.to_owned(), line]
				} else {
					vec![line]
				}
			})
			.collect()
	};
	Some(source)
}

fn main() -> io::Result<()> {
	println!("cargo:rerun-if-changed={}", BINDING);

	let file = OpenOptions::new()
		.write(true)
		.create_new(true)
		.open(BINDING);

	if let Err(err) = file {
		return if err.kind() == ErrorKind::AlreadyExists {
			Ok(())
		} else {
			Err(err)
		};
	}

	let mut file = file?;

	let source = post_compile().expect("Compile failed");

	let header = vec![
		"use ::std::os::raw::c_char",
		"use ::gio_sys::{GCancellable, GInputStream}",
		"use ::glib::ffi::{GArray, GError}",
	]
	.join(";\n")
		+ ";\n";

	file.write(header.as_bytes())?;
	file.write_all(source.as_bytes())?;
	Ok(())
}
