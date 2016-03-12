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
		let mut steep = false;
		let mut t_x0 = x0 as i32;
		let mut t_x1 = x1 as i32;
		let mut t_y0 = y0 as i32;
		let mut t_y1 = y1 as i32;

		if (t_x0 - t_x1).abs() < (t_y0 - t_y1).abs() {
			let mut tmp = t_x0;
			t_x0 = t_y0;
			t_y0 = tmp;

			tmp = t_x1;
			t_x1 = t_y1;
			t_y1 = tmp;
			steep = true;
		}

		if t_x0 > t_x1 {
			let mut tmp = t_x0;
			t_x0 = t_x1;
			t_x1 = tmp;

			tmp = t_y0;
			t_y0 = t_y1;
			t_y1 = tmp;
		}

		for x in t_x0..t_x1 {
			let t = ((x - t_x0) as f32) / ((t_x1 - t_x0) as f32);
			let y = (((t_y0 as f32) * (1.0 - t)) + ((t_y1 as f32) * t)) as u32;
			if steep {
				self.plot(y, x as u32, color.clone());
			} else {
				self.plot(x as u32, y, color.clone());
			}
		}
	}
}
