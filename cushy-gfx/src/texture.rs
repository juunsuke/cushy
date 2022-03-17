

use std::sync::Arc;

use cushy_gl::{Texture2D, TexFilters};
use crate::*;


//////////////////////////////////////////////////////////////////////////////////////////////////// Texture

#[derive(Clone, Debug)]
pub struct Texture (Arc<TexType>);

impl PartialEq for Texture {
	fn eq(&self, rhs: &Self) -> bool {
		self.get_underlying() == rhs.get_underlying()
	}
}

impl Texture {
	
	pub fn from_canvas(cnv: &Canvas) -> Texture {
		let tex = TexType::new_raw(
			cnv.size(),
			TexFilters::NearestNearest,
			Some(cnv.data()),
		);

		Texture(Arc::new(tex))
	}

	pub fn sub(&self, r: &RectU32) -> Texture {
		// Create a sub-texture
		let tex = TexType::new_sub(
			&self.0,
			r,
		);

		Texture(Arc::new(tex))
	}

	pub fn size(&self) -> SizeU32 {
		self.0.get_size()
	}

	pub fn bind(&self) {
		self.0.get_underlying().bind();
	}

	pub fn get_uv(&self) -> (Point, Point) {
		self.0.get_uv()
	}

	pub fn get_underlying(&self) -> &Texture2D {
		self.0.get_underlying()
	}
}

impl From<Canvas> for Texture {
	fn from(cnv: Canvas) -> Texture {
		Texture::from_canvas(&cnv)
	}
}

impl From<&Canvas> for Texture {
	fn from(cnv: &Canvas) -> Texture {
		Texture::from_canvas(cnv)
	}
}



//////////////////////////////////////////////////////////////////////////////////////////////////// RawTex

#[derive(Debug)]
struct RawTex {
	tex: Texture2D,
	size: SizeU32,
}


//////////////////////////////////////////////////////////////////////////////////////////////////// SubTex

#[derive(Debug)]
struct SubTex {
	parent: Arc<TexType>,
	r: RectU32,
}


//////////////////////////////////////////////////////////////////////////////////////////////////// TexType

#[derive(Debug)]
enum TexType {
	Raw (RawTex),
	Sub (SubTex),
}

impl TexType {
	
	fn new_raw(size: SizeU32, filters: TexFilters, data: Option<&[u32]>) -> TexType {
		// Create a new raw texture entry
		let tex = Texture2D::new(filters);

		// Upload the data
		if let Some(data) = data {
			tex.upload_raw(size.w, size.h, data);
		}

		let raw = RawTex {
			tex,
			size,
		};

		TexType::Raw(raw)
	}

	fn new_sub(parent: &Arc<TexType>, r: &RectU32) -> TexType {
		// Since the parent can change its size at any time, no time will be
		// wasted checking the bounds of 'r', neither here nor when UV is calculated
		// Proving an out-of-bounds rect is undefined behavior, but should not cause
		// issues other than not displaying properly
		let sub = SubTex {
			parent: Arc::clone(parent),
			r: *r,
		};

		TexType::Sub(sub)
	}

	fn get_underlying(&self) -> &Texture2D {
		match self {
			TexType::Raw (tex) => &tex.tex,
			TexType::Sub (tex) => tex.parent.get_underlying(),
		}
	}

	fn get_size(&self) -> SizeU32 {
		match self {
			TexType::Raw (tex) => tex.size,
			TexType::Sub (tex) => tex.r.size(),
		}
	}

	fn get_uv(&self) -> (Point, Point) {
		match self {
			TexType::Raw (_) => (Point::new(0.0, 0.0), Point::new(1.0, 1.0)),

			TexType::Sub (sub) => {
				// Calc the UV coordinates of the sub-texture
				let (uv1, uv2) = sub.parent.get_uv();
				let psize = sub.parent.get_size();
				let du = (uv2.x - uv1.x) / (psize.w as f32);
				let dv = (uv2.y - uv1.y) / (psize.h as f32);

				let sx = sub.r.x as f32;
				let sy = sub.r.y as f32;
				let sw = sub.r.w as f32;
				let sh = sub.r.h as f32;

				let u1 = uv1.x + (sx * du);
				let v1 = uv1.y + (sy * dv);
				let u2 = u1 + (sw * du);
				let v2 = v1 + (sh * dv);

				(Point::new(u1, v1), Point::new(u2, v2))
			}
		}
	}
}




