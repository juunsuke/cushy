
use glfw::{WindowEvent, GlfwReceiver, Context, WindowHint};

use crate::gl;


#[derive(Copy, Clone, Debug)]
pub enum WindowMode {
	Window (u32, u32),
	BorderlessWindow (u32, u32),
	Fullscreen (u32, u32),
	FullWindow,
}


#[derive(Clone, Debug)]
pub enum Event {
	Close,
	Unknown (WindowEvent),
}

impl From<WindowEvent> for Event {
	fn from(value: WindowEvent) -> Self {
		// Convert a GLFW event to a Cushy event
	    match value {
			WindowEvent::Close => Event::Close,
			_ => Event::Unknown(value),
		}
	}
}


pub struct Window {
	events: GlfwReceiver<(f64, WindowEvent)>,
	window: glfw::PWindow,
	glfw: glfw::Glfw,
}

impl Window {

	pub fn new(mode: WindowMode, title: impl AsRef<str>) -> Result<Window, String> {
		// Init GLFW
		use glfw::fail_on_errors;
		let mut glfw = glfw::init(fail_on_errors!()).or(Err(String::from("Error initializing GLFW")))?;

		glfw.window_hint(WindowHint::ContextVersion(4, 6));
		glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
		glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

		// Create the window
		let (mut window, events) = match mode {
			WindowMode::Window (w, h) => glfw.create_window(w, h, title.as_ref(), glfw::WindowMode::Windowed),

			_ => todo!(),
		}.ok_or(String::from("Error creating a window"))?;

		window.make_current();
		window.set_all_polling(true);

		// Setup OpenGL
		gl::init(|s| glfw.get_proc_address_raw(s));

		let win = Window {
			events,
			window,
			glfw,
		};

		Ok(win)
	}

	pub fn swap(&mut self) {
		self.window.swap_buffers();
	}

	pub fn poll_glfw_events(&mut self) -> Vec<WindowEvent> {
		// Poll and collect events
		self.glfw.poll_events();
		let events = glfw::flush_messages(&self.events);
		events.into_iter()
			.map(|(_, m)| m)
			.collect()
	}

	pub fn poll_events(&mut self) -> Vec<Event> {
		// Poll and convert events
		self.poll_glfw_events()
			.into_iter()
			.map(Event::from)
			.collect()
	}
}


