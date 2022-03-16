
use glfw::{Action, Key, WindowEvent};

//////////////////////////////////////////////////////////////////////////////////////////////////// Event

#[derive(Clone, Debug)]
pub enum Event {
	QuitRequest,
	Resize (u32, u32),

	UnknownGlfw (WindowEvent),
}

impl Event {

	pub fn from_glfw(ev: &WindowEvent) -> Self {
		match ev {
			WindowEvent::Size (w, h) => Event::Resize(*w as u32, *h as u32),

			WindowEvent::Key (Key::Escape, _, Action::Press, _) => Event::QuitRequest,

			_ => Event::UnknownGlfw (ev.clone()),
		}
	}

	pub fn dispatch(self, h: &mut impl EventHandler) {
		// Dispatch the event to the proper handler function
		match self {
			Event::QuitRequest => h.on_quit_request(),
			Event::Resize (ww, wh) => h.on_resize(ww, wh),

			Event::UnknownGlfw (we) => h.on_unknown_glfw_event(we),
		}
	}
}

//////////////////////////////////////////////////////////////////////////////////////////////////// EventHandler

pub trait EventHandler {
	
	fn on_quit_request(&mut self) {}
	fn on_resize(&mut self, _ww:u32, _wh:u32) {}

	fn on_unknown_glfw_event(&mut self, _we: WindowEvent) {}
}



