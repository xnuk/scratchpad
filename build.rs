use bindgen::{
	builder,
	callbacks::{EnumVariantValue, ParseCallbacks},
};

use syn::{
	parse_quote,
	visit_mut::{self, VisitMut},
	Ident, Item,
};

use std::{
	convert::AsRef,
	env::var_os,
	fs::File,
	io::{self, Write},
	iter::{FromIterator, IntoIterator},
	path::PathBuf,
};

use quote::ToTokens;

macro_rules! reserved {
	($i:ident) => {concat!(reserved!(@), stringify!($i))};
	(@) => {"___REMOVE_HERE___"};
}

const LIBMSI_REGEX: &'static str = "(?i).*libmsi.*";

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

struct RemoveThere;

impl VisitMut for RemoveThere {
	fn visit_ident_mut(&mut self, node: &mut Ident) {
		let name = format!("{}", node);

		if let Some(name) = name.strip_prefix(reserved!(@)) {
			*node = Ident::new(name, node.span());
		}

		visit_mut::visit_ident_mut(self, node);
	}
}

fn post2(source: &String) -> Option<String> {
	let mut file = syn::parse_file(source).ok()?;

	macro_rules! check {
		(($a:ident, $b:ident) as $c:path, $e:expr) => {{
			if let $c($a) = $a {
				if let $c($b) = $b {
					$e
				}
			}
		}};
	}

	macro_rules! remove {
		($i:expr) => {{
			file.items.remove($i);
			continue;
		}};
	}

	let mut i = 0;

	while i < file.items.len() {
		let (before, after) = file.items.split_at_mut(i);
		let item = &mut after[0];

		if let Some(last) = before.last_mut() {
			check! {
				(last, item) as Item::Impl,
				if
					last.trait_ == item.trait_ &&
					last.self_ty == item.self_ty
				{
					last.items.append(&mut item.items);
					remove!(i);
				}
			}

			check! {
				(last, item) as Item::ForeignMod,
				if last.abi == item.abi {
					last.items.append(&mut item.items);
					remove!(i);
				}
			}
		}

		if let Item::Type(item) = item {
			if format!("{}", item.ident).contains(reserved!(@)) {
				remove!(i);
			}
		}

		if let Item::ForeignMod(item) = item {
			item.attrs.push(parse_quote!(
				#[link(name = "msi")]
			));
		}
		i += 1;
	}

	RemoveThere.visit_file_mut(&mut file);

	Some(file.to_token_stream().to_string())
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
		.blocklist_function("libmsi_query_get_error")
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
		.rustfmt_bindings(false)
		.use_core()
		.parse_callbacks(Box::new(Callback))
		.generate()
		.ok()
		.map(|v| v.to_string())
}

fn main() -> io::Result<()> {
	println!("cargo:rerun-if-changed=build.rs");

	let source =
		post2(&compile().expect("Compile failed")).expect("Compile failed");

	let header = vec![
		"use ::std::os::raw::c_char",
		"use ::gio_sys::{GCancellable, GInputStream}",
		"use ::glib::ffi::{GArray, GError}",
	]
	.join(";\n")
		+ ";\n";

	let path = PathBuf::from(var_os("OUT_DIR").unwrap()).join("bindings.rs");
	let mut file = File::create(path)?;
	file.write(header.as_bytes())?;
	file.write_all(source.as_bytes())?;
	Ok(())
}
