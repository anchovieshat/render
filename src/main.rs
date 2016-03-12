extern crate bincode;
extern crate rustc_serialize;

mod image;
mod tga;
use image::{Image, Color};
use tga::TGA;

fn main() {
	let mut img = Image::new(10, 10);
	let white = Color(0xFFFFFFFF);
	img.line(0, 0, 9, 9, &white);

	let out = TGA::new(&img);
	out.save("test.tga");
}
