use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::ops::{Sub, Mul, Add};

use image::{Image, Color};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Vec2<T> {
	pub x: T,
	pub y: T,
}

impl<T> Vec2<T> {
	/*pub fn new(x: T, y: T) -> Vec2<T> {
		Vec2 {
			x: x,
			y: y,
		}
	}*/

	pub fn new(v: (T, T)) -> Vec2<T> {
		Vec2 {
			x: v.0,
			y: v.1,
		}
	}
}

impl Sub for Vec2<f32> {
	type Output = Vec2<f32>;

	fn sub(self, v: Vec2<f32>) -> Vec2<f32> {
		Vec2::new((self.x - v.x, self.y - v.y))
	}
}

impl Mul for Vec2<f32> {
	type Output = Vec2<f32>;

	fn mul(self, v: Vec2<f32>) -> Vec2<f32> {
		Vec2::new((self.x * v.x, self.y * v.y))
	}
}

impl Add for Vec2<f32> {
	type Output = Vec2<f32>;

	fn add(self, v: Vec2<f32>) -> Vec2<f32> {
		Vec2::new((self.x + v.x, self.y + v.y))
	}
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

impl<T> Vec3<T> {
	pub fn new(x: T, y: T, z: T) -> Vec3<T> {
		Vec3 {
			x: x,
			y: y,
			z: z,
		}
	}
}

#[derive(Debug)]
pub struct Triangle<T> {
	pub p0: Vec2<T>,
	pub p1: Vec2<T>,
	pub p2: Vec2<T>,
}

impl<T> Triangle<T> {
	pub fn new(p0: Vec2<T>, p1: Vec2<T>, p2: Vec2<T>) -> Triangle<T> {
		Triangle {
			p0: p0,
			p1: p1,
			p2: p2,
		}
	}
}



pub fn cross_f(u: &Vec3<f32>, v: &Vec3<f32>) -> Vec3<f32> {
	Vec3::new(((u.y * v.z) - (u.z * v.y)), ((u.z * v.x) - (u.x * v.z)), ((u.x * v.y) - (u.y * v.x)))
}

pub fn sub(u: &Vec3<f32>, v: &Vec3<f32>) -> Vec3<f32> {
	Vec3::new((u.x - v.x), (u.y - v.y), (u.z - v.z))
}

pub fn mul_to_num(u: &Vec3<f32>, v: &Vec3<f32>) -> f32 {
	(u.x * v.x) + (u.y * v.y) + (u.z * v.z)
}

pub fn num_to_mul(v: &Vec3<f32>, n: f32) -> Vec3<f32> {
	Vec3::new((v.x * n), (v.y * n), (v.z * n))
}

pub fn magnitude(v: &Vec3<f32>) -> f32 {
	((v.x * v.x) + (v.y * v.y) + (v.z * v.z)).sqrt()
}

pub fn normalize(v: &Vec3<f32>) -> Vec3<f32> {
	let mag = magnitude(v);
	num_to_mul(v, 1.0 / mag)
}

pub struct Object {
	pub verts: Vec<Vec3<f32>>,
	pub faces: Vec<Vec<u32>>,
}

impl Object {
	pub fn load(filename: &str) -> Object {
		let mut verts = Vec::new();
		let mut faces = Vec::new();

		let file = File::open(filename).unwrap();
		let reader = BufReader::new(file);

		for line in reader.lines() {
			let line = line.unwrap();
			if !line.contains("#") {
				if line.contains("v ") {
					let mut values = Vec::new();

					for chunk in line.split_whitespace() {
						if chunk != "v" {
							values.push(chunk.parse::<f32>().unwrap());
						}
					}

					let vert = Vec3::new(values[0], values[1], values[2]);

					verts.push(vert);
				} else if line.contains("f ") {
					let mut values = Vec::new();

					for chunk in line.split_whitespace() {
						if chunk != "f" {
							if line.contains("/") {
								let piece = chunk.split('/').nth(0).unwrap();
								values.push(piece.parse::<u32>().unwrap() - 1);
							} else {
								values.push(chunk.parse::<u32>().unwrap() - 1);
							}
						}
					}
					faces.push(values);
				}
			}
		}

		Object {
			verts: verts,
			faces: faces,
		}
	}

	pub fn draw(&self, img: &mut Image, zbuf: &mut Vec<f32>) {
		let light_dir: Vec3<f32> = Vec3::new(0.0, 0.0, -0.7);

		for i in 0..self.faces.len() {
			let face = self.faces.get(i).unwrap();
			let mut screen_coords = Vec::new();
			let mut world_coords = Vec::new();
			for j in 0..3 {
				let v = self.verts.get(face[j] as usize).unwrap();
				let tmp = Vec2::new(((((v.x + 1.0) * (img.width as f32)) / 2.0), (((v.y + 1.0) * (img.height as f32)) / 2.0)));
				screen_coords.push(tmp);
				world_coords.push(v);
			}
			let mut n = cross_f(&sub(world_coords[2], world_coords[0]), &sub(world_coords[1], world_coords[0]));
			n = normalize(&n);

			let intensity = mul_to_num(&n, &light_dir);

			if intensity > 0.0 {
				let mut t = Triangle::new(screen_coords[0].clone(), screen_coords[1].clone(), screen_coords[2].clone());
				img.triangle(&mut t, zbuf, &Color::new((intensity * 255.0) as u8, (intensity * 255.0) as u8, (intensity * 255.0) as u8, 255));
			}
		}
	}
}
