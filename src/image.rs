use std::fmt;
use std::f32;

use nalgebra::{Vec2, Vec3};

use object::barycentric;

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

	pub fn triangle(&mut self, t: &mut Vec<Vec3<f32>>, zbuf: &mut Vec<f32>, color: &Color) {
		let mut bbox_min = Vec2::new(f32::INFINITY, f32::INFINITY);
		let mut bbox_max = Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
		let clamp = Vec2::new((self.width as f32) - 1.0, (self.height as f32) - 1.0);

		for i in 0..3 {
			for j in 0..2 {
				bbox_min[j] = (0.0 as f32).max(bbox_min[j].min(t[i][j]));
				bbox_max[j] = clamp[j].min(bbox_max[j].max(t[i][j]));
			}
		}

		for x in (bbox_min.x as i32)..(bbox_max.x as i32) {
			for y in (bbox_min.y as i32)..(bbox_max.y as i32) {
				let mut p = Vec3::new(x as f32, y as f32, 0.0);
				let bc_screen = barycentric(&t[0], &t[1], &t[2], &p);
				if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 { continue; }

				for i in 0..3 { p.z += t[i][2] * bc_screen[i]; }
				if zbuf[self.trans(p.x as u32, p.y as u32)] < p.z {
					zbuf[self.trans(p.x as u32, p.y as u32)] = p.z;
					self.plot(p.x as u32, p.y as u32, color.clone());
				}
			}
		}
	}
}
