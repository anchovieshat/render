extern crate bincode;
extern crate rustc_serialize;
extern crate nalgebra;
extern crate byteorder;

mod vbuffer;
mod tga;
mod object;

use nalgebra::Vec3;
use vbuffer::{VBuffer, Color};
use object::Object;
use tga::TGA;

fn main() {
	let width = 800;
	let height = 800;
	let depth = 255;
	let light_dir: Vec3<f32> = Vec3::new(0.0, 0.0, -1.0);
	let camera: Vec3<f32> = Vec3::new(0.0, 0.0, 3.0);

	let mut zbuf = Vec::new();
	for _ in 0..(width * height) {
		zbuf.push(std::i32::MIN);
	}

	let mut img = VBuffer::new(width, height);

	let mut obj = Object::load("head.obj", Some("head_diffuse.tga"));

	obj.rasterize(&mut img, &mut zbuf, depth, &light_dir, &camera);

	let out = TGA::new(&img);
	out.save("test.tga");

	let mut zb_img = VBuffer::new(width, height);
	for i in 0..width {
		for j in 0..height {
			let tmp = zbuf[img.trans(i, j)];
			if tmp != std::i32::MIN {
				zb_img.plot(i, j, Color((tmp * depth) as u32));
			} else {
				zb_img.plot(i, j, Color(0x000000FF));
			}
		}
	}

	let zb_out = TGA::new(&zb_img);
	zb_out.save("zbuf.tga");
}
