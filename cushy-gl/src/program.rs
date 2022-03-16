

use crate::*;
use crate::api as gl;


///////////////////////////////////////////////////////////////////////////////////////////////////// UniformValue


#[derive(Copy, Clone)]
pub enum UniformValue<'a> {
	Float (f32),
	Bool (bool),
	Vec2 (f32, f32),
	Vec3 (f32, f32, f32),
	Vec4 (f32, f32, f32, f32),
	Mat4 ([f32; 16]),
	Mat4ref (&'a [f32; 16]),
}

impl UniformValue<'_> {
}


///////////////////////////////////////////////////////////////////////////////////////////////////// Program

#[derive(Debug)]
pub struct Program {
	id: u32,
}

impl Program {

	fn get_int(&self, pname: u32) -> i32 {
		// Get an integer parameter
		let mut res: i32 = 0;
		unsafe {
			gl::GetProgramiv(self.id, pname, &mut res);
		}

		res
	}

	fn get_info_log(&self) -> String {
		// Extract the info log
		// Get the length and allocate a buffer
		let len = self.get_int(gl::INFO_LOG_LENGTH);
		let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
		buf.resize(len as usize, 0);

		// Get the log
		unsafe {
			gl::GetProgramInfoLog(self.id, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut i8);
		}

		// Convert to a string
		String::from_utf8_lossy(&buf).into_owned()
	}

	pub fn from_sources(vtx: impl AsRef<str>, frag: impl AsRef<str>) -> Result<Program, String> {
		// Create the shaders
		let vtx = VertexShader::from_source(vtx)?;
		let frag = FragmentShader::from_source(frag)?;

		// Create the program
		Program::from_shaders(&vtx, &frag)
	}

	pub fn from_shaders(vtx: &VertexShader, frag: &FragmentShader) -> Result<Program, String> {
		// Create the program
		let id = unsafe { gl::CreateProgram() };
		let prg = Program { id };

		// Attach the shaders, link the program, and detach them
		unsafe {
			gl::AttachShader(id, vtx.id());
			gl::AttachShader(id, frag.id());

			gl::LinkProgram(id);

			gl::DetachShader(id, vtx.id());
			gl::DetachShader(id, frag.id());
		}

		// Check success
		if prg.get_int(gl::LINK_STATUS)==0 {
			// Failure
			return Err(prg.get_info_log());
		}

		Ok(prg)
	}

	pub fn id(&self) -> u32 {
		self.id
	}

	pub fn bind(&self) {
		// Use the program
		unsafe {
			gl::UseProgram(self.id);
		}
	}

	pub fn find_uniform(&self, name: impl AsRef<str>) -> Option<u32> {
		// Find a uniform with the given name
		let name = std::ffi::CString::new(name.as_ref()).unwrap();
		let index = unsafe { gl::GetUniformLocation(self.id, name.as_ptr() as *const i8) };

		if index>=0 {
			Some(index as u32)
		}
		else {
			None
		}
	}

	pub fn set_uniform(&self, index: u32, val: &UniformValue) {
		// Set a uniform
		let i = index as i32;

		match *val {
			UniformValue::Float (f)			=> unsafe { gl::Uniform1f(i, f); },
			UniformValue::Bool (true)		=> unsafe { gl::Uniform1i(i, 1); },
			UniformValue::Bool (false)		=> unsafe { gl::Uniform1i(i, 0); },
			UniformValue::Vec2 (a, b)		=> unsafe { gl::Uniform2f(i, a, b); },
			UniformValue::Vec3 (a, b, c)	=> unsafe { gl::Uniform3f(i, a, b, c); },
			UniformValue::Vec4 (a, b, c, d) => unsafe { gl::Uniform4f(i, a, b, c, d); },
			UniformValue::Mat4 (mat)		=> unsafe { gl::UniformMatrix4fv(i, 1, 0, mat.as_ptr()); },
			UniformValue::Mat4ref (mat)		=> unsafe { gl::UniformMatrix4fv(i, 1, 0, mat.as_ptr()); },
		}
	}
}

impl Drop for Program {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteProgram(self.id);
		}
	}
}


