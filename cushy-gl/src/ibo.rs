

use crate::*;
use crate::api as gl;


///////////////////////////////////////////////////////////////////////////////////////////////////// IndexBuffer

#[derive(Debug)]
pub struct IndexBuffer {
	id: u32,
	usage: BufferUsage,
	data: Vec<u32>,
}


impl IndexBuffer {

	pub fn new(usage: BufferUsage) -> IndexBuffer {
		// Create an IBO
		let mut id: u32 = 0;
		unsafe {
			gl::GenBuffers(1, &mut id);
		}

		let ib = IndexBuffer {
			id,
			usage,
			data: Vec::new(),
		};
		
		ib
	}

	pub fn id(&self) -> u32 {
		self.id
	}

	pub fn data(&self) -> &Vec<u32> {
		&self.data
	}

	pub fn data_mut(&mut self) -> &mut Vec<u32> {
		&mut self.data
	}

	pub fn bind(&self) {
		unsafe {
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
		}
	}

	pub fn unbind(&self) {
		unsafe {
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
		}
	}

	pub fn set_data(&mut self, mut data: Vec<u32>) {
		// Swap out the data buffer
		std::mem::swap(&mut self.data, &mut data);
	}

	pub fn add_data(&mut self, data: &[u32]) {
		// Add data at the end of the buffer
		self.data.extend_from_slice(data);
	}

	pub fn upload(&mut self) {
		// Upload the data to the IBO
		// Prepare the size and pointers
		let size = (self.data.len() * std::mem::size_of::<u32>()) as isize;
		let ptr = self.data.as_ptr() as *const gl::types::GLvoid;

		self.bind();

		unsafe {
			gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, ptr, self.usage.gl_usage());
		}

		self.unbind();
	}

	pub fn draw_nobind(&self, prim: PrimitiveType, first: u32, count: u32) {
		// Draw
		let ptr = (first as usize*std::mem::size_of::<u32>()) as *const gl::types::GLvoid;

		unsafe {
			gl::DrawElements(prim.gl_type(), count as i32, gl::UNSIGNED_INT, ptr);
		}
	}

	pub fn draw(&self, prim: PrimitiveType, va: &VertexArray, first: u32, count: u32) {
		va.bind();
		self.bind();

		self.draw_nobind(prim, first, count);

		self.unbind();
		va.unbind();
	}
}


impl Drop for IndexBuffer {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &self.id);
		}
	}
}

