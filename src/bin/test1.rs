
use cushy::*;


fn main() {

	let mut win = Window::new(WindowMode::Window(1024, 768), "Test").unwrap();

	'main: loop {
		gl::clear(Color::from_f32(0.2, 0.4, 1.0, 1.0));
	
		win.swap();

		let events = win.poll_events();
		for ev in events {
			match ev {
				Event::Close => break 'main,

				_ => (),
			}
		}
	}
}



