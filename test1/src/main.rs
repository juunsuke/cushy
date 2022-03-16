

use cushy_gl::*;
use cushy_gfx::*;


fn main() {

	let mut win = Window::default();
	let mut cam = Camera::new();
	let mut qr = QuadRenderer::new();


	let cnv = Canvas::from_file("kanade.png").unwrap();

	let tex = Texture::from(&cnv);
	let tex_size = Size::from(tex.size());

	let mut q = Quad::new();

	let (w, h) = win.size();
	let wsize = Size::new(w as f32, h as f32);

	q.set_pos((wsize/2.0).into());
	//q.set_color(Color::from((1.0, 0.3, 0.6)));
	q.set_origin((tex_size/2.0).into());
	//q.set_scale(Scaling::new(1.0, 6.0));
	q.set_texture(Some(&tex));
	//q.set_size(Size::new(200.0, 50.0));

	let mut a = 0.0;

	loop {
		for ev in win.process_events() {
			match ev {
				Event::Resize (w, h) => cam.set_vp_size(SizeU32::new(w, h)),

				_ => (),
			}
		}

		if win.must_quit() {
			break;
		}

		win.clear(0.2, 0.3, 0.4, 1.0);


		a += 1.0;
		q.set_rot(Rotation::from_deg(a));
		
		qr.add(&mut q);
		qr.draw(&cam);


		win.swap_buffers();
	}
}



