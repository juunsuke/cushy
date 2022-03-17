

use std::f32::consts::PI;
use std::ops::{Div, Mul, Add, Sub};

use cgmath::{Matrix3,vec2};



//////////////////////////////////////////////////////////////////////////////////////////////////// Size

pub type Size = SizeAny<f32>;
pub type SizeU32 = SizeAny<u32>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct SizeAny<T: Copy> {
	pub w: T,
	pub h: T,
}

impl<T:Copy> SizeAny<T> {
	pub fn new(w: T, h: T) -> Self {
		Self { w, h }
	}
}

impl<T:Copy+Default> Default for SizeAny<T> {
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


//////////////////////////////////////////////////////////////////////////////////////////////////// Rect

pub type Rect = RectAny<f32>;
pub type RectU32 = RectAny<u32>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct RectAny<T: Copy> {
	pub x: T,
	pub y: T,
	pub w: T,
	pub h: T,
}

impl<T: Copy> RectAny<T> {
	pub fn new(x: T, y: T, w: T, h: T) -> Self {
		Self { x, y, w, h }
	}

	pub fn pos(&self) -> PointAny<T> {
		PointAny::new(self.x, self.y)
	}

	pub fn size(&self) -> SizeAny<T> {
		SizeAny::new(self.w, self.h)
	}
}

impl<T: Copy> RectAny<T>
where
	T: PartialOrd + Add<T, Output=T>,
{
	pub fn contains(&self, p: PointAny<T>) -> bool {
		// Check if a point is contained
		p.x >= self.x
		&& p.y >= self.y
		&& p.x < (self.x+self.w)
		&& p.y < (self.y+self.h)
	}

	pub fn contains_all(&self, inner: &Self) -> bool {
		// Check if 'inner' fits completely into self
		inner.x >= self.x
		&& inner.y >= self.y
		&& (inner.x+inner.w) <= (self.x+self.w)
		&& (inner.y+inner.h) <= (self.y+self.h)
	}

	pub fn contains_none(&self, other: &Self) -> bool {
		// Check if 'other' is completely outside of self
		// >= and <= are used because borders are between pixels and are allowed to touch
		other.x >= (self.x+self.w)
		|| other.y >= (self.y+self.h)
		|| (other.x+other.w) <= self.x
		|| (other.y+other.h) <= self.y
	}

	pub fn intersects(&self, other: &Self) -> bool {
		!self.contains_none(other)
	}
}

impl<T: Copy> RectAny<T>
where
	T: PartialOrd
		+ Add<T, Output=T>
		+ Sub<T, Output=T>
		+ std::ops::SubAssign
		+ num::Zero,
{
	pub fn clip(&self, other: &Self) -> Option<(Self, PointAny<T>)> {
		// Clip 'other' so that it fits within self
		// Return None if completely outside
		let zero = T::zero();

		if (self.w <= zero)
			|| (other.w <= zero)
			|| self.contains_none(other)
		{
			// Completely outside
			None
		}
		else {
			// At least part is inside
			let mut x = other.x;
			let mut y = other.y;
			let mut w = other.w;
			let mut h = other.h;

			if x < self.x {
				w -= self.x - x;
				x = self.x;
			}
			if y < self.y {
				h -= self.y - y;
				y = self.y;
			}

			if (x+w) > (self.x+self.w) {
				w = (self.x+self.w) - x;
			}
			if (y+h) > (self.y+self.h) {
				h = (self.y+self.h) - y;
			}

			let r = Self::new(x, y, w, h);
			let p = PointAny::new(x-other.x, y-other.y);

			Some((r, p))
		}
	}
}

impl<T:Copy+Default> Default for RectAny<T> {
	fn default() -> Self {
		Self {
			x: T::default(),
			y: T::default(),
			w: T::default(),
			h: T::default(),
		}
	}
}

impl From<RectU32> for Rect {
	fn from(v: RectU32) -> Rect {
		Rect {
			x: v.x as f32,
			y: v.y as f32,
			w: v.w as f32,
			h: v.h as f32,
		}
	}
}


//////////////////////////////////////////////////////////////////////////////////////////////////// Point

pub type Point = PointAny<f32>;
pub type PointU32 = PointAny<u32>;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PointAny<T: Clone> {
	pub x: T,
	pub y: T,
}

impl<T:Copy+Default> Default for PointAny<T> {
	fn default() -> Self {
		Self {
			x: T::default(),
			y: T::default(),
		}
	}
}

impl<T:Copy> PointAny<T> {

	pub fn new(x: T, y: T) -> Self {
		Self {
			x,
			y
		}
	}

}

impl PointAny<f32> {

	pub fn translation_matrix(&self) -> Matrix3<f32> {
		// Create a translation matrix from this point
		Matrix3::from_translation(vec2(self.x, self.y))
		
	}

	pub fn origin_matrix(&self) -> Matrix3<f32> {
		// Create a translation matrix from this point
		Matrix3::from_translation(vec2(-self.x, -self.y))
	}

}


impl<T:Copy> From<SizeAny<T>> for PointAny<T> {
	fn from(v: SizeAny<T>) -> Self {
		Self {
			x: v.w,
			y: v.h,
		}
	}
}

impl<T> Add for PointAny<T>
where
	T: Copy + Add<T, Output=T>
{
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl<T> Sub for PointAny<T>
where
	T: Copy + Sub<T, Output=T>
{
	type Output = Self;

	fn sub(self, rhs: Self) -> Self {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
		}
	}
}

impl<T> Mul<T> for PointAny<T>
where
	T: Copy + Mul<T, Output=T>
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self {
		Self {
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





