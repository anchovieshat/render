use std::fs::File;
use std::io::Write;

use bincode::SizeLimit;
use bincode::rustc_serialize::encode;

use byteorder::{LittleEndian, ReadBytesExt};

use vbuffer::Color;
use vbuffer::VBuffer;

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
	pub width: u16,
	pub height: u16,
	bits_per_pixel: u8,
	img_desc: u8,
}

pub struct TGA {
	pub head: TGA_Header,
	pub data: Vec<Color>,
}

impl TGA {
	pub fn new(img: &VBuffer) -> TGA {
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
				img_desc: 0x08,
				..Default::default()
			},
			data: configured_data,
		}
	}

	pub fn save(&self, filename: &str) {
		let mut file = File::create(filename).unwrap();

		println!("saving {}..", filename);

		let encoded_head: Vec<u8> = encode(&self.head, SizeLimit::Infinite).unwrap();
		let mut encoded_data: Vec<u8> = Vec::new();
		for x in 0..self.data.len() {
			let tmp = self.data[x].0;
			encoded_data.push(((tmp << 8) >> 24) as u8);
			encoded_data.push(((tmp << 16) >> 24) as u8);
			encoded_data.push(((tmp << 24) >> 24) as u8);
			encoded_data.push((tmp >> 24) as u8);
		}

		file.write_all(&encoded_head).unwrap();
		file.write_all(&encoded_data).unwrap();
	}

	pub fn load(filename: &str) -> Option<TGA> {
		let file = File::open(filename).ok();

		if file.is_some() {
			let mut file = file.unwrap();
			let id_len = file.read_u8().unwrap();
			let colormap_t = file.read_u8().unwrap();
			let data_t = file.read_u8().unwrap();
			let colormap_origin = file.read_u16::<LittleEndian>().unwrap();
			let colormap_len = file.read_u16::<LittleEndian>().unwrap();
			let colormap_depth = file.read_u8().unwrap();
			let x_origin = file.read_u16::<LittleEndian>().unwrap();
			let y_origin = file.read_u16::<LittleEndian>().unwrap();
			let width = file.read_u16::<LittleEndian>().unwrap();
			let height = file.read_u16::<LittleEndian>().unwrap();
			let bits_per_pixel = file.read_u8().unwrap();
			let img_desc = file.read_u8().unwrap();
			let mut data: Vec<Color> = Vec::with_capacity(((width as usize) * (height as usize)));
			let bytes = (bits_per_pixel as usize) / 8;

			if (data_t == 10) || (data_t == 11) { // RLE
				println!("Decoding RLE...");
				println!("bytes: {}", bytes);
				let pix_count = (width as u32) * (height as u32);
				let mut cur_pix = 0;
				let mut cur_byte = 0;
				let mut byte_data: Vec<u8> = Vec::with_capacity(((width as usize) * (height as usize) * bytes));
				for _ in 0..byte_data.capacity() {
					byte_data.push(0);
				}
				println!("pixels: {}", pix_count); //933803
				println!("filename: {}, dimensions: ({}, {}), size: {}, desc: {}", filename, width, height, pix_count, img_desc);
				while cur_pix < pix_count {
					let mut chunk_header = file.read_u8().unwrap();;
					if chunk_header < 128 {
						chunk_header += 1;

						for _ in 0..chunk_header {
							let mut color_buf = Vec::new();
							for _ in 0..bytes {
								color_buf.push(file.read_u8().unwrap());
							}
							for t in 0..bytes {
								byte_data[cur_byte] = color_buf[t];
								cur_byte += 1;
							}
							cur_pix += 1;
						}
					} else {
						chunk_header -= 127;
						let mut color_buf = Vec::new();
						for _ in 0..bytes {
							color_buf.push(file.read_u8().unwrap());
						}
						for _ in 0..chunk_header {
							for t in 0..bytes {
								byte_data[cur_byte] = color_buf[t];
								cur_byte += 1;
							}
							cur_pix += 1;
						}
					}
				}

				let mut i = 0;
				while i < byte_data.len() {
					data.push(Color::new(byte_data[i + 2], byte_data[i + 1], byte_data[i + 0], 0xFF));
					i += 3;
				}
			} else if (data_t == 2) || (data_t == 3) { // Plain
				let mut skip_count = 0;
				for _ in 0..data.capacity() {

					let mut color_v = Vec::with_capacity(4);

					for _ in 0..bytes {
						let channel = file.read_u8().ok();
						if channel.is_some() {
							color_v.push(channel.unwrap());
						}
					}
					let size = color_v.len();
					if size == 0 {
						skip_count += 1;
					}
					for _ in 0..(4 - size) {
						color_v.push(0);
					}
					let color = Color::new(color_v[0], color_v[1], color_v[2], 0xFF);
					data.push(color);
				}

				println!("skips? {}", skip_count);
			}

			Some(TGA {
				head: TGA_Header {
					id_len: id_len,
					colormap_t: colormap_t,
					data_t: data_t,
					colormap_origin: colormap_origin,
					colormap_len: colormap_len,
					colormap_depth: colormap_depth,
					x_origin: x_origin,
					y_origin: y_origin,
					width: width,
					height: height,
					bits_per_pixel: bits_per_pixel,
					img_desc: img_desc,
				},
				data: data,
			})
		} else {
			None
		}
	}
}
