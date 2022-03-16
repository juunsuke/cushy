

use crate::*;
use crate::api as gl;


///////////////////////////////////////////////////////////////////////////////////////////////////// BufferUsage

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BufferUsage {
	Stream,
	Static,
	Dynamic,
}

impl BufferUsage {
	pub fn gl_usage(&self) -> u32 {
		match *self {
			BufferUsage::Stream => gl::STREAM_DRAW,
			BufferUsage::Static => gl::STATIC_DRAW,
			BufferUsage::Dynamic => gl::DYNAMIC_DRAW,
		}
	}
}


///////////////////////////////////////////////////////////////////////////////////////////////////// VertexBuffer

#[derive(Debug)]
pub struct VertexBuffer<T> {
	id: u32,
	usage: BufferUsage,
	capacity: usize,
	data: Vec<T>,
}

impl<T: Vertex> VertexBuffer<T> {
	
	pub fn new(usage: BufferUsage) -> VertexBuffer<T> {
		// Create a VBO
		let mut id: u32 = 0;
		unsafe {
			gl::GenBuffers(1, &mut id);
		}

		let vb = VertexBuffer {
			id,
			usage,
			capacity: 0,
			data: Vec::new(),
		};
		
		vb
	}

	pub fn id(&self) -> u32 {
		self.id
	}

	pub fn data(&self) -> &Vec<T> {
		&self.data
	}

	pub fn data_mut(&mut self) -> &mut Vec<T> {
		&mut self.data
	}

	pub fn swap_data(&mut self, data: &mut Vec<T>) {
		std::mem::swap(&mut self.data, data);
	}

	pub fn add_vertices(&mut self, v: &[T]) {
		self.data.extend_from_slice(v);
	}

	pub fn bind(&self) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
		}
	}

	pub fn unbind(&self) {
		unsafe {
			gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		}
	}

	pub fn upload(&mut self) {
		// Upload the data to OpenGL
		self.bind();

		// Number of vertices
		let count = self.data.len();

		// Prepare the size and pointers
		let size = (count * std::mem::size_of::<T>()) as isize;
		let ptr = self.data.as_ptr() as *const gl::types::GLvoid;

		// Create a new buffer ?
		if count > self.capacity {
			self.capacity = count;
			unsafe {
				gl::BufferData(gl::ARRAY_BUFFER, size, ptr, self.usage.gl_usage());
			}
		}
		else {
			// Simply update
			unsafe {
				gl::BufferSubData(gl::ARRAY_BUFFER, 0, size, ptr);
			}
		}

		self.unbind();
	}
}


impl<T> Drop for VertexBuffer<T> {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &self.id);
		}
	}
}


