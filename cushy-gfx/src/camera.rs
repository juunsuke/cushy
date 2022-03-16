

use cgmath::Matrix4;

use cushy_gl::Event;
use crate::*;


//////////////////////////////////////////////////////////////////////////////////////////////////// StretchMode

#[derive(Copy, Clone, Debug)]
pub enum StretchMode {
	// Do not stretch, use the whole viewport with a 1:1 pixel ratio
	// The camera resolution will change to match the viewport
	None,

	// Stretch to completely fill the viewport, ignoring aspect ratio
	Fill,

	// Stretch to fill as much as the viewport as possible, while keeping a fixed aspect ratio
	// The output will be centered, with black bars added on the borders if needed
	KeepAspect (f32),
}

impl Default for StretchMode {
	fn default() -> Self {
		Self::None
	}
}



//////////////////////////////////////////////////////////////////////////////////////////////////// Camera

#[derive(Copy, Clone, Default, Debug)]
pub struct Camera {
	// Stretch mode
	stretch: StretchMode,

	// Camera transform, will be applied to all renderings
	transform: Transform,

	// Viewport size
	vp_size: SizeU32,

	// Cached projection matrix
	proj: Option<Matrix4<f32>>,
}

impl Camera {

	pub fn new() -> Self {
		// Default camera
		Self {
			stretch: StretchMode::None,
			transform: Transform::default(),
			vp_size: SizeU32::new(480, 270),
			proj: None,
		}
	}

	pub fn stretch_mode(&self) -> &StretchMode					{ &self.stretch }
	pub fn transform(&self) -> &Transform						{ &self.transform }
	pub fn vp_size(&self) -> SizeU32							{ self.vp_size }

	pub fn set_stretch_mode(&mut self, sm: StretchMode) {
		self.stretch = sm;
		self.calc_proj_matrix();
	}

	pub fn set_transform(&mut self, t: Transform) {
		self.transform = t;
		
		// TODO: This ain't right
		self.calc_proj_matrix();
	}

	pub fn set_vp_size(&mut self, vp: SizeU32) {
		self.vp_size = vp;
		self.calc_proj_matrix();
	}

	pub fn resize_event(&mut self, ev: &Event) {
		// Fix the viewport size if this is a resize event from the windowing system
		if let Event::Resize (w, h) = *ev {
			self.set_vp_size(SizeU32::new(w, h));
		}
	}

	fn calc_proj_matrix(&mut self) {
		// Calc the projection matrix
		let w = self.vp_size.w as f32;
		let h = self.vp_size.h as f32;
		self.proj = Some(cgmath::ortho(0.0, w, h, 0.0, 1.0, -1.0));
	}

	pub fn calc_matrix(&mut self) {
		// Calc the matrices if needed
		if self.proj.is_none() {
			self.calc_proj_matrix();
		}
	}

	pub fn proj_matrix(&self) -> &Matrix4<f32> {
		self.proj
			.as_ref()
			.expect("The Camera's projection matrix needs to be calculated before being used")
	}
}


