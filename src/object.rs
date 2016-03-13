use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use nalgebra::Vec3;
use nalgebra::cross;
use nalgebra::Norm;

use image::{Image, Color};

fn one_mul(u: &Vec3<f32>, v: &Vec3<f32>) -> f32 {
	(u.x * v.x) + (u.y * v.y) + (u.z * v.z)
}

pub fn barycentric(a: &Vec3<f32>, b: &Vec3<f32>, c: &Vec3<f32>, p: &Vec3<f32>) -> Vec3<f32> {
	let mut t = Vec::new();
	for _ in 0..2 {
		t.push(Vec3::new(0.0, 0.0, 0.0));
	}
	for i in 0..2 {
		t[i][0] = c[i] - a[i];
		t[i][1] = b[i] - a[i];
		t[i][2] = a[i] - p[i];
	}
	let u = cross(&t[0], &t[1]);

	if u[2].abs() < 1e-2 { return Vec3::new(-1.0, 1.0, 1.0); }
	return Vec3::new(1.0 - ((u.x + u.y) / u.z), u.y / u.z, u.x / u.z);
}

fn world_to_screen(height: u32, width: u32, v: &Vec3<f32>) -> Vec3<f32> {
	Vec3::new(((((v.x + 1.0) * (width as f32)) / 2.0) + 0.5) as i32 as f32, ((((v.y + 1.0) * (height as f32)) / 2.0) + 0.5) as i32 as f32, v.z)
}

pub struct Object {
	pub verts: Vec<Vec3<f32>>,
	pub faces: Vec<Vec<u32>>,
	pub normals: Vec<Vec3<f32>>,
	pub tex_map: Vec<Vec3<f32>>,
}

impl Object {
	pub fn load(filename: &str) -> Object {
		let mut verts = Vec::new();
		let mut faces = Vec::new();
		let mut normals = Vec::new();
		let mut tex_map = Vec::new();

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
				} else if line.contains("vn ") {
					let mut values = Vec::new();

					for chunk in line.split_whitespace() {
						if chunk != "vn" {
							values.push(chunk.parse::<f32>().unwrap());
						}
					}

					let norm = Vec3::new(values[0], values[1], values[2]);

					normals.push(norm);
				} else if line.contains("vt ") {
					let mut values = Vec::new();

					for chunk in line.split_whitespace() {
						if chunk != "vt" {
							values.push(chunk.parse::<f32>().unwrap());
						}
					}

					let tex = Vec3::new(values[0], values[1], values[2]);

					tex_map.push(tex);
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
			normals: normals,
			tex_map: tex_map,
		}
	}

	pub fn wireframe(&self, img: &mut Image, color: &Color) {
		for i in 0..self.faces.len() {
			let face = self.faces.get(i).unwrap();
			for j in 0..3 {
				let v0 = self.verts.get(face[j] as usize).unwrap();
				let v1 = self.verts.get(face[(j + 1) % 3] as usize).unwrap();
				let x0 = (((v0.x + 1.0) * (img.width as f32))  / 2.0) as u32;
				let y0 = (((v0.y + 1.0) * (img.height as f32)) / 2.0) as u32;
				let x1 = (((v1.x + 1.0) * (img.width as f32))  / 2.0) as u32;
				let y1 = (((v1.y + 1.0) * (img.height as f32)) / 2.0) as u32;
				img.line(x0, y0, x1, y1, &color.clone());
			}
		}
	}
	pub fn rasterize(&self, img: &mut Image, zbuf: &mut Vec<f32>) {
		let light_dir: Vec3<f32> = Vec3::new(0.0, 0.0, -1.0);

		for i in 0..self.faces.len() {
			let face = self.faces.get(i).unwrap();
			let mut world_coords = Vec::new();
			let mut screen_coords = Vec::new();

			for j in 0..3 {
				world_coords.push(Vec3::new(0.0, 0.0, 0.0));
				let v = self.verts.get(face[j] as usize).unwrap();
				screen_coords.push(world_to_screen(img.height, img.width, v));
				world_coords[j] = *v;
			}

			let n = cross(&(world_coords[2] - world_coords[0]), &(world_coords[1] - world_coords[0])).normalize();
			let intensity = one_mul(&n, &light_dir);
			if intensity > 0.0 {
				let mut t = vec![screen_coords[0], screen_coords[1], screen_coords[2]];
				img.triangle(&mut t, zbuf, &Color::new((intensity * 255.0) as u8, (intensity * 255.0) as u8, (intensity * 255.0) as u8, 255));
			}
		}
	}
}
