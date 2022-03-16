
use crate::*;
use crate::api as gl;


///////////////////////////////////////////////////////////////////////////////////////////////////// PrimitiveType

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PrimitiveType {
	Points,
	LineStrip,
	LineLoop,
	Lines,
	LineStripAdjacency,
	LinesAdjacency,
	TriangleStrip,
	TriangleFan,
	Triangles,
	TriangleStripAdjacency,
	TrianglesAdjacency,
}

impl PrimitiveType {

	pub fn gl_type(&self) -> u32 {
		use PrimitiveType::*;
		match self {
			Points => gl::POINTS,
			LineStrip => gl::LINE_STRIP,
			LineLoop => gl::LINE_LOOP,
			Lines => gl::LINES,
			LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
			LinesAdjacency => gl::LINES_ADJACENCY,
			TriangleStrip => gl::TRIANGLE_STRIP,
			TriangleFan => gl::TRIANGLE_FAN,
			Triangles => gl::TRIANGLES,
			TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
			TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
		}
	}

}

///////////////////////////////////////////////////////////////////////////////////////////////////// VertexArray

#[derive(Debug)]
pub struct VertexArray {
	id: u32,
	base_index: u32,
}

impl VertexArray {
	pub fn new() -> VertexArray {
		// Create a VAO
		let mut id: u32 = 0;
		unsafe {
			gl::GenVertexArrays(1, &mut id);
		}

		// Create the VA
		let va = VertexArray {
			id,
			base_index: 0,
		};

		va
	}

	pub fn id(&self) -> u32 {
		self.id
	}

	pub fn bind(&self) {
		// Bind the VAO
		unsafe {
			gl::BindVertexArray(self.id);
		}
	}

	pub fn unbind(&self) {
		unsafe {
			gl::BindVertexArray(0);
		}
	}

	pub fn add_vertex_buffer<T>(&mut self, vb: &VertexBuffer<T>)
	where
		T: Vertex
	{
		// Add the VBO to the VAO
		self.bind();
		vb.bind();

		// Register the attributes
		let mut pos = 0;
		let stride = std::mem::size_of::<T>() as i32;
		let div = T::divisor();

		for attr in T::attributes().iter() {
			// Extract the GL data
			let (typ, count, norm, size) = attr.gl_data();

			unsafe {
				gl::EnableVertexAttribArray(self.base_index);
				gl::VertexAttribPointer(
					self.base_index,
					count,
					typ,
					norm,
					stride,
					pos as *const gl::types::GLvoid
				);

				if div>0 {
					gl::VertexAttribDivisor(self.base_index, div);
				}
			}

			self.base_index += 1;
			pos += size;
		}

		vb.unbind();
		self.unbind();
	}

	pub fn draw_nobind(&self, prim: PrimitiveType, first: u32, count: u32) {
		// Draw
		unsafe {
			gl::DrawArrays(prim.gl_type(), first as i32, count as i32);
		}
	}

	pub fn draw(&self, prim: PrimitiveType, first: u32, count: u32) {
		self.bind();
		self.draw_nobind(prim, first, count);
		self.unbind();
	}

	pub fn draw_instanced_nobind(&self, prim: PrimitiveType, first: u32, count: u32, inst: u32) {
		// Draw
		unsafe {
			gl::DrawArraysInstanced(prim.gl_type(), first as i32, count as i32, inst as i32);
		}
	}

	pub fn draw_instanced(&self, prim: PrimitiveType, first: u32, count: u32, inst: u32) {
		self.bind();
		self.draw_instanced_nobind(prim, first, count, inst);
		self.unbind();
	}

}


impl Drop for VertexArray {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteBuffers(1, &self.id);
		}
	}
}

