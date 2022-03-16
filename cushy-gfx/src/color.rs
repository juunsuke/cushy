

//////////////////////////////////////////////////////////////////////////////////////////////////// Color

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
pub struct Color (pub u32);

// Helper functions
fn shift(v:u8, c:u32) -> u32		{ (v as u32) << c }
fn unshift(v:u32, c:u32) -> u8		{ ((v >> c) & 0xFF) as u8 }
fn f32_to_u8(v:f32) -> u8			{ (v.clamp(0.0, 1.0) * 255.0) as u8 }
fn u8_to_f32(v:u8) -> f32			{ (v as f32) / 255.0 }


impl Color {

	pub fn from_u8(r:u8, g:u8, b:u8, a:u8) -> Self {
		Color (
			shift(a, 24) |
			shift(b, 16) |
			shift(g, 8) |
			shift(r, 0)
		)
	}

	pub fn from_f32(r:f32, g:f32, b:f32, a:f32) -> Self {
		Self::from_u8(
			f32_to_u8(r),
			f32_to_u8(g),
			f32_to_u8(b),
			f32_to_u8(a),
		)
	}

	pub fn to_u8(&self) -> (u8, u8, u8, u8) {
		(
			unshift(self.0, 0),
			unshift(self.0, 8),
			unshift(self.0, 16),
			unshift(self.0, 24),
		)
	}
	
	pub fn to_f32(&self) -> (f32, f32, f32, f32) {
		let (r, g, b, a) = self.to_u8();

		(
			u8_to_f32(r),
			u8_to_f32(g),
			u8_to_f32(b),
			u8_to_f32(a),
		)
	}
	
	pub fn as_u32(&self) -> u32 {
		self.0
	}
}

impl std::ops::Deref for Color {
	type Target = u32;
	fn deref(&self) -> &u32 {
		&self.0
	}
}

impl From<Color> for (f32, f32, f32, f32) {
	fn from(v: Color) -> Self {
		v.to_f32()
	}
}

impl From<u32> for Color {
	fn from(v: u32) -> Self {
		Self (v)
	}
}

impl From<(f32, f32, f32, f32)> for Color {
	fn from(v: (f32, f32, f32, f32)) -> Self {
		Color::from_f32(v.0, v.1, v.2, v.3)
	}
}

impl From<(f32, f32, f32)> for Color {
	fn from(v: (f32, f32, f32)) -> Self {
		Color::from_f32(v.0, v.1, v.2, 1.0)
	}
}

impl From<(u8, u8, u8, u8)> for Color {
	fn from(v: (u8, u8, u8, u8)) -> Self {
		Color::from_u8(v.0, v.1, v.2, v.3)
	}
}

impl From<(u8, u8, u8)> for Color {
	fn from(v: (u8, u8, u8)) -> Self {
		Color::from_u8(v.0, v.1, v.2, 255)
	}
}








