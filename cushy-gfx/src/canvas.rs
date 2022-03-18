
use crate::*;


/// A buffer containing an image
#[derive(Clone, Debug)]
pub struct Canvas {
	// Canvas size
	size: SizeU32,

	// Raw data
	data: Vec<u32>,

	// Dirty flag
	dirty: bool,
}

impl Canvas {
	/// Create a new empty canvas
	///
	/// If no clear color is provided (set to [`None`]), the contents of the canvas will be undefined.
	pub fn new(size: SizeU32, clear_col: Option<Color>) -> Canvas {
		let len = (size.w*size.h) as usize;

		let mut cnv = Canvas {
			size,
			data: Vec::with_capacity(len),
			dirty: true,
		};

		// Set the proper size and clear
		if let Some(col) = clear_col {
			cnv.data.resize(len, col.as_u32());
		}
		else {
			// Safety: Only contains u32's, and the capacity was already properly set
			unsafe {
				cnv.data.set_len(len);
			}
		}

		cnv
	}

	/// Load a canvas from a PNG file
	///
	/// An error is returned if the file cannot be read, or if it is not a proper PNG file.
	/// Loading of the PNG file is done through the [`lodepng`] crate.
	pub fn from_file<P:AsRef<std::path::Path>>(path: P) -> Result<Canvas, String> {
		// Try to load the file
		let img = lodepng::decode32_file(path).or_else(|e| Err(e.to_string()))?;

		Ok(Canvas::from_lodepng(img))
	}

	/// Load a canvas from a PNG file already loaded in memory
	///
	/// This is useful for incorporating small images directly into the application binary, without
	/// requiring the file to be present at runtime.
	/// ```
	/// let canvas = Canvas::from_memory(include_bytes!("some_file.png"));
	/// ```
	pub fn from_memory<T>(data: T) -> Result<Canvas, String>
	where
		T:AsRef<[u8]>
	{
		// Try to load the memory file
		let img = lodepng::decode32(data).or_else(|e| Err(e.to_string()))?;
		
		Ok(Canvas::from_lodepng(img))
	}

	fn from_lodepng(img: lodepng::Bitmap<lodepng::RGBA>) -> Canvas {
		// Convert a lodepng image into a Canvas
		let w = img.width as u32;
		let h = img.height as u32;
		let mut cnv = Canvas::new(SizeU32::new(w, h), None);

		// Copy the image data
		let sptr = img.buffer.as_ptr() as *const u32;
		let dptr = cnv.data.as_mut_ptr();

		// Safety: We are certain of the size of both buffers
		//         Both buffers are different and guaranteed to not overlap
		unsafe {
			std::ptr::copy_nonoverlapping(sptr, dptr, img.width*img.height);
		}

		cnv
	}

/*
	/// Create a new canvas containing rendered text
	///
	/// The canvas will have a transparant background (all RGBA set to 0), and the drawn text will
	/// have a fixed RGB (taken from the col argument), with the A value varying to represent the letters.
	/// Such a canvas is suitable to be used directly as a texture, or as a source for a blending blit
	/// onto another canvas.
	///
	/// Because of the way [`Fonts`](Font) are handled, the canvas might be slightly larger than the actual
	/// text, especially on the top and bottom.
	pub fn from_text(fnt: &Font, size: u32, s: &str, col: Color) -> Canvas {
		// Calc the required width
		let w = fnt.text_width(s, size);
		let h = size;

		// Make an empty canvas and write the text on it
		let mut cnv = Canvas::new(w, h, Some(Color(0)));
		cnv.draw_text_noblend(fnt, size, 0, 0, s, col);

		cnv
	}
*/

	/// Get the size of the buffer
	pub fn size(&self) -> SizeU32 {
		self.size
	}

	/// Get a shared reference to the data buffer
	///
	/// The data is laid out as consecutive sets of 4 bytes, which can be manipulated directly
	/// using the [`Color`] struct.  The index of a specific pixel can be calculated easily:
	/// ```
	/// let index = (y * width) + x;
	/// ```
	pub fn data(&self) -> &Vec<u32> {
		&self.data
	}

	/// Returns the current value of the dirty flag
	///
	/// The dirty flag can be set manually, but is usally set automatically by functions that
	/// modify the canvas data.
	pub fn dirty(&self) -> bool {
		self.dirty
	}

	/// Change the current value of the dirty flag
	///
	/// There is usually no point in changing this manually.
	pub fn set_dirty(&mut self, dirty: bool) {
		self.dirty = dirty;
	}

	/// Clear the whole canvas to the provided color
	pub fn clear(&mut self, col: Color) {
		// Clear the canvas with a given color
		self.data
			.iter_mut()
			.for_each(|p| *p = col.0);

		self.dirty = true;
	}

	fn pos_to_index(&self, p: PointU32) -> usize {
		(p.y*self.size.w + p.x) as usize
	}

	fn pos_valid(&self, p: PointU32) -> bool {
		p.x < self.size.w
		&& p.y < self.size.h
	}

	/// Return the color of the pixel at the given coordinates
	///
	/// Instead of panicking, the method will return [`None`] if the coordinates are out of bounds.
	pub fn get_pixel(&mut self, p: PointU32) -> Option<Color> {
		// Change a single pixel
		if self.pos_valid(p) {
			let pos = self.pos_to_index(p);
			Some(Color(self.data[pos]))
		}
		else {
			None
		}
	}

	/// Set the color of the pixel at the given coordinates
	///
	/// This method will never panic.  If the coordinates are out of bounds, it will silently fail.
	pub fn set_pixel(&mut self, p: PointU32, col: Color) {
		// Change a single pixel
		if self.pos_valid(p) {
			let pos = self.pos_to_index(p);
			self.data[pos] = col.0;
			self.dirty = true;
		}
	}

	/// Draw an horizontal line at the given coordinates
	///
	/// If all or part of the line are out of bounds, it will be clipped.  This function never panics.
	pub fn hline(&mut self, p: PointU32, mut w: u32, col: Color) {
		// Clip
		if p.x>=self.size.w || p.y>=self.size.h {
			return;
		}

		if (p.x+w) > self.size.w {
			w = self.size.w - p.x;
		}

		// Draw the line
		let pos = self.pos_to_index(p);

		self.data[pos..(pos+w as usize)]
			.iter_mut()
			.for_each(|p| *p = col.0);
	}

	pub fn vline(&mut self, p: PointU32, mut h: u32, col: Color) {
		// Clip
		if p.x>=self.size.w || p.y>=self.size.h {
			return;
		}

		if (p.y+h) > self.size.h {
			h = self.size.h - p.y;
		}

		// Draw the line
		let mut pos = self.pos_to_index(p);

		for _ in 0..h {
			self.data[pos] = col.0;
			pos += self.size.w as usize;
		}
	}

	pub fn rect(&mut self, r: RectU32, col: Color) {
		if r.w==0 || r.h==0 {
			return;
		}

		let tl = r.pos();
		let tr = PointU32::new(r.x+r.w-1, r.y);
		let bl = PointU32::new(r.x, r.y+r.h-1);

		self.hline(tl, r.w, col);
		self.hline(bl, r.w, col);
		self.vline(tl, r.h, col);
		self.vline(tr, r.h, col);
	}

	pub fn rect_fill(&mut self, r: RectU32, col: Color) {
		if r.w==0 || r.h==0 {
			return;
		}
		
		let mut w = r.w;
		let mut h = r.h;

		if r.x+w > self.size.w {
			w = self.size.w - r.x;
		}
		if r.y+h > self.size.h {
			h = self.size.h - r.y;
		}

		for y in r.y..(r.y+h) {
			self.hline(PointU32::new(r.x, y), w, col);
		}
	}
	
/*
	/// Render text on the canvas
	///
	/// The text is blended with the current pixel data using a source-alpha blending.  For a non-blending
	/// version, see [`draw_text_noblend`](Self::draw_text_noblend).
	///
	/// This method never panics.  Drawing out of bounds is simply ignored.
	///
	/// The A component of the provided color is ignored.  It is substituted with the "strength" of
	/// the pixels as provided by the font drawing algorithm.  The latter is then used to weigh
	/// the percentage of the provided color to use, against the percentage of the background pixel color.
	/// The A component of the previous pixel is always maintained, so overall transparancy will not be affected.
	///
	/// For instance, if the font algorithm for a certain pixel returns a value of 0.75, then 75% of
	/// the provided color will be blended with 25% of the background pixel at that location.
	/// The rendering algorithm looks something like this:
	/// ```
	/// // Source color
	/// let (sr, sg, sb, sa) = col.as_f32();
	/// // Destination color
	/// let (dr, dg, db, da) = self.get_pixel(x, y).unwrap().as_f32();
	/// // Weight of the pixel, as provided by the font algorithm
	/// let w: f32 = 0.75;  // Hard-coded to 0.75 for now, but can be anything from 0.0 to 1.0
	///
	/// // Weigh each color component
	/// let r = (sr*w) + (dr*(1.0-w));
	/// let g = (sg*w) + (dg*(1.0-w));
	/// let b = (sb*w) + (db*(1.0-w));
	///
	/// // Build the final color
	/// let value = Color::from_f32(r, g, b, da);
	/// ```
	/// This version is most suitable for building a canvas with a background and text drawn onto it,
	/// such as for user interfaces.
	pub fn draw_text(&mut self, fnt: &Font, size: u32, x: i32, y: i32, s: &str, col: Color) {
		// Draw a string using the supplied font
		fnt.draw(s, size, |fx, fy, fv| {
			if fv>0.0 {
				if let Some(dst) = self.get_pixel(x+fx, y+fy) {
					self.set_pixel(x+fx, y+fy, Color::blend_srcalpha(col.with_alpha(fv), dst));
				}
			}
		});

		self.dirty = true;
	}

	/// Render text on the canvas
	///
	/// The text is not blended and the the current pixel is ignored.  For a blending version, see
	/// [`draw_text`](Self::draw_text).
	///
	/// This method never panics.  Drawing out of bounds is simply ignored.
	///
	/// The A component of the provided color is ignored.  It is substituted with the "strength" of
	/// the pixels as provided by the font drawing algorithm.  This will create a partially transparant
	/// area where the letters are drawn.
	///
	/// This version is most suitable for being applied to textures that will be directly rendered.
	pub fn draw_text_noblend(&mut self, fnt: &Font, size: u32, x: i32, y: i32, s: &str, col: Color) {
		// Draw a string using the supplied font
		fnt.draw(s, size, |fx, fy, fv| {
			if fv>0.0 {
				self.set_pixel(x+fx, y+fy, col.with_alpha(fv));
			}
		});

		self.dirty = true;
	}
*/

}
