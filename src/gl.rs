
use std::os::raw;

use crate::Color;


mod api {
	#![allow(clippy::all)]
	include!("gl_46_core.rs");
}


pub fn init<F>(loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
	api::load_with(loadfn);
}

pub fn clear(col: Color) {
	// Clear the back buffer
	let (r, g, b, a) = col.as_f32();
	
	unsafe {
		api::ClearColor(r, g, b, a);
		api::Clear(api::COLOR_BUFFER_BIT);
	}
}

