use std::fs::File;
use std::io::Write;

use bincode::SizeLimit;
use bincode::rustc_serialize::encode;

use image::Color;
use image::Image;

#[derive(RustcEncodable, Default)]
#[repr(C, packed)]
pub struct TGA_Header {
	id_len: u8,
	colormap_t: u8,
	data_t: u8,
	colormap_origin: u16,
	colormap_len: u16,
	colormap_depth: u8,
	x_origin: u16,
	y_origin: u16,
	width: u16,
	height: u16,
	bits_per_pixel: u8,
	img_desc: u8,
}

pub struct TGA {
	head: TGA_Header,
	data: Vec<Color>,
}

impl TGA {
	pub fn new(img: &Image) -> TGA {
		let mut configured_data = img.data.clone();
		for c in configured_data.iter_mut() {
			c.0 = c.0.swap_bytes();
		}
		TGA {
			head: TGA_Header {
				data_t: 2,
				width: (img.width as u16).swap_bytes(),
				height: (img.height as u16).swap_bytes(),
				bits_per_pixel: 32,
				img_desc: 0x28,
				..Default::default()
			},
			data: configured_data,
		}
	}

	pub fn save(&self, filename: &str) {
		let mut file = File::create(filename).unwrap();

		let encoded_head: Vec<u8> = encode(&self.head, SizeLimit::Infinite).unwrap();
		let mut encoded_data: Vec<u8> = Vec::new();
		for x in 0..self.data.len() {
			let tmp = self.data[x].0;
			encoded_data.push(((tmp << 24) >> 24) as u8);
			encoded_data.push(((tmp << 16) >> 24) as u8);
			encoded_data.push(((tmp << 8) >> 24) as u8);
			encoded_data.push((tmp >> 24) as u8);
		}

		file.write_all(&encoded_head).unwrap();
		file.write_all(&encoded_data).unwrap();
	}
}
