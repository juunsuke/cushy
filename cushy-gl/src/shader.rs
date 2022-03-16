

use crate::api as gl;


////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct VertexShader (pub Shader);
pub struct FragmentShader (pub Shader);

impl VertexShader {
	pub fn from_source(src: impl AsRef<str>) -> Result<VertexShader, String> {
		Ok(VertexShader(Shader::from_source(gl::VERTEX_SHADER, src)?))
	}
}

impl FragmentShader {
	pub fn from_source(src: impl AsRef<str>) -> Result<FragmentShader, String> {
		Ok(FragmentShader(Shader::from_source(gl::FRAGMENT_SHADER, src)?))
	}
}

impl std::ops::Deref for VertexShader {
	type Target = Shader;
	fn deref(&self) -> &Shader {
		&self.0
	}
}

impl std::ops::Deref for FragmentShader {
	type Target = Shader;
	fn deref(&self) -> &Shader {
		&self.0
	}
}


//////////////////////////////////////////////////////////////////////////////////////////////////// Shader

pub struct Shader {
	id: u32,
}

impl Shader {

	fn get_int(&self, pname: u32) -> i32 {
		// Get an integer parameter
		let mut res: i32 = 0;
		unsafe {
			gl::GetShaderiv(self.id, pname, &mut res);
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
			gl::GetShaderInfoLog(self.id, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut i8);
		}

		// Convert to a string
		String::from_utf8_lossy(&buf).into_owned()
	}

	pub fn from_source(kind: u32, src: impl AsRef<str>) -> Result<Shader, String> {
		// Create a GL shader
		let id = unsafe { gl::CreateShader(kind)  };

		let shader = Shader {
			id
		};
		

		// Set the source and compile
		let src = src.as_ref();
		let sptr = src.as_ptr() as *const i8;
		let len = src.len() as i32;
		unsafe {
			gl::ShaderSource(id, 1, &sptr, &len);
			gl::CompileShader(id);
		}

		// Check success
		if shader.get_int(gl::COMPILE_STATUS)==0 {
			// Failure
			// Extract and return the error message
			return Err(shader.get_info_log());
		}

		Ok(shader)
	}

	pub fn id(&self) -> u32 {
		self.id
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteShader(self.id);
		}
	}
}


