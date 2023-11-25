
use rand::Rng;


/// Represents a 32-bit RGBA color
///
/// This is a newtype struct wrapping a u32 value.  It provides methods to manipulate individual
/// components of a color, either using u8 or f32 values.  Manipulating through f32 values has a
/// performance cost and is provided for convinience when performance is not an issue.
///
/// The components are laid out in ABGR order, with A being the highest-order 8 bits.
/// This is the format provided as-is to OpenGL in texture data and vertex buffers.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Color (pub u32);


fn shift(v:u8, n:u32) -> u32 {
	(v as u32) << n
}

fn unshift(v:u32, n:u32) -> u8 {
	((v>>n) & 0xFF) as u8
}

fn f32_to_u8(v:f32) -> u8 {
	(v.clamp(0.0, 1.0)*255.0) as u8
}

fn u8_to_f32(v:u8) -> f32 {
	(v as f32)/255.0
}


impl Color
{
	/// Generate a random color
	///
	/// This uses the [`rand`] crate and uses the thread's Rng.
	pub fn random() -> Color {
		// Generate a random color
		let mut rng = rand::thread_rng();
		Color::from_u8(rng.gen(), rng.gen(), rng.gen(), 255)
	}

	/// Build a color from u8 components
	pub fn from_u8(r:u8, g:u8, b:u8, a:u8) -> Color {
		Color(
			shift(a, 24) |
			shift(b, 16) |
			shift(g, 8) |
			shift(r, 0)
		)
	}

	/// Build a color from f32 components
	///
	/// Each component is first clamped to t [0.0..1.0=] range and then converted to a u8.
	pub fn from_f32(r:f32, g:f32, b:f32, a:f32) -> Color {
		Color::from_u8(
			f32_to_u8(r),
			f32_to_u8(g),
			f32_to_u8(b),
			f32_to_u8(a)
		)
	}

	/// Represent the color as a u32
	///
	/// Can also access the value directly:
	/// ```
	/// let v = Color::random().0;
	/// ```
	pub fn as_u32(&self) -> u32 {
		self.0
	}

	/// Break down the color into R, G, B, A components
	pub fn as_u8(&self) -> (u8, u8, u8, u8) {
		(
			unshift(self.0, 0),
			unshift(self.0, 8),
			unshift(self.0, 16),
			unshift(self.0, 24),
		)
	}

	/// Break down the color into R, G, B, A components
	///
	/// Each component is converted to an f32 with a range of [0.0..1.0=]
	pub fn as_f32(&self) -> (f32, f32, f32, f32) {
		let (r, g, b, a) = self.as_u8();
		(
			u8_to_f32(r),
			u8_to_f32(g),
			u8_to_f32(b),
			u8_to_f32(a),
		)
	}

	/// Return a new color with a modified A component
	pub fn with_alpha(&self, a: f32) -> Color {
		Color((self.0 & 0x00FFFFFF) | shift(f32_to_u8(a), 24))
	}

	fn blend(s:u8, d:u8, a:u8) -> u8 {
		let s = s as u32;
		let d = d as u32;
		let a = a as u32;

		((s*a/255) + (d*(255-a)/255)) as u8
	}

	/// Blend two color together using an src-alpha algorithm
	///
	/// See [`Canvas::draw_text`](super::Canvas::draw_text) for an explaination of the algorithm
	pub fn blend_srcalpha(src: Color, dst: Color) -> Color {
		// Src-alpha color blend
		let (sr, sg, sb, sa) = src.as_u8();

		// No blinding ?
		if sa==255 {
			return src;
		}
		if sa==0 {
			return dst;
		}

		let (dr, dg, db, da) = dst.as_u8();

		Color::from_u8(
			Color::blend(sr, dr, sa),
			Color::blend(sg, dg, sa),
			Color::blend(sb, db, sa),
			da
		)
	}

	pub fn none() -> Color {
		Color(0)
	}

	pub fn transparent() -> Color {
		Color(0)
	}

	pub fn black() -> Color {
		Color(0xFF000000)
	}

	pub fn white() -> Color {
		Color(0xFFFFFFFF)
	}

	pub fn gray(v: f32) -> Color {
		let v = f32_to_u8(v);
		Color::from_u8(v, v, v, 255)
	}
}

impl From<u32> for Color {
	fn from(v: u32) -> Color {
		Color(v)
	}
}

impl From<(u8, u8, u8, u8)> for Color {
	fn from(v:(u8, u8, u8, u8)) -> Color {
		Color::from_u8(v.0, v.1, v.2, v.3)
	}
}

impl From<(f32, f32, f32, f32)> for Color {
	fn from(v:(f32, f32, f32, f32)) -> Color {
		Color::from_f32(v.0, v.1, v.2, v.3)
	}
}

impl From<Color> for u32 {
	fn from(col: Color) -> u32 {
		col.0
	}
}
