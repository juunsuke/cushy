

use crate::api as gl;


///////////////////////////////////////////////////////////////////////////////////////////////////// TexFilter

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TexFilter {
	Nearest,
	Linear
}

impl TexFilter {
	pub fn gl_filter(&self) -> u32 {
		match *self {
			TexFilter::Nearest => gl::NEAREST,
			TexFilter::Linear => gl::LINEAR,
		}
	}
}


///////////////////////////////////////////////////////////////////////////////////////////////////// TexFilters

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TexFilters {
	NearestNearest,
	NearestLinear,
	LinearLinear,
	LinearNearest,
}

impl Default for TexFilters {
	fn default() -> Self {
		Self::NearestNearest
	}
}

impl TexFilters {
	pub fn min(&self) -> TexFilter {
		use TexFilters::*;
		match *self {
			NearestNearest | NearestLinear => TexFilter::Nearest,
			LinearLinear | LinearNearest => TexFilter::Linear,
		}
	}

	pub fn mag(&self) -> TexFilter {
		use TexFilters::*;
		match *self {
			NearestNearest | LinearNearest => TexFilter::Nearest,
			LinearLinear | NearestLinear => TexFilter::Linear,
		}
	}

	pub fn gl_filters(&self) -> (u32, u32) {
		use TexFilters::*;
		match *self {
			NearestNearest => (gl::NEAREST, gl::NEAREST),
			NearestLinear => (gl::NEAREST, gl::LINEAR),
			LinearLinear => (gl::LINEAR, gl::LINEAR),
			LinearNearest => (gl::LINEAR, gl::NEAREST),
		}
	}
}


///////////////////////////////////////////////////////////////////////////////////////////////////// GlTexture

#[derive(Debug)]
pub struct Texture2D {
	id: u32,
	filters: TexFilters,
}

impl Texture2D {

	pub fn new(filters: TexFilters) -> Self {
		// Create the texture
		let mut id = 0;
		unsafe {
			gl::GenTextures(1, &mut id);
		}

		Self {
			id,
			filters,
		}
	}

	pub fn id(&self) -> u32 {
		self.id
	}

	pub fn filters(&self) -> TexFilters {
		self.filters
	}

	pub fn min_filter(&self) -> TexFilter {
		self.filters.min()
	}

	pub fn mag_filter(&self) -> TexFilter {
		self.filters.mag()
	}

	pub fn bind(&self) {
		//println!("GlTexture Binding {}", self.id());
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, self.id());
		}
	}

	pub fn unbind(&self) {
		unsafe {
			gl::BindTexture(gl::TEXTURE_2D, 0);
		}
	}

	pub fn upload_raw(&self, w: u32, h: u32, data: &[u32]) {
		// Upload raw data
		assert!((w*h) as usize==data.len());

		self.bind();

		// Set the filters
		let (min, mag) = self.filters.gl_filters();

		unsafe {
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min as i32);
			gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag as i32);
		}

		// Create the store
		unsafe {
			gl::TexImage2D(
				gl::TEXTURE_2D,
				0,
				gl::RGBA as i32,
				w as i32,
				h as i32,
				0,
				gl::RGBA,
				gl::UNSIGNED_BYTE,
				data.as_ptr() as *const gl::types::GLvoid
			);
		}
	}
}

impl PartialEq for Texture2D {
	fn eq(&self, o: &Texture2D) -> bool {
		// Check if both OpenGL textures are the same
		self.id == o.id
	}
}

impl Drop for Texture2D {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteTextures(1, &self.id);
		}
	}
}




