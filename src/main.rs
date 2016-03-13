extern crate bincode;
extern crate rustc_serialize;
extern crate nalgebra;

mod image;
mod tga;
mod object;

use image::Image;
use object::Object;
use tga::TGA;

fn main() {
	let width = 4096;
	let height = 4096;

	let mut zbuf = Vec::new();
	for _ in 0..(width * height) {
		zbuf.push(std::f32::NEG_INFINITY);
	}

	let mut img = Image::new(width, height);

	let obj = Object::load("head.obj");

	obj.rasterize(&mut img, &mut zbuf);

	let out = TGA::new(&img);
	out.save("test.tga");
}
