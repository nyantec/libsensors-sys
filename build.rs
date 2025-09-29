use std::env;
use std::path::PathBuf;

fn main() {
	let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());

	for module in ["sensors", "error"] {
		let mut path = out.join(module);
		path.set_extension("rs");

		bindgen::builder()
			.header_contents("include.h", &format!("#include <sensors/{module}.h>"))
			.allowlist_file(format!(r".*/sensors/{module}\.h"))
			.default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
			.generate_cstr(true)
			.generate()
			.unwrap_or_else(|err| panic!("Unable to generate bindings for <sensors/{module}.h>: {err}"))
			.write_to_file(&path)
			.unwrap_or_else(|err| panic!("Failed to write bindings to {path:?}: {err}"));
	}
}
