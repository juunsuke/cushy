


use cgmath::Matrix3;


use cushy_gl::*;
use crate::*;


const VS_SRC: &str = include_str!("../shaders/quad.vert");
const FS_SRC: &str = include_str!("../shaders/quad.frag");


//////////////////////////////////////////////////////////////////////////////////////////////////// Quad

#[repr(C, align(4))]
#[derive(Copy, Clone, Debug)]
pub struct QuadVertex {
	// Transform matrix, split
	pub m1: [f32;3],
	pub m2: [f32;3],
	pub m3: [f32;3],
	
	// Size
	pub w: f32,
	pub h: f32,

	// Color
	pub col: u32,

	// Texture coordinates
	pub u: f32,
	pub v: f32,
	pub du: f32,
	pub dv: f32,
}

impl Default for QuadVertex {
	fn default() -> Self {
		Self {
			m1: [0.0, 0.0, 0.0],
			m2: [0.0, 0.0, 0.0],
			m3: [0.0, 0.0, 0.0],
			w: 0.0,
			h: 0.0,
			col: 0xFFFFFFFF,
			u: 0.0,
			v: 0.0,
			du: 1.0,
			dv: 1.0,
		}
	}
}

impl Vertex for QuadVertex {
	fn attributes() -> Vec<VertexAttrib> {
		// Return the attributes
		vec! [
			// Matrix
			VertexAttrib::Float3(false),
			VertexAttrib::Float3(false),
			VertexAttrib::Float3(false),

			// Size
			VertexAttrib::Float2(false),

			// Color
			VertexAttrib::UByte4,

			// Texture
			VertexAttrib::Float2(false),
			VertexAttrib::Float2(false),
		]
	}

	fn divisor() -> u32 {
		1
	}
}



//////////////////////////////////////////////////////////////////////////////////////////////////// Rot

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug, Default)]
pub struct Rot (pub f32);

use std::f32::consts::PI;
pub fn rad_to_deg(rad: f32) -> f32 { rad * 180.0 / PI }
pub fn deg_to_rad(deg: f32) -> f32 { deg * PI / 180.0 }

impl Rot {

	pub fn from_rad(rad: f32) -> Rot						{ Rot (rad) }
	pub fn from_deg(deg: f32) -> Rot						{ Rot (deg_to_rad(deg)) }

	pub fn as_rad(&self) -> f32								{ self.0 }
	pub fn as_deg(&self) -> f32								{ rad_to_deg(self.0) }
}


//////////////////////////////////////////////////////////////////////////////////////////////////// Quad

#[derive(Clone, Debug)]
pub struct Quad {
	trans: Transform,
	origin: Point,
	size: Option<Size>,
	col: Color,
	tex: Option<Texture>,
}

impl Quad {

	pub fn new() -> Self {
		Self {
			trans: Transform::default(),
			origin: Point::default(),
			size: None,
			col: Color(0xFFFFFFFF),
			tex: None,
		}
	}

	pub fn transform(&self) -> &Transform				{ &self.trans }
	pub fn transform_mut(&mut self) -> &mut Transform	{ &mut self.trans }
	pub fn set_transform(&mut self, t: Transform)		{ self.trans = t }

	pub fn pos(&self) -> Point							{ self.trans.pos }
	pub fn rot(&self) -> Rotation						{ self.trans.rot }
	pub fn scale(&self) -> Scaling						{ self.trans.scale }
	pub fn origin(&self) -> Point						{ self.origin }
	pub fn color(&self) -> Color						{ self.col }
	pub fn texture(&self) -> Option<Texture>			{ self.tex.clone() }

	pub fn set_pos(&mut self, p: Point)					{ self.trans.pos = p; }
	pub fn set_rot(&mut self, r: Rotation)				{ self.trans.rot = r; }
	pub fn set_scale(&mut self, sc: Scaling)			{ self.trans.scale = sc; }
	pub fn set_origin(&mut self, or: Point)				{ self.origin = or; }
	pub fn set_size(&mut self, sz: Size)				{ self.size = Some(sz); }
	pub fn set_color(&mut self, col:Color)				{ self.col = col; }
	
	pub fn size(&self) -> Size {
		// Forced size takes priority
		if let Some(size) = self.size {
			size
		}
		else if let Some(tex) = &self.tex {
			tex.size().into()
		}
		else {
			panic!("Quad has no texture or forced size");
		}
	}

	pub fn set_texture(&mut self, tex: Option<&Texture>) {
		self.tex = tex.map(|t| t.clone());
	}

	pub fn calc_matrix(&self) -> Matrix3<f32> {
		// Calc the transform matrix
		let mut mat = self.trans.calc_matrix();

		// Adjust for origin
		if self.origin.x!=0.0 || self.origin.y!=0.0 {
			mat = mat * self.origin.origin_matrix();
		}

		mat
	}

	pub fn make_vertex(&self) -> QuadVertex {
		// Calc the matrix
		let mat = self.calc_matrix();

		let size = self.size();
		let (uv1, uv2) = if let Some(tex) = &self.tex {
			tex.get_uv()
		}
		else {
			(Point::new(0.0, 0.0), Point::new(0.0, 0.0))
		};

		let u = uv1.x;
		let v = uv1.y;
		let du = uv2.x - u;
		let dv = uv2.y - v;

		// Create a vertex
		QuadVertex {
			m1: *mat.x.as_ref(),
			m2: *mat.y.as_ref(),
			m3: *mat.z.as_ref(),
			w: size.w,
			h: size.h,
			col: self.col.0,
			u,
			v,
			du,
			dv,
		}
	}
}




//////////////////////////////////////////////////////////////////////////////////////////////////// QRBatch

#[derive(Debug)]
struct QRBatch {
	tex: Option<Texture>,
	start: u32,
	count: u32,
}


//////////////////////////////////////////////////////////////////////////////////////////////////// QuadRenderer

#[derive(Debug)]
pub struct QuadRenderer {
	prg: Program,
	vbo: VertexBuffer<QuadVertex>,
	vao: VertexArray,
	data: Vec<QuadVertex>,
	batches: Vec<QRBatch>,
}

impl Default for QuadRenderer {
	fn default() -> Self {
		Self::new()
	}
}

impl QuadRenderer {

	pub fn new() -> Self {
		// Compile the shader
		let prg = Program::from_sources(VS_SRC, FS_SRC).unwrap();

		// Create the VB and VA
		let vbo = VertexBuffer::new(BufferUsage::Stream);
		let mut vao = VertexArray::new();
		vao.add_vertex_buffer(&vbo);

		Self {
			prg,
			vbo,
			vao,
			data: Vec::new(),
			batches: Vec::new(),
		}
	}

	pub fn clear(&mut self) {
		self.data.clear();
	}

	pub fn add(&mut self, q: &mut Quad) {
		// Make the vertex
		let len = self.data.len();
		self.data.push(q.make_vertex());

		// Check if a new batch has to be created
		let tex = q.texture();

		if let Some(batch) = self.batches.last_mut() {
			if batch.tex == tex {
				batch.count += 1;
				return;
			}
		}

		// Create a new batch
		let batch = QRBatch {
			tex: tex.clone(),
			start: len as u32 * 4,
			count: 1,
		};

		self.batches.push(batch);
	}

	pub fn draw(&mut self, cam: &Camera) {
		// Swap out buffer with the VBO's
		self.vbo.swap_data(&mut self.data);
		self.vbo.upload();

		// Bind the program and set uniforms
		self.prg.bind();

		let id = self.prg.find_uniform("UniProj").unwrap();
		let proj = cam.proj_matrix();
		self.prg.set_uniform(id, &UniformValue::Mat4ref(proj.as_ref()));

		// Draw the batches
		for batch in self.batches.iter() {
			if let Some(tex) = &batch.tex {
				tex.bind();
			}
			self.vao.draw_instanced(PrimitiveType::TriangleStrip, batch.start, 4, batch.count);
		}
		
		// Clear everything
		self.batches.clear();
		self.data.clear();
	}
}





