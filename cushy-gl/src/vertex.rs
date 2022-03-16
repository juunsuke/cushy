

use crate::api as gl;


///////////////////////////////////////////////////////////////////////////////////////////////////// VertexAttrib

pub enum VertexAttrib {
	Float1 (bool),
	Float2 (bool),
	Float3 (bool),
	Float4 (bool),

	Byte4,
	UByte4,

	Int1,
	Int2,
	Int3,
	Int4,

	UInt1,
	UInt2,
	UInt3,
	UInt4,
}

impl VertexAttrib {
	pub fn gl_data(&self) -> (gl::types::GLenum, gl::types::GLint, gl::types::GLboolean, usize) {
		use VertexAttrib::*;
		match *self {
			Float1 (norm)	=> (gl::FLOAT, 1, if norm {gl::TRUE} else {gl::FALSE}, 4),
			Float2 (norm)	=> (gl::FLOAT, 2, if norm {gl::TRUE} else {gl::FALSE}, 8),
			Float3 (norm)	=> (gl::FLOAT, 3, if norm {gl::TRUE} else {gl::FALSE}, 12),
			Float4 (norm)	=> (gl::FLOAT, 4, if norm {gl::TRUE} else {gl::FALSE}, 16),
			Byte4			=> (gl::BYTE, 4, gl::TRUE, 4),
			UByte4			=> (gl::UNSIGNED_BYTE, 4, gl::TRUE, 4),
			Int1			=> (gl::INT, 1, gl::TRUE, 4),
			Int2			=> (gl::INT, 2, gl::TRUE, 8),
			Int3			=> (gl::INT, 3, gl::TRUE, 12),
			Int4			=> (gl::INT, 4, gl::TRUE, 16),
			UInt1			=> (gl::UNSIGNED_INT, 1, gl::TRUE, 4),
			UInt2			=> (gl::UNSIGNED_INT, 2, gl::TRUE, 8),
			UInt3			=> (gl::UNSIGNED_INT, 3, gl::TRUE, 12),
			UInt4			=> (gl::UNSIGNED_INT, 4, gl::TRUE, 16),
		}
	}
}


///////////////////////////////////////////////////////////////////////////////////////////////////// Vertex

pub trait Vertex: Copy+Clone {
	fn attributes() -> Vec<VertexAttrib>;
	fn divisor() -> u32 { 0 }
}


///////////////////////////////////////////////////////////////////////////////////////////////////// PosVertex

#[repr(C, align(4))]
#[derive(Copy, Clone)]
pub struct PosVertex {
	// Position
	pub x: f32,
	pub y: f32,
}

impl Vertex for PosVertex {
	fn attributes() -> Vec<VertexAttrib> {
		// Return the attributes
		vec! [ VertexAttrib::Float2(false)	]
	}
}


///////////////////////////////////////////////////////////////////////////////////////////////////// ColorVertex

#[repr(C, align(4))]
#[derive(Copy, Clone)]
pub struct ColorVertex {
	// Color
	pub col: u32,
}

impl Vertex for ColorVertex {
	fn attributes() -> Vec<VertexAttrib> {
		// Return the attributes
		vec! [ VertexAttrib::UByte4 ]
	}
}


///////////////////////////////////////////////////////////////////////////////////////////////////// TexCoordVertex

#[repr(C, align(4))]
#[derive(Copy, Clone)]
pub struct TexCoordVertex {
	// Texture coordinates
	pub u: f32,
	pub v: f32,
}

impl Vertex for TexCoordVertex {
	fn attributes() -> Vec<VertexAttrib> {
		// Return the attributes
		vec! [ VertexAttrib::Float2(false)	]
	}
}



