use std::fmt;

use object::{Triangle, Vec2};

#[derive(Clone, RustcEncodable)]
pub struct Color(pub u32);

impl fmt::Debug for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "#{:08X}", self.0)
	}
}

impl Color {
	pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
		let color = ((r as u32) << 24) + ((g as u32) << 16) + ((b as u32) << 8) + (a as u32);
		Color(color)
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
			return;
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

	pub fn triangle(&mut self, t: &mut Triangle<f32>, color: &Color) {
		if (t.p0.y == t.p1.y) && (t.p0.y == t.p2.y) { return; }

		let mut tri = Triangle::new(Vec2::new(((t.p0.x as i32), (t.p0.y as i32))), Vec2::new(((t.p1.x as i32), (t.p1.y as i32))), Vec2::new(((t.p2.x as i32), (t.p2.y as i32))));

		if tri.p0.y > tri.p1.y {
			let tmp = tri.p0.clone();
			tri.p0 = tri.p1.clone();
			tri.p1 = tmp;
		}
		if tri.p0.y > tri.p2.y {
			let tmp = tri.p0.clone();
			tri.p0 = tri.p2.clone();
			tri.p2 = tmp;
		}
		if tri.p1.y > tri.p2.y {
			let tmp = tri.p2.clone();
			tri.p2 = tri.p1.clone();
			tri.p1 = tmp;
		}

		let total_height = tri.p2.y - tri.p0.y;
		for i in 0..(total_height as u32) {
			let second_half = ((i as i32) > (tri.p1.y - tri.p0.y)) || (tri.p1.y == tri.p0.y);
			let seg_height;
			if second_half {
				seg_height = tri.p2.y - tri.p1.y;
			} else {
				seg_height = tri.p1.y - tri.p0.y;
			}

			let alpha = (i as i32) / (total_height as i32);
			let beta;
			if second_half {
				beta = (((i as i32) - (tri.p1.y - tri.p0.y)) as i32) / (seg_height as i32);
			} else {
				beta = ((i as i32) - 0) / (seg_height as i32);
			}

			let mut a = ((tri.p2.clone() - tri.p0.clone()) * Vec2::new((alpha, alpha))) + tri.p0.clone();
			let mut b;
			if second_half {
				b = ((tri.p2.clone() - tri.p1.clone()) * Vec2::new((beta, beta))) + tri.p1.clone();
			} else {
				b = ((tri.p1.clone() - tri.p0.clone()) * Vec2::new((beta, beta))) + tri.p0.clone();
			}

			if a.x > b.x {
				let tmp = b.clone();
				b = a.clone();
				a = tmp;
			}

			for j in (a.x as u32)..(b.x as u32) {
				self.plot(j, (tri.p0.y + (i as i32)) as u32, color.clone());
			}
		}
	}
}
