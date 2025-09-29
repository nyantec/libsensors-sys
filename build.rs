use std::env;
use std::path::PathBuf;

fn main() {
	let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
	let src_dir = {
		let mut src_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
		src_dir.push("src");
		src_dir
	};

	for module in ["sensors", "error"] {
		let out = {
			let mut out = out_dir.join(module);
			out.set_extension("rs");
			out
		};

		match bindgen::builder()
			.header_contents("include.h", &format!("#include <sensors/{module}.h>"))
			.allowlist_file(format!(r".*/sensors/{module}\.h"))
			.default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
			.generate_cstr(true)
			.generate()
		{
			Ok(bindings) => bindings
				.write_to_file(&out)
				.unwrap_or_else(|err| panic!("Failed to write bindings to {out:?}: {err}")),

			Err(err) => {
				eprintln!("Unable to re‐generate bindings for <sensors/{module}.h>: {err}");

				// Fall back to pre‐generated bindings
				let src = {
					let mut src = src_dir.join(module);
					src.set_extension("rs");
					src
				};

				std::fs::copy(&src, &out).unwrap_or_else(|err| panic!("Failed to copy {src:?} to {out:?}: {err}"));
			}
		}
	}
}
