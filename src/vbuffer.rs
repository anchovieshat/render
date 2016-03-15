use std::fmt;
use std::mem;

use nalgebra::{Vec2, Vec3};
use nalgebra::cast;

use tga::TGA;

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

	pub fn scale_u8(base: u8, scalar: f32) -> Color {
		Color::new(((base as f32) * scalar) as u8, ((base as f32) * scalar) as u8, ((base as f32) * scalar) as u8, 0xFF)
	}

	pub fn scale_u32(base: u32, scalar: f32) -> Color {
		Color::new((((base >> 24) as f32) * scalar) as u8, ((((base << 8) >> 24) as f32) * scalar) as u8, ((((base << 16) >> 24) as f32) * scalar) as u8, 0xFF)
	}

	pub fn get(&self, idx: usize) -> Option<u8> {
		match idx {
			0 => Some(((self.0 << 24) >> 24) as u8),
			1 => Some(((self.0 << 16) >> 24) as u8),
			2 => Some(((self.0 << 8) >> 24) as u8),
			3 => Some(((self.0 << 0) >> 24) as u8),
			_ => None,
		}
	}
}

#[derive(Clone)]
pub struct VBuffer {
	pub width: u32,
	pub height: u32,
	pub data: Vec<Color>,
}

impl VBuffer {
	pub fn new(width: u32, height: u32) -> VBuffer {
		let mut data = Vec::with_capacity((width * height) as usize);

		for _ in 0..data.capacity() {
			data.push(Color(0x000000FF));
		}

		VBuffer {
			width: width,
			height: height,
			data: data,
		}
	}

	pub fn load(filename: &str) -> Option<VBuffer> {
		let file = TGA::load(filename);
		if file.is_some() {
			let file = file.unwrap();
			Some(VBuffer {
				width: file.head.width as u32,
				height: file.head.height as u32,
				data: file.data,
			})
		} else {
			None
		}
	}

	pub fn trans(&self, x: u32, y: u32) -> usize {
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

	pub fn triangle(&mut self, t: &mut Vec<Vec3<i32>>, uv: &mut Vec<Vec2<i32>>, zbuf: &mut Vec<i32>, intensity: f32, texture: &Option<VBuffer>) {
		if (t[0].y == t[1].y) && (t[0].y == t[2].y) { return; }

		if t[0].y > t[1].y { t.swap(0, 1); uv.swap(0, 1); }
		if t[0].y > t[2].y { t.swap(0, 2); uv.swap(0, 2); }
		if t[1].y > t[2].y { t.swap(1, 2); uv.swap(1, 2); }

		let total_height = t[2].y - t[0].y;
		for i in 0..total_height {
			let sec_half = (i > (t[1].y - t[0].y)) || (t[1].y == t[0].y);
			let seg_height;
			if sec_half {
				seg_height = t[2].y - t[1].y;
			} else {
				seg_height = t[1].y - t[0].y;
			}

			let alpha = (i as f32) / (total_height as f32);
			let beta;
			if sec_half {
				beta = ((i - (t[1].y - t[0].y)) as f32) / (seg_height as f32);
			} else {
				beta = ((i - 0) as f32) / (seg_height as f32);
			}

			let mut a;
			{ // Reduce namespace clutter
				// Cast i32 Vec3 to f32 Vec3, perform operations
				let af = cast::<Vec3<i32>, Vec3<f32>>(t[2] - t[0]);
				let t_af2 = cast::<Vec3<i32>, Vec3<f32>>(t[0]);
				let t_b = (af * alpha) + t_af2;
				a = cast::<Vec3<f32>, Vec3<i32>>(t_b);
			}
			let mut b;
			if sec_half {
				// Cast i32 Vec3 to f32 Vec3, perform operations
				let af = cast::<Vec3<i32>, Vec3<f32>>(t[2] - t[1]);
				let t_af2 = cast::<Vec3<i32>, Vec3<f32>>(t[1]);
				let t_b = (af * beta) + t_af2;
				b = cast::<Vec3<f32>, Vec3<i32>>(t_b);
			} else {
				// Cast i32 Vec3 to f32 Vec3, perform operations
				let af = cast::<Vec3<i32>, Vec3<f32>>(t[1] - t[0]);
				let t_af2 = cast::<Vec3<i32>, Vec3<f32>>(t[0]);
				let t_b = (af * beta) + t_af2;
				b = cast::<Vec3<f32>, Vec3<i32>>(t_b);
			}

			let mut uv_a;
			{	//((uv[2] - uv[0]) * (alpha)) + uv[0];
				let af = cast::<Vec2<i32>, Vec2<f32>>(uv[2] - uv[0]);
				let t_af2 = cast::<Vec2<i32>, Vec2<f32>>(uv[0]);
				let t_b = (af * alpha) + t_af2;
				uv_a = cast::<Vec2<f32>, Vec2<i32>>(t_b);
			}

			let mut uv_b;
			if sec_half {
				// Cast i32 Vec2 to f32 Vec2, perform operations
				let af = cast::<Vec2<i32>, Vec2<f32>>(uv[2] - uv[1]);
				let t_af2 = cast::<Vec2<i32>, Vec2<f32>>(uv[1]);
				let t_b = (af * beta) + t_af2;
				uv_b = cast::<Vec2<f32>, Vec2<i32>>(t_b);
			} else {
				// Cast i32 Vec2 to f32 Vec2, perform operations
				let af = cast::<Vec2<i32>, Vec2<f32>>(uv[1] - uv[0]);
				let t_af2 = cast::<Vec2<i32>, Vec2<f32>>(uv[0]);
				let t_b = (af * beta) + t_af2;
				uv_b = cast::<Vec2<f32>, Vec2<i32>>(t_b);
			}
			if a.x > b.x { mem::swap(&mut a, &mut b); mem::swap(&mut uv_a, &mut uv_b); }

			for j in a.x..b.x {
				let phi;
				if a.x == b.x {
					phi = 1.0;
				} else {
					phi = ((j - a.x) as f32) / ((b.x - a.x) as f32);
				}
				let p;
				{ // Reduce namespace clutter
					let t_a = cast::<Vec3<i32>, Vec3<f32>>(a);
					let tmp = cast::<Vec3<i32>, Vec3<f32>>(b - a);
					let t_p = (tmp * phi) + t_a;
					p = cast::<Vec3<f32>, Vec3<i32>>(t_p);
				}
				let uv_p = ((uv_b - uv_a) * (phi as i32)) + uv_a;
				let idx = self.trans(p.x as u32, p.y as u32);
				if *zbuf.get(idx).unwrap() < p.z {
					zbuf[idx] = p.z;
					let texture = texture.as_ref().unwrap();
					let t_color = texture.data.get(self.trans(uv_p.x as u32, uv_p.y as u32)).unwrap();
					//let t_color = Color::scale_u32(t_color.0, intensity);
					//let t_color = Color::scale_u8(255, intensity);
					self.plot(p.x as u32, p.y as u32, t_color.clone());
				}

			}
		}
	}
}
