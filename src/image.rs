use std::fmt;

#[derive(Clone, RustcEncodable)]
pub struct Color(pub u32);

impl fmt::Debug for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "#{:08X}", self.0)
	}
}

pub struct Image {
	pub width: u32,
	pub height: u32,
	pub data: Vec<Color>,
}

impl Image {
	pub fn new(width: u32, height: u32) -> Image {
		let mut data = Vec::with_capacity((width * height) as usize);

		for _ in 0..data.capacity() {
			data.push(Color(0x000000FF));
		}

		Image {
			width: width,
			height: height,
			data: data,
		}
	}

	fn trans(&self, x: u32, y: u32) -> usize {
		((self.width * y) as usize) + (x as usize)
	}

	pub fn plot(&mut self, x: u32, y: u32, color: Color) {
		if x >= self.width || y >= self.height {
			panic!("Point: ({},{}) outside of bounds!", x, y);
		} else {
			let idx = self.trans(x, y);
			self.data[idx] = color;
		}
	}

	pub fn line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, color: &Color) {
		self.plot(x0, y0, color.clone());
		self.plot(x1, y1, color.clone());
	}
}
