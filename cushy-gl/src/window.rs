
use glfw::{Glfw, Context, WindowEvent, WindowHint};

use crate::*;



//////////////////////////////////////////////////////////////////////////////////////////////////// WindowBorder

#[derive(Copy, Clone, Debug)]
pub enum WindowBorder {
	Fixed,
	Resizable,
	Borderless,
}


//////////////////////////////////////////////////////////////////////////////////////////////////// VideoMode

#[derive(Copy, Clone, Debug)]
pub enum VideoMode {
	Desktop,
	Windowed (u32, u32, WindowBorder),
}


//////////////////////////////////////////////////////////////////////////////////////////////////// Window

#[derive(Debug)]
pub struct Window {
	// GLFW stuff
	win: glfw::Window,
	events_receiver: std::sync::mpsc::Receiver<(f64, WindowEvent)>,
	glfw: Glfw,
}

impl Default for Window {
	fn default() -> Self {
		Self::new(&VideoMode::Windowed(1366, 768, WindowBorder::Resizable), "Cushy window")
	}
}

impl Window {

	pub fn new(vm: &VideoMode, caption: impl AsRef<str>) -> Self {
		// Create a basic default window
		let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
		
		// Hints
		glfw.window_hint(WindowHint::ContextVersion(3, 3));
		glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
		glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

		if let VideoMode::Windowed (_, _, WindowBorder::Fixed) = vm {
			glfw.window_hint(WindowHint::Resizable(false));
		}

		let (mut window, events_receiver) = glfw.create_window(
			1366,
			768,
			caption.as_ref(),
			glfw::WindowMode::Windowed,
		).expect("Error creating a window");

		window.make_current();
		window.set_key_polling(true);
		window.set_char_polling(true);
		window.set_cursor_pos_polling(true);
		window.set_mouse_button_polling(true);
		window.set_scroll_polling(true);
		window.set_size_polling(true);
		window.set_focus_polling(true);
		window.set_close_polling(true);


		// Setup the OpenGL load address
		api::load_with(|s| glfw.get_proc_address_raw(s));

		// Some very basic OpenGL initializations
		unsafe {
			use crate::api as gl;

			gl::Disable(gl::DEPTH_TEST);
			gl::Enable(gl::BLEND);
			gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
		}

		let mut win = Self {
			events_receiver,
			win: window,
			glfw,
		};

		// Fake a resize event to settle some things
		let (ww, wh) = win.win.get_size();
		win.on_resize(ww as u32, wh as u32);

		win
	}

	pub fn glfw_window(&self) -> &glfw::Window {
		&self.win
	}

	pub fn glfw_window_mut(&mut self) -> &mut glfw::Window {
		&mut self.win
	}

	fn collect_events(&mut self) -> Vec<Event> {
		// Collect all queued events
		self.glfw.poll_events();

		glfw::flush_messages(&self.events_receiver)
			.map(|(_, ev)| Event::from_glfw(&ev))
			.collect()
	}

	pub fn process_events(&mut self) -> Vec<Event> {
		// Collect queued events
		let mut events = self.collect_events();

		// Take a peek at the events, though most will fall through to the owner when they call get_event()
		events.retain(|ev| {
			match ev {
				// These events are processed, but still fall through in case the owner needs them
				Event::QuitRequest => self.win.set_should_close(true),
				Event::Resize (w, h) => self.on_resize(*w, *h),

				// An event we don't care about
				_ => (),
			}

			true
		});

		events
	}

	pub fn size(&self) -> (u32, u32) {
		let (w, h) = self.win.get_size();

		(w as u32, h as u32)
	}

	fn on_resize(&mut self, w:u32, h:u32) {
		// Set the viewport
		unsafe {
			api::Viewport(0, 0, w as i32, h as i32);
		}
	}

	pub fn must_quit(&self) -> bool {
		self.win.should_close()
	}

	pub fn swap_buffers(&mut self) {
		// Swap OpenGL buffers
		self.win.swap_buffers();
	}

	pub fn clear(&self, r:f32, g:f32, b:f32, a:f32) {
		// Clear the back buffer
		unsafe {
			api::ClearColor(r, g, b, a);
			api::Clear(api::COLOR_BUFFER_BIT);
		}
	}
}


