use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
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

					let vert = Vec3 {
						x: values[0],
						y: values[1],
						z: values[2],
					};

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
}
