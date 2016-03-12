extern crate bincode;
extern crate rustc_serialize;

mod image;
mod tga;
mod object;

use image::{Image, Color};
use object::{Object, Vec2};
use tga::TGA;

fn main() {
	let width = 200;
	let height = 200;

	let mut img = Image::new(width, height);
	let white = Color(0xFFFFFFFF);
	let red = Color(0xFF0000FF);
	let green = Color(0x00FF00FF);

	let t0 = vec![Vec2 { x: 10, y: 70 }, Vec2 { x: 50, y: 160 }, Vec2 { x: 70, y: 80 }];
	let t1 = vec![Vec2 { x: 180, y: 50 }, Vec2 { x: 150, y: 1 }, Vec2 { x: 70, y: 180 }];
	let t2 = vec![Vec2 { x: 180, y: 150 }, Vec2 { x: 120, y: 160 }, Vec2 { x: 130, y: 180 }];

	img.triangle(t0.get(0).unwrap(), t0.get(1).unwrap(), t0.get(2).unwrap(), &red);
	img.triangle(t1.get(0).unwrap(), t1.get(1).unwrap(), t1.get(2).unwrap(), &white);
	img.triangle(t2.get(0).unwrap(), t2.get(1).unwrap(), t2.get(2).unwrap(), &green);

	//let obj = Object::load("head.obj");
	//obj.draw(&mut img, &white);

	let out = TGA::new(&img);
	out.save("test.tga");
}
