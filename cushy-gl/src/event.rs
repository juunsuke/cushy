
use glfw::{Action, WindowEvent};

use crate::{Key, Modifiers};



//////////////////////////////////////////////////////////////////////////////////////////////////// Event

#[derive(Clone, Debug)]
pub enum Event {
	Char (char),
	KeyDown (Key, Modifiers),
	KeyUp (Key),

	MouseMove (i32, i32),
	MouseDown (u32),
	MouseUp (u32),
	MouseWheelUp,
	MouseWheelDown,

	WindowResize (u32, u32),
	WindowFocus (bool),
	QuitRequest,

	UnknownGlfw (WindowEvent),
}

impl Event {

	pub fn from_glfw(ev: &WindowEvent) -> Option<Self> {
		match *ev {
			WindowEvent::Key (k, _, a, m) => Event::key_event(k, a, m),
			WindowEvent::Char (ch) => Some(Event::Char(ch)),

			WindowEvent::CursorPos (x, y) => Some(Event::MouseMove(x as i32, y as i32)),
			WindowEvent::MouseButton (b, a, _) => Event::mouse_button_event(b, a),
			WindowEvent::Scroll (_, y) => Event::mouse_wheel_event(y),

			WindowEvent::Size (w, h) => Some(Event::WindowResize(w as u32, h as u32)),
			WindowEvent::Focus (f) => Some(Event::WindowFocus(f)),

			_ => Some(Event::UnknownGlfw (ev.clone())),
		}
	}

	fn key_event(k: glfw::Key, a: Action, m: glfw::Modifiers) -> Option<Event> {
		// Ignore the unknown key
		if let glfw::Key::Unknown = k {
			return None;
		}

		// Convert the key and modifiers
		let k = Key::from_glfw(k);
		let m = Modifiers::from_glfw(m);

		match a {
			Action::Press => Some(Event::KeyDown(k, m)),
			Action::Release => Some(Event::KeyUp(k)),
			Action::Repeat => Some(Event::KeyDown(k, m)),
		}
	}

	fn mouse_button_event(b: glfw::MouseButton, a: Action) -> Option<Event>{
		// Integer mouse buttons make more sense
		let b = b as u32;

		match a {
			Action::Press => Some(Event::MouseDown(b)),
			Action::Release => Some(Event::MouseUp(b)),
			Action::Repeat => None,
		}
	}

	fn mouse_wheel_event(y: f64) -> Option<Event> {
		if y>0.5 {
			Some(Event::MouseWheelUp)
		}
		else if y< -0.5 {
			Some(Event::MouseWheelDown)
		}
		else {
			None
		}
	}

	pub fn dispatch(self, h: &mut impl EventHandler) {
		// Dispatch the event to the proper handler function
		match self {
			Event::Char (ch) => h.on_char(ch),
			Event::KeyDown (k, m) => h.on_key_down(k, m),
			Event::KeyUp (k) => h.on_key_up(k),

			Event::MouseMove (x, y) => h.on_mouse_move(x, y),
			Event::MouseDown (b) => h.on_mouse_down(b),
			Event::MouseUp (b) => h.on_mouse_up(b),
			Event::MouseWheelDown => h.on_mouse_wheel_down(),
			Event::MouseWheelUp => h.on_mouse_wheel_up(),

			Event::WindowResize (ww, wh) => h.on_window_resize(ww, wh),
			Event::WindowFocus (f) => h.on_window_focus(f),
			Event::QuitRequest => h.on_quit_request(),

			Event::UnknownGlfw (we) => h.on_unknown_glfw_event(we),
		}
	}
}

//////////////////////////////////////////////////////////////////////////////////////////////////// EventHandler

pub trait EventHandler {
	fn on_char(&mut self, _ch:char) {}
	fn on_key_down(&mut self, _k:Key, _m:Modifiers) {}
	fn on_key_up(&mut self, _k:Key) {}

	fn on_mouse_move(&mut self, _x:i32, _y:i32) {}
	fn on_mouse_down(&mut self, _b:u32) {}
	fn on_mouse_up(&mut self, _b:u32) {}
	fn on_mouse_wheel_down(&mut self) {}
	fn on_mouse_wheel_up(&mut self) {}

	fn on_window_resize(&mut self, _ww:u32, _wh:u32) {}
	fn on_window_focus(&mut self, _f:bool) {}
	fn on_quit_request(&mut self) {}

	fn on_unknown_glfw_event(&mut self, _we: WindowEvent) {}
}



