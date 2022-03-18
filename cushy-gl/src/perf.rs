use std::time::Instant;


#[derive(Copy, Clone, Debug)]
pub struct Perf {
	// State
	last_tally: Instant,
	frames: u32,
	tallied: bool,

	// Last tally result
	fps: f32,

	// Requested max fps
	last_frame: Instant,
	max_fps: Option<u32>,
}

impl Perf {
	pub fn new() -> Perf {
		Perf {
			last_tally: Instant::now(),
			frames: 0,
			tallied: false,
			fps: 0.0,
			last_frame: Instant::now(),
			max_fps: Some(240),
		}
	}

	pub fn reset(&mut self) {
		// Reset the state
		self.last_tally = Instant::now();
		self.frames = 0;
		self.tallied = false;
	}

	pub fn pre_swap(&mut self) {
		// Called just before OpenGL swap buffers
		self.frames += 1;

		// Tally if a second has passed
		let now = Instant::now();
		let dur = now.duration_since(self.last_tally).as_millis();
		
		if dur>=1000 {
			// Tally
			self.fps = ((self.frames as f64) / ((dur as f64)/1000.0)) as f32;
			self.tallied = true;

			// Reset
			self.last_tally = now;
			self.frames = 0;
		}
		else {
			self.tallied = false;
		}

		// Check if a max fps is set
		if let Some(max) = self.max_fps {
			// Convert that into milliseconds
			let min = 1000000 / max;

			// Check the elapsed time
			let dur = now
				.duration_since(self.last_frame)
				.as_micros()
				as u32;

			if dur < min {
				let diff = min-dur;
				
				std::thread::sleep(std::time::Duration::from_micros(diff as u64));
			}
			
			self.last_frame = Instant::now();
		}
	}

	pub fn max_fps(&self) -> Option<u32> {
		self.max_fps
	}

	pub fn set_max_fps(&mut self, max: Option<u32>) {
		self.max_fps = max;
	}

	pub fn fps(&self) -> f32 {
		// Last tallied FPS
		self.fps
	}

	pub fn tallied(&self) -> bool {
		// Tallied last frame ?
		self.tallied
	}
}
