pub mod nvrtc;

#[cfg(test)]
mod tests {
	use std::os::raw;
	use std::ffi::CString;
	use nvrtc::*;

	fn vecadd_src() -> CString {
		return CString::new("extern \"C\" __global__ void\n".to_string() +
			"addfv(const float* a, const float* b, float* out, size_t n) {\n" +
			"	size_t i = blockDim.x*blockIdx.x + threadIdx.x;\n" +
			"	if(i >= n) {\n" +
			"		return;\n" +
			"	}\n" +
			"	out[i] = a[i] + b[i];\n" +
			"}\n"
		).unwrap();
	}

	// A macro that expects an nvrtc call to succeed.
	macro_rules! rtc {
		($call:expr) => ({
			let res: nvrtcResult = $call;
			assert_eq!(res, nvrtcResult::NVRTC_SUCCESS);
		})
	}

	macro_rules! compile {
		($prg:expr, $call:expr) => ({
			let res: nvrtcResult = $call;
			if res != nvrtcResult::NVRTC_SUCCESS {
				let mut size : usize = 0;
				let sizeptr: *mut usize = &mut size as *mut usize;
				let retsz: nvrtcResult = nvrtcGetProgramLogSize($prg, sizeptr);
				if retsz != nvrtcResult::NVRTC_SUCCESS {
					eprintln!("after error {:?}, error {:?} getting program log size",
					          res, retsz);
					return;
				}
				// GetProgramLog will write into a string of length 'size'.  So we need
				// to create a block of memory of length 'size':
				let mut log: Vec<raw::c_char> = vec![];
				log.resize(size+1, 0);
				let retlog: nvrtcResult = nvrtcGetProgramLog($prg, log.as_mut_ptr());
				if retlog != nvrtcResult::NVRTC_SUCCESS {
					eprintln!("after error {:?}, error {:?} getting {}-byte program log",
					          res, retlog, size);
				}
				// There's probably some cleaner (mem::transmute?) way to convert a
				// sequence of raw bytes into a proper rust string so that we can print
				// it.  Contributions welcome.
				let asu8: Vec<u8> = log.into_iter().map(|v| v as u8).collect();
				let ruststr: String = String::from_utf8_unchecked(asu8);
				println!("nvrtc: {}", ruststr);
			}
		})
	}

	#[test]
	fn create_destroy() {
		let name = CString::new("vecadd").unwrap();
		let source = vecadd_src();
		let num_hdr: ::std::os::raw::c_int = 0;
		let mut headers: Vec<*const raw::c_char> = vec![];
		let mut incl: Vec<*const raw::c_char> = vec![];

		unsafe {
		let mut prg: nvrtcProgram = ::std::mem::uninitialized();
		// headers and incl need to be mutably borrowed, even though they would
		// never modify their arguments.  It's not bindgen's fault; nvrtc's header
		// declares them in a way that they look mutable.
		rtc!(nvrtcCreateProgram(&mut prg, source.as_ptr(), name.as_ptr(), num_hdr,
		                        headers.as_mut_ptr(), incl.as_mut_ptr()));
		rtc!(nvrtcDestroyProgram(&mut prg));
		}
	}

	// Compilation without any headers.
	#[test]
	fn compile_no_hdr() {
		let name = CString::new("vecadd").unwrap();
		let source = vecadd_src();
		let num_hdr: raw::c_int = 0;
		let mut headers: Vec<*const raw::c_char> = vec![];
		let mut incl: Vec<*const raw::c_char> = vec![];
		let mut options: Vec<*const raw::c_char> = vec![];

		unsafe {
		let mut prg: nvrtcProgram = ::std::mem::uninitialized();
		rtc!(nvrtcCreateProgram(&mut prg, source.as_ptr(), name.as_ptr(), num_hdr,
		                        headers.as_mut_ptr(), incl.as_mut_ptr()));
		compile!(prg, nvrtcCompileProgram(prg, 0 as raw::c_int,
		                                  options.as_mut_ptr()));
		rtc!(nvrtcDestroyProgram(&mut prg));
		}
	}
}
