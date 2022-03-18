
use rand::{thread_rng, Rng};

use crate::*;


////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn perf_test(
	win: &mut Window,
	qrt: QuadRendererType,
	quad_count: u32,
	quad_size: u32,
	parallel: bool,
	tex_interleave: bool,
	scattered: bool,
	print_info: bool,
) -> Option<f32> {
	// Print info
	if print_info {
		println!("Testing {} quads of {}x{} pixels:", quad_count, quad_size, quad_size);

		match qrt {
			QuadRendererType::Cpu => println!("   - CPU matrix calculations"),
			QuadRendererType::Gpu => println!("   - GPU matrix calculations"),
		}

		if parallel {
			println!("   - Using parallel processing");
		}
		else {
			println!("   - NOT using parallel processing");
		}

		if tex_interleave {
			println!("   - Textures are interleaved in a worst case scenario");
		}
		else {
			println!("   - Same texture for all quads, single draw call");
		}

		if scattered {
			println!("   - Scattered quads");
		}
		else {
			println!("   - Piled up quads");
		}
	}

	// Create a camera
	let mut cam = Camera::new();
	let (ww, wh) = win.size();
	cam.set_vp_size(SizeU32::new(ww, wh));

	// Create the renderer
	let mut qr = QuadRenderer::new(qrt);
	qr.set_parallel(parallel);

	// Create a simple canvas
	let mut cnv = Canvas::new(SizeU32::new(quad_size, quad_size), Some((0.8, 0.4, 0.2).into()));

	{
		let u = quad_size/5;
		cnv.rect_fill(RectU32::new(u, u, u*3, u*3), (0.4, 0.8, 0.2).into());
		cnv.rect_fill(RectU32::new(u*2, u*2, u, u), (0.2, 0.4, 0.8).into());
	}

	// Make a texture out of it
	let tex1 = Texture::from(&cnv);

	// Second texture will be the same or a new one for interleave mode
	let tex2 = if tex_interleave {
		Texture::from(&cnv)
	}
	else {
		tex1.clone()
	};

	let maxx = (ww-quad_size) as f32;
	let maxy = (wh-quad_size) as f32;

	let mut qs = Vec::new();
	for i in 0..quad_count {
		let mut q = Quad::new();

		if scattered {
			let x = thread_rng().gen_range(0.0 .. maxx);
			let y = thread_rng().gen_range(0.0 .. maxy);
			q.set_pos(Point::new(x, y));
		}

		if (i%2) == 0 {
			q.set_texture(Some(&tex1));
		} else {
			q.set_texture(Some(&tex2));
		}

		qs.push(q);
	}

	// Run the loop until aborted or ran for 5 seconds
	let mut fps = Vec::new();

	while fps.len()<5 {
		// Process events
		win.process_events();
		if win.must_quit() {
			return None;
		}

		// Clear
		win.clear(0.2, 0.3, 0.4, 1.0);

		// Render the quads
		for q in qs.iter_mut() {
			qr.add(q);
		}

		qr.draw(&cam);

		// Swap
		win.swap_buffers();

		// Check the tally
		if win.perf().tallied() {
			fps.push(win.perf().fps());
		}
	}

	// Calc the average of last 3 seconds
	let res = (fps[2]+fps[3]+fps[4])/3.0;
	
	if print_info {
		println!("   * {} frames/second average over 3 seconds", res);
	}

	Some(res)
}



