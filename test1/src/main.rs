
#![allow(dead_code)]

use cushy_gfx::*;


fn perf(win: &mut Window) {
	perf_test(
		win,
		QuadRendererType::Gpu,		// Cpu or Gpu
		10000,						// Quad count
		100,						// Quad size
		false,						// Parallel
		false,						// Tex interleave
		true,						// Scattered
		true,						// Print info
	);

}


fn main() {

	let mut win = Window::default();
	win.set_vsync(VSync::Off);
	//win.perf_mut().set_max_fps(None);

	//perf(&mut win);


	let mut cam = Camera::new();
	let (ww, wh) = win.size();
	cam.set_vp_size(SizeU32::new(ww, wh));
	let mut qr = QuadRenderer::new(QuadRendererType::Cpu);


	let cnv = Canvas::from_file("kanade.png").unwrap();

	let tex = Texture::from(&cnv);
	
	//let tex2 = tex.sub(&RectU32::new(185, 185, 210, 115));
	let tex2 = Texture::from(Canvas::from_file("kanade2.png").unwrap());

	let tex_size = Size::from(tex.size());

	let mut q = Quad::new();

	let (w, h) = win.size();
	let wsize = Size::new(w as f32, h as f32);

	q.set_pos((wsize/2.0).into());
	//q.set_color(Color::from((1.0, 0.3, 0.6)));
	q.set_origin((tex_size/2.0).into());
	//q.set_scale(Scaling::new(0.5, 1.0));
	q.set_texture(Some(&tex));
	//q.set_size(Size::new(200.0, 50.0));

	let mut a = 0.0;


	loop {
		for ev in win.process_events() {
			match ev {
				Event::WindowResize (w, h) => cam.set_vp_size(SizeU32::new(w, h)),

				Event::KeyDown (Key::Escape, _) => win.set_must_quit(true),

				_ => println!("{:?}", ev),
			}
		}

		if win.must_quit() {
			break;
		}

		win.clear(0.2, 0.3, 0.4, 1.0);

		a += 1.0;
		q.set_rot(Rotation::from_deg(a));


		let mut q2 = Quad::new();
		q2.set_pos(Point::new(5.0, 100.0));
		q2.set_texture(Some(&tex2));
		
		qr.add(&q);
		qr.add(&q2);
		qr.draw(&cam);


		win.swap_buffers();

		if win.perf().tallied() {
			println!("{} frames/second", win.perf().fps());
		}
	}

}



