use bindgen::{Builder, CargoCallbacks};
use std::{env::var_os, error::Error, path::PathBuf};

const LIBMSI_REGEX: &'static str = "(?i).*libmsi.*";

fn main() -> Result<(), Box<dyn Error>> {
	println!("cargo:rustc-link-lib=dylib=msi");
	println!("cargo:rerun-if-changed=./src/wrapper.h");

	let bindings = Builder::default()
		.header("./src/wrapper.h")
		.clang_arg("--include-directory=/usr/lib/glib-2.0/include/")
		.clang_arg("--include-directory=/usr/include/glib-2.0/")
		.clang_arg("--include-directory=/usr/include/libmsi-1.0/")
		.allowlist_function(LIBMSI_REGEX)
		.allowlist_type(LIBMSI_REGEX)
		.allowlist_var(LIBMSI_REGEX)
		.parse_callbacks(Box::new(CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	let path = PathBuf::from(var_os("OUT_DIR").unwrap());
	bindings.write_to_file(path.join("bindings.rs"))?;
	// bindings.write_to_file("./src/lib.rs")?;

	Ok(())
}
