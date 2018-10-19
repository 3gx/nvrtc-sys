extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
	let cuda_path = PathBuf::from(match env::var("CUDA_HOME") {
		Ok(chome) => chome,
		Err(_) => "/usr/local/cuda".to_string()
	});
	let cuda = match cuda_path.to_str() {
		Some(c) => c,
		None => "nvrtc-sys: error creating string from cuda path",
	};
	for libdir in vec!["lib64", "lib"] {
		let mut clib_path = cuda_path.clone();
		clib_path.push(libdir);
		// Don't check if the path exists first.  If someone is having issues and
		// turns verbosity on, this will at least clue them in how to hack it.
		println!("cargo:rustc-link-search=native={}/{}", cuda, libdir);
	}
	println!("cargo:rustc-link-lib=nvrtc"); // link against nvrtc.

	// The bindgen::Builder is the main entry point
	// to bindgen, and lets you build up options for
	// the resulting bindings.
	let bindings = bindgen::Builder::default()
			// Tell clang where to find cuda.h.
			.clang_arg(format!("-I{}/include", cuda))
			.header("nvrtc-sys.h")
			.whitelist_recursively(true) // FIXME set to false
			.whitelisted_type("nvrtcResult")
			.whitelisted_type("nvrtcProgram")
			.whitelisted_function("nvrtcCompileProgram")
			.whitelisted_function("nvrtcCreateProgram")
			.whitelisted_function("nvrtcDestroyProgram")
			.whitelisted_function("nvrtcGetErrorString")
			.whitelisted_function("nvrtcGetProgramLogSize")
			.whitelisted_function("nvrtcGetProgramLog")
			.whitelisted_function("nvrtcGetPTX")
			.whitelisted_function("nvrtcGetPTXSize")
			.whitelisted_function("nvrtcVersion")
			.generate()
			.expect("Unable to generate nvrtc-sys bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
	bindings
			.write_to_file(out_path.join("nvrtc.rs"))
			.expect("Couldn't write nvrtc-sys bindings!");
}
