extern crate bincode;
extern crate rustc_serialize;

mod image;
mod tga;
mod object;

use image::Image;
use object::Object;
use tga::TGA;

fn main() {
	let width = 2048;
	let height = 2048;

	let mut img = Image::new(width, height);

	let obj = Object::load("head.obj");
	obj.draw(&mut img);

	let out = TGA::new(&img);
	out.save("test.tga");
}
