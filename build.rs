use bindgen::{Builder, CargoCallbacks};
use std::{env::var_os, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
	println!("cargo:rustc-link-lib=dylib=msi");
	println!("cargo:rerun-if-changed=./src/wrapper.h");

	let bindings = Builder::default()
		.header("./src/wrapper.h")
		.clang_arg("--include-directory=/usr/lib/glib-2.0/include/")
		.clang_arg("--include-directory=/usr/include/glib-2.0/")
		.clang_arg("--include-directory=/usr/include/libmsi-1.0/")
		.allowlist_function(".*[lL][iI][bB][mM][sS][iI].*")
		.allowlist_type(".*[lL][iI][bB][mM][sS][iI].*")
		.allowlist_var(".*[lL][iI][bB][mM][sS][iI].*")
		.parse_callbacks(Box::new(CargoCallbacks))
		.generate()
		.expect("Unable to generate bindings");

	let path = PathBuf::from(var_os("OUT_DIR").unwrap());
	bindings.write_to_file(path.join("bindings.rs"))?;
	// bindings.write_to_file("./src/lib.rs")?;

	Ok(())
}
