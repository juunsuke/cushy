

use cushy_gfx::*;



fn main() {

	let mut win = Window::default();
	win.set_vsync(VSync::Off);

	perf_test(
		&mut win,
		QuadRendererType::Cpu,		// Cpu or Gpu
		10000,						// Quad count
		100,						// Quad size
		true,						// Parallel
		false,						// Tex interleave
		false,						// Scattered
		true,						// Print info
	);

/*
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


	let mut qs = Vec::new();
	for _ in 0..100000 {
		let mut q = Quad::new();
		let x = rand::thread_rng().gen_range(100.0..800.0);
		let y = rand::thread_rng().gen_range(100.0..500.0);
		//q.set_pos(Point::new(x, y));
		q.set_texture(Some(&tex2));

		qs.push(q);
	}

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

		for q in qs.iter_mut() {
			//q.set_rot(Rotation::from_deg(a));
			qr.add(q);
		}

		a += 1.0;
		q.set_rot(Rotation::from_deg(a));


		let mut q2 = Quad::new();
		q2.set_pos(Point::new(5.0, 100.0));
		q2.set_texture(Some(&tex2));
		
		//qr.add(&q);
		//qr.add(&q2);
		qr.draw(&cam);


		win.swap_buffers();

		if win.perf().tallied() {
			println!("{} frames/second", win.perf().fps());
		}
	}
*/
}



