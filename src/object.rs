use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use nalgebra::{Vec2, Vec3, DMat};
use nalgebra::cross;
use nalgebra::Norm;
use nalgebra::new_identity;

use vbuffer::VBuffer;

fn one_mul(u: &Vec3<f32>, v: &Vec3<f32>) -> f32 {
	(u.x * v.x) + (u.y * v.y) + (u.z * v.z)
}

pub fn view_port(x: f32, y: f32, w: f32, h: f32, depth: i32) -> DMat<f32> {
	let mut m: DMat<f32> = new_identity(4);
	m[(0, 3)] = (x + w) / 2.0;
	m[(1, 3)] = (y + h) / 2.0;
	m[(2, 3)] = (depth as f32) / 2.0;

	m[(0, 0)] = w / 2.0;
	m[(1, 1)] = h / 2.0;
	m[(2, 2)] = (depth as f32) / 2.0;
	return m;
}

pub fn mat_to_v3(m: &DMat<f32>) -> Vec3<i32> {
	Vec3::new((m[(0, 0)] / m[(3, 0)]) as i32, (m[(1, 0)] / m[(3, 0)]) as i32, (m[(2, 0)] / m[(3, 0)]) as i32)
}

pub fn v3_to_mat(v: &Vec3<f32>) -> DMat<f32> {
	unsafe {
		let mut m = DMat::new_uninitialized(4, 1);
		m[(0, 0)] = v.x;
		m[(1, 0)] = v.y;
		m[(2, 0)] = v.z;
		m[(3, 0)] = 1.0;
		return m;
	}
}

pub struct Object {
	pub verts: Vec<Vec3<f32>>,
	pub faces: Vec<Vec<Vec3<i32>>>,
	pub normals: Vec<Vec3<f32>>,
	pub tex_map: Vec<Vec3<f32>>,
	pub texture: Option<VBuffer>,
}

impl Object {
	pub fn load(obj_fname: &str, tex_fname: Option<&str>) -> Object {
		let mut verts = Vec::new();
		let mut faces = Vec::new();
		let mut normals = Vec::new();
		let mut tex_map = Vec::new();

		let file = File::open(obj_fname).unwrap();
		let texture;
		if tex_fname.is_some() {
			texture = VBuffer::load(tex_fname.unwrap());
		} else {
			texture = None;
		}

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
							for i in 0..3 {
								let piece = chunk.split('/').nth(i).unwrap();
								values.push(piece.parse::<i32>().unwrap() - 1);
							}
						}
					}
					let mut t_values = Vec::new();
					for _ in 0..3 {
						let n = values.pop().unwrap();
						let u = values.pop().unwrap();
						let v = values.pop().unwrap();
						let tmp = Vec3::new(v, u, n);
						t_values.push(tmp);
					}
					t_values.reverse();
					faces.push(t_values);
				}
			}
		}

		Object {
			verts: verts,
			faces: faces,
			normals: normals,
			tex_map: tex_map,
			texture: texture,
		}
	}

	pub fn face(&mut self, idx: usize) -> Vec<i32> {
		let mut face = Vec::new();
		let end = self.faces.get(idx).unwrap().len();
		for i in 0..end {
			face.push(self.faces.get(idx).unwrap().get(i).unwrap()[0]);
		}
		face
	}

	pub fn uv(&self, f_idx: u32, v_idx: u32) -> Vec2<i32> {
		let tmp = self.faces.get(f_idx as usize).unwrap();
		let idx = tmp.get(v_idx as usize).unwrap()[1];
		let tex = self.texture.as_ref().unwrap();
		let x = self.tex_map.get(idx as usize).unwrap().x;
		let y = self.tex_map.get(idx as usize).unwrap().y;
		Vec2::new((x * (tex.width as f32)) as i32, (y * (tex.height as f32)) as i32)
	}

	pub fn rasterize(&mut self, img: &mut VBuffer, zbuf: &mut Vec<i32>, depth: i32, light_dir: &Vec3<f32>, camera: &Vec3<f32>) {
		let mut projection: DMat<f32> = new_identity(4);
		let view = view_port((img.width / 8) as f32, (img.height / 8) as f32, ((img.width * 3) / 4) as f32,  ((img.height * 3) / 4) as f32, depth);
		projection[(3, 2)] = -1.0 / camera.z;

		for i in 0..self.faces.len() {
			let face = self.face(i);
			let mut world_coords = Vec::new();
			let mut screen_coords = Vec::new();

			for j in 0..3 {
				world_coords.push(Vec3::new(0.0, 0.0, 0.0));
				let v = self.verts.get(face[j] as usize).unwrap();
				screen_coords.push(mat_to_v3(&(view.clone() * projection.clone() * v3_to_mat(v))));
				world_coords[j] = *v;
			}

			let n = cross(&(world_coords[2] - world_coords[0]), &(world_coords[1] - world_coords[0])).normalize();
			let intensity = one_mul(&n, &light_dir);
			if intensity > 0.0 {
				let mut uv = Vec::new();
				for k in 0..3 {
					uv.push(self.uv(i as u32, k));
				}
				let mut t = screen_coords;

				img.triangle(&mut t, &mut uv, zbuf, intensity, &self.texture);
			}
		}
	}
}
