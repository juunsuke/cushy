

use std::f32::consts::PI;
use std::ops::{Div, Mul, Add, Sub};

use cgmath::{Matrix3,vec2};



//////////////////////////////////////////////////////////////////////////////////////////////////// Size

pub type Size = SizeAny<f32>;
pub type SizeU32 = SizeAny<u32>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SizeAny<T> {
	pub w: T,
	pub h: T,
}

impl<T> SizeAny<T> {
	pub fn new(w: T, h: T) -> Self {
		Self { w, h }
	}
}

impl<T:Default> Default for SizeAny<T> {
	fn default() -> Self {
		Self {
			w: T::default(),
			h: T::default(),
		}
	}
}

impl<T:Div<Output=T>+Copy> Div<T> for SizeAny<T> {
	type Output = SizeAny<T>;

	fn div(self, rhs: T) -> Self::Output {
		SizeAny {
			w: self.w/rhs,
			h: self.h/rhs,
		}
	}
}

impl<T:Mul<Output=T>+Copy> Mul<T> for SizeAny<T> {
	type Output = SizeAny<T>;

	fn mul(self, rhs: T) -> Self::Output {
		SizeAny {
			w: self.w*rhs,
			h: self.h*rhs,
		}
	}
}


impl From<SizeU32> for Size {
	fn from(v: SizeU32) -> Size {
		Size {
			w: v.w as f32,
			h: v.h as f32,
		}
	}
}


//////////////////////////////////////////////////////////////////////////////////////////////////// Transform

#[derive(Copy, Clone, Debug, Default)]
pub struct Transform {
	// Translation
	pub pos: Point,

	// Rotation
	pub rot: Rotation,

	// Scale
	pub scale: Scaling,
}

impl Transform {

	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_pos(mut self, pos: Point) -> Self {
		self.pos = pos;
		self
	}

	pub fn with_rot(mut self, rot: Rotation) -> Self {
		self.rot = rot;
		self
	}

	pub fn with_scale(mut self, scale: Scaling) -> Self {
		self.scale = scale;
		self
	}

	pub fn with_all(mut self, pos: Point, rot: Rotation, scale: Scaling) -> Self {
		self.pos = pos;
		self.rot = rot;
		self.scale = scale;
		self
	}

	pub fn calc_matrix(&self) -> Matrix3<f32> {
		// Calc the transform matrix

		// Translation
		let mut mat = self.pos.translation_matrix();

		// Rotation
		if self.rot.0 != 0.0 {
			mat = mat * self.rot.rotation_matrix();
		}

		// Scale
		if self.scale.x!=1.0 || self.scale.y!=1.0 {
			mat = mat * self.scale.scaling_matrix();
		}

		mat
	}

}

//////////////////////////////////////////////////////////////////////////////////////////////////// Point

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}

impl Point {

	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}

	pub fn translation_matrix(&self) -> Matrix3<f32> {
		// Create a translation matrix from this point
		Matrix3::from_translation(vec2(self.x, self.y))
		
	}

	pub fn origin_matrix(&self) -> Matrix3<f32> {
		// Create a translation matrix from this point
		Matrix3::from_translation(vec2(-self.x, -self.y))
	}

}

impl From<Size> for Point {
	fn from(v: Size) -> Point {
		Point {
			x: v.w,
			y: v.h,
		}
	}
}

impl Add for Point {
	type Output = Point;

	fn add(self, rhs: Point) -> Point {
		Point {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl Sub for Point {
	type Output = Point;

	fn sub(self, rhs: Point) -> Point {
		Point {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl Mul<f32> for Point {
	type Output = Point;

	fn mul(self, rhs: f32) -> Point {
		Point {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}


//////////////////////////////////////////////////////////////////////////////////////////////////// Rotation

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Rotation (pub f32);

impl Rotation {

	pub fn from_deg(deg: f32) -> Self {
		// Create from degrees
		Self (Self::deg_to_rad(deg))
	}

	pub fn from_rad(rad: f32) -> Self {
		// Create from radians
		Self (rad)
	}

	pub fn as_deg(&self) -> f32 {
		Self::rad_to_deg(self.0)
	}

	pub fn as_rad(&self) -> f32 {
		self.0
	}

	pub fn rad_to_deg(rad: f32) -> f32 {
		rad * 180.0 / PI
	}

	pub fn deg_to_rad(deg: f32) -> f32 {
		deg * PI / 180.0
	}

	pub fn rotation_matrix(&self) -> Matrix3<f32> {
		Matrix3::from_angle_z(cgmath::Rad(self.0))
	}

}


//////////////////////////////////////////////////////////////////////////////////////////////////// Scale

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Scaling {
	pub x: f32,
	pub y: f32,
}

impl Scaling {

	pub fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}

	pub fn reset(&mut self) {
		// Reset the default scaling
		self.x = 1.0;
		self.y = 1.0;
	}

	pub fn uniform(&mut self, s: f32) {
		// Uniform scaling
		self.x = s;
		self.y = s;
	}

	pub fn scaling_matrix(&self) -> Matrix3<f32> {
		Matrix3::from_nonuniform_scale(self.x, self.y)
	}

}

impl Default for Scaling {
	fn default() -> Self {
		Self {
			x: 1.0,
			y: 1.0,
		}
	}
}






