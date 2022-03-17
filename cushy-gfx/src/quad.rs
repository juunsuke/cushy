


use cgmath::{Matrix3, vec3};
use rayon::prelude::*;

use cushy_gl::*;
use crate::*;


const VS_CPU_SRC: &str = include_str!("../shaders/cpu_quad.vert");
const VS_GPU_SRC: &str = include_str!("../shaders/gpu_quad.vert");
const FS_SRC: &str = include_str!("../shaders/quad.frag");


//////////////////////////////////////////////////////////////////////////////////////////////////// QuadRenderType

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum QuadRendererType {
	Cpu,
	Gpu,
}

impl QuadRendererType {
	fn compile_program(&self) -> Program {
		match self {
			QuadRendererType::Cpu => Program::from_sources(VS_CPU_SRC, FS_SRC).unwrap(),
			QuadRendererType::Gpu => Program::from_sources(VS_GPU_SRC, FS_SRC).unwrap(),
		}
	}
}

//////////////////////////////////////////////////////////////////////////////////////////////////// QuadVertexCpu

#[repr(C, align(4))]
#[derive(Copy, Clone, Debug)]
pub struct QuadVertexCpu {
	// Transformed vertex
	pub vert: [f32; 3],

	// Color
	pub col: u32,

	// Texture coordinates
	pub u: f32,
	pub v: f32,
}

impl Vertex for QuadVertexCpu {
	fn attributes() -> Vec<VertexAttrib> {
		// Return the attributes
		vec! [
			// Vertex
			VertexAttrib::Float3(false),

			// Color
			VertexAttrib::UByte4,

			// Texture
			VertexAttrib::Float2(false),
		]
	}
}


//////////////////////////////////////////////////////////////////////////////////////////////////// QuadVertexGpu

#[repr(C, align(4))]
#[derive(Copy, Clone, Debug)]
pub struct QuadVertexGpu {
	// Translation
	pub tx: f32,
	pub ty: f32,

	// Rot
	pub rot: f32,

	// Scale
	pub sx: f32,
	pub sy: f32,

	// Origin
	pub ox: f32,
	pub oy: f32,

	// Position
	pub px: f32,
	pub py: f32,

	// Color
	pub col: u32,

	// Texture coordinates
	pub u: f32,
	pub v: f32,
}

impl Vertex for QuadVertexGpu {
	fn attributes() -> Vec<VertexAttrib> {
		// Return the attributes
		vec! [
			// Trans
			VertexAttrib::Float2(false),

			// Rot
			VertexAttrib::Float1(false),

			// Scale
			VertexAttrib::Float2(false),

			// Origin
			VertexAttrib::Float2(false),

			// Pos
			VertexAttrib::Float2(false),

			// Color
			VertexAttrib::UByte4,

			// Texture
			VertexAttrib::Float2(false),
		]
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
	
	pub fn has_size(&self) -> bool {
		self.tex.is_some() || self.size.is_some()
	}

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

	pub fn make_vertex_cpu(&self) -> [QuadVertexCpu; 4] {
		// Calc the matrix
		let mat = self.calc_matrix();
		
		// Size and UV
		let size = self.size();
		let (uv1, uv2) = if let Some(tex) = &self.tex {
			tex.get_uv()
		}
		else {
			(Point::new(0.0, 0.0), Point::new(0.0, 0.0))
		};

		let u1 = uv1.x;
		let v1 = uv1.y;
		let u2 = uv2.x;
		let v2 = uv2.y;

		let w = size.w;
		let h = size.h;

		let p0 = mat * vec3(0.0, 0.0, 1.0);
		let p1 = mat * vec3(w,   0.0, 1.0);
		let p2 = mat * vec3(0.0, h,   1.0);
		let p3 = mat * vec3(w,   h,   1.0);

		// Make the vertices
		[
			QuadVertexCpu {
				vert: p0.into(),
				col: self.col.0,
				u: u1,
				v: v1,
			},
			QuadVertexCpu {
				vert: p1.into(),
				col: self.col.0,
				u: u2,
				v: v1,
			},
			QuadVertexCpu {
				vert: p2.into(),
				col: self.col.0,
				u: u1,
				v: v2,
			},
			QuadVertexCpu {
				vert: p3.into(),
				col: self.col.0,
				u: u2,
				v: v2,
			},
		]
	}

	pub fn make_vertex_gpu(&self) -> [QuadVertexGpu; 4] {
		// Size and UV
		let size = self.size();
		let (uv1, uv2) = if let Some(tex) = &self.tex {
			tex.get_uv()
		}
		else {
			(Point::new(0.0, 0.0), Point::new(0.0, 0.0))
		};

		let u1 = uv1.x;
		let v1 = uv1.y;
		let u2 = uv2.x;
		let v2 = uv2.y;

		let w = size.w;
		let h = size.h;

		let p0 = Point::new(0.0, 0.0);
		let p1 = Point::new(w,   0.0);
		let p2 = Point::new(0.0, h);
		let p3 = Point::new(w,   h);

		// Transformationc omponents
		let tx = self.trans.pos.x;
		let ty = self.trans.pos.y;
		let rot = self.trans.rot.as_rad();
		let sx = self.trans.scale.x;
		let sy = self.trans.scale.y;
		let ox = self.origin.x;
		let oy = self.origin.y;

		// Make the vertices
		[
			QuadVertexGpu {
				tx, ty, rot, sx, sy, ox, oy,
				px: p0.x,
				py: p0.y,
				col: self.col.0,
				u: u1,
				v: v1,
			},
			QuadVertexGpu {
				tx, ty, rot, sx, sy, ox, oy,
				px: p1.x,
				py: p1.y,
				col: self.col.0,
				u: u2,
				v: v1,
			},
			QuadVertexGpu {
				tx, ty, rot, sx, sy, ox, oy,
				px: p2.x,
				py: p2.y,
				col: self.col.0,
				u: u1,
				v: v2,
			},
			QuadVertexGpu {
				tx, ty, rot, sx, sy, ox, oy,
				px: p3.x,
				py: p3.y,
				col: self.col.0,
				u: u2,
				v: v2,
			},
		]
	}
}




//////////////////////////////////////////////////////////////////////////////////////////////////// QRBatch

#[derive(Debug)]
struct QRBatch {
	tex: Option<Texture>,
	start: u32,
	count: u32,
}


//////////////////////////////////////////////////////////////////////////////////////////////////// QRBuffer

#[derive(Debug)]
enum QRBuffer {
	Cpu (VertexBuffer<QuadVertexCpu>),
	Gpu (VertexBuffer<QuadVertexGpu>),
}

//////////////////////////////////////////////////////////////////////////////////////////////////// QuadRenderer

#[derive(Debug)]
pub struct QuadRenderer {
	qrt: QuadRendererType,
	prg: Program,
	qrb: QRBuffer,
	vao: VertexArray,
	ibo: IndexBuffer,
	ibo_size: u32,
	data: Vec<Quad>,
	batches: Vec<QRBatch>,
}

impl QuadRenderer {

	fn new_cpu(vao: &mut VertexArray) -> QRBuffer {
		// Create the VBO and VAO
		let vbo = VertexBuffer::new(BufferUsage::Stream);
		vao.add_vertex_buffer(&vbo);
		QRBuffer::Cpu(vbo)
	}

	fn new_gpu(vao: &mut VertexArray) -> QRBuffer {
		// Create the VBO and VAO
		let vbo = VertexBuffer::new(BufferUsage::Stream);
		vao.add_vertex_buffer(&vbo);
		QRBuffer::Gpu(vbo)
	}

	pub fn new(qrt: QuadRendererType) -> Self {
		// Compile the appropriate shader
		let prg = qrt.compile_program();

		// VAO
		let mut vao = VertexArray::new();

		// Create the proper variant of vertex buffer
		let qrb = match qrt {
			QuadRendererType::Cpu => Self::new_cpu(&mut vao),
			QuadRendererType::Gpu => Self::new_gpu(&mut vao),
		};

		// Create the IB
		let ibo = IndexBuffer::new(BufferUsage::Static);

		let mut qr = Self {
			qrt,
			prg,
			qrb,
			vao,
			ibo,
			ibo_size: 0,
			data: Vec::new(),
			batches: Vec::new(),
		};
		
		// Pre-fill the IB for 1024 quads, it shouldn't have to change much
		// It will be made bigger if more than 1024 quads are needed
		qr.resize_ibo(1024);

		qr
	}

	pub fn clear(&mut self) {
		self.data.clear();
	}

	pub fn add(&mut self, q: &Quad) {
		// Skip if it has no size
		if !q.has_size() {
			return;
		}

		// Copy the quad
		let len = self.data.len();
		self.data.push(q.clone());

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
			start: len as u32,
			count: 1,
		};

		self.batches.push(batch);
	}

	fn resize_ibo(&mut self, size: u32) {
		// Resize the static IBO
		self.ibo_size = size;
		self.ibo.auto_quads(size);
		self.ibo.upload();
	}

	fn fit_ibo(&mut self, size: u32) {
		// Make sure the static IBO is large enough
		if size > self.ibo_size {
			self.resize_ibo(size);
		}
	}

	fn build_vertices_cpu(&mut self) -> Vec<QuadVertexCpu> {
		// Build the vertices for all the quads in parallel
		let mut vtx = Vec::with_capacity(self.data.len());
		let data = std::mem::replace(&mut self.data, Vec::new());

		data
			.into_par_iter()
			.map(|q| q.make_vertex_cpu())
			.collect_into_vec(&mut vtx);

		// The created vector is of type Vec<[QuadVertexCpu;4]>
		// It needs to be converted into Vec<QuadVertexCpu>
		let mut vtx = std::mem::ManuallyDrop::new(vtx);
		let ptr = vtx.as_mut_ptr() as *mut QuadVertexCpu;
		let len = vtx.len();
		let capacity = vtx.capacity();

		unsafe { Vec::from_raw_parts(ptr, len*4, capacity*4) }
	}

	fn build_vertices_gpu(&mut self) -> Vec<QuadVertexGpu> {
		// Build the vertices for all the quads in parallel
		let mut vtx = Vec::with_capacity(self.data.len());
		let data = std::mem::replace(&mut self.data, Vec::new());

		data
			.into_par_iter()
			.map(|q| q.make_vertex_gpu())
			.collect_into_vec(&mut vtx);

		// The created vector is of type Vec<[QuadVertexCpu;4]>
		// It needs to be converted into Vec<QuadVertexCpu>
		let mut vtx = std::mem::ManuallyDrop::new(vtx);
		let ptr = vtx.as_mut_ptr() as *mut QuadVertexGpu;
		let len = vtx.len();
		let capacity = vtx.capacity();

		unsafe { Vec::from_raw_parts(ptr, len*4, capacity*4) }
	}

	pub fn size(&self) -> usize {
		// Number of queued quads
		self.data.len()
	}

	pub fn draw(&mut self, cam: &Camera) {
		// Get the number of quads first
		let len = self.data.len();

		// Build the vertices and uplaod them
		match self.qrt {
			QuadRendererType::Cpu => {
				let vtx = self.build_vertices_cpu();

				if let QRBuffer::Cpu(ref mut vbo) = self.qrb {
					vbo.set_data(vtx);
					vbo.upload();
				}
			},

			QuadRendererType::Gpu => {
				let vtx = self.build_vertices_gpu();

				if let QRBuffer::Gpu(ref mut vbo) = self.qrb {
					vbo.set_data(vtx);
					vbo.upload();
				}
			},
		}

		// Make sure the IBO is large enough
		self.fit_ibo(len as u32);

		// Bind the program and set uniforms
		self.prg.bind();

		let id = self.prg.find_uniform("UniProj").unwrap();
		let proj = cam.proj_matrix();
		self.prg.set_uniform(id, &UniformValue::Mat4ref(proj.as_ref()));

		self.vao.bind();
		self.ibo.bind();

		// Draw the batches
		for batch in self.batches.iter() {
			if let Some(tex) = &batch.tex {
				tex.bind();
			}

			self.ibo.draw_nobind(PrimitiveType::Triangles, batch.start*6, batch.count*6);
		}

		self.ibo.unbind();
		self.vao.unbind();
		
		// Clear everything
		self.batches.clear();
		self.data.clear();
	}
}





