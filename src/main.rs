extern crate bincode;
extern crate rustc_serialize;

mod image;
mod tga;
mod object;

use image::{Image, Color};
use object::Object;
use tga::TGA;

fn main() {
	let width = 800;
	let height = 800;

	let mut img = Image::new(width, height);
	let obj = Object::load("head.obj");
	let white = Color(0xFFFFFFFF);
	let red =   Color(0xFF0000FF);

	println!("number of verts: {}", obj.verts.len());
	println!("number of faces: {}", obj.faces.len());

	for i in 0..obj.faces.len() {
		let face = obj.faces.get(i).unwrap();
		for j in 0..3 {
			let v0 = obj.verts.get(face[j] as usize).unwrap();
			let v1 = obj.verts.get(face[(j + 1) % 3] as usize).unwrap();

			let x0 = (((v0.x + 1.0) * (width as f32))  / 2.0) as u32;
			let y0 = (((v0.y + 1.0) * (height as f32)) / 2.0) as u32;
			let x1 = (((v1.x + 1.0) * (width as f32))  / 2.0) as u32;
			let y1 = (((v1.y + 1.0) * (height as f32)) / 2.0) as u32;
			img.line(x0, y0, x1, y1, &white.clone());
		}
	}

	let out = TGA::new(&img);
	out.save("test.tga");
}
