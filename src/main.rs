extern crate bincode;
extern crate rustc_serialize;

mod image;
mod tga;
use image::{Image, Color};
use tga::TGA;

fn main() {
	let mut img = Image::new(100, 100);
	let white = Color(0xFFFFFFFF);
	let red =   Color(0xFF0000FF);

	img.line(13, 20, 80, 40, &white.clone());
	img.line(20, 13, 40, 80, &red.clone());
	img.line(80, 40, 13, 20, &red.clone());

	let out = TGA::new(&img);
	out.save("test.tga");
}
