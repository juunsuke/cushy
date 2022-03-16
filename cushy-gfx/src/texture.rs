

use std::sync::Arc;

use cushy_gl::{Texture2D, TexFilters};
use crate::*;


//////////////////////////////////////////////////////////////////////////////////////////////////// Texture

#[derive(Clone, Debug)]
pub struct Texture (Arc<TexType>);

impl PartialEq for Texture {
	fn eq(&self, rhs: &Self) -> bool {
		Arc::ptr_eq(&self.0, &rhs.0)
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

	pub fn size(&self) -> SizeU32 {
		self.0.get_size()
	}

	pub fn bind(&self) {
		self.0.get_underlying().bind();
	}

	pub fn get_uv(&self) -> (Point, Point) {
		self.0.get_uv()
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


//////////////////////////////////////////////////////////////////////////////////////////////////// TexType

#[derive(Debug)]
enum TexType {
	Raw (RawTex),
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

	fn get_underlying(&self) -> &Texture2D {
		match self {
			TexType::Raw (tex) => &tex.tex,
		}
	}

	fn get_size(&self) -> SizeU32 {
		match self {
			TexType::Raw (tex) => tex.size,
		}
	}

	fn get_uv(&self) -> (Point, Point) {
		match self {
			TexType::Raw (_) => (Point::new(0.0, 0.0), Point::new(1.0, 1.0)),
		}
	}
}




