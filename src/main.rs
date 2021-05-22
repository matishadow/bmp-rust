use show_image::{ImageView, ImageInfo, create_window};
use image::io::Reader as ImageReader;
use std::io;
use image::GenericImageView;
use std::fs;
use std::convert::TryInto;

fn convert_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

struct BmpHeader {
    header_type: [char; 2],
    size: u32,
    reserved1: u16,
    reserved2: u16,
    offset: u32,
}

struct InfoHeader {
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bits: u16,
    compression: u32,
    image_size: u32,
    x_resolution: i32,
    y_resolution: i32,
    n_colors: u32,
    important_colors: u32
}

struct BmpFile {
    header: BmpHeader,
    info_header: InfoHeader,
    data: Vec<u8>
}

impl BmpHeader {
    pub fn new(bytes: &Vec<u8>) -> Self {
        Self {
            header_type: [bytes[0] as char, bytes[1] as char],
            size: ((bytes[5] as u32) << 24 | (bytes[4] as u32) << 16 | (bytes[3] as u32) << 8 | bytes[2] as u32) as u32,
            reserved1: 0,
            reserved2: 0,
            offset: ((bytes[13] as u32) << 24 | (bytes[12] as u32) << 16 | (bytes[11] as u32) << 8 | bytes[10] as u32) as u32,
        }
    }
}

impl InfoHeader {
    pub fn new(bytes: &Vec<u8>) -> Self {
        Self {
            size: ((bytes[17] as u32) << 24 | (bytes[16] as u32) << 16 | (bytes[15] as u32) << 8 | bytes[14] as u32) as u32,
            width: ((bytes[21] as i32) << 24 | (bytes[20] as i32) << 16 | (bytes[19] as i32) << 8 | bytes[18] as i32) as i32,
            height: ((bytes[25] as i32) << 24 | (bytes[24] as i32) << 16 | (bytes[23] as i32) << 8 | bytes[22] as i32) as i32,
            planes: ((bytes[27] as u16) << 8 | bytes[26] as u16) as u16,
            bits: ((bytes[29] as u16) << 8 | bytes[28] as u16) as u16,
            compression: ((bytes[33] as u32) << 24 | (bytes[32] as u32) << 16 | (bytes[32] as u32) << 8 | bytes[30] as u32) as u32,
            image_size: ((bytes[37] as u32) << 24 | (bytes[36] as u32) << 16 | (bytes[35] as u32) << 8 | bytes[34] as u32) as u32,
            x_resolution: ((bytes[41] as i32) << 24 | (bytes[40] as i32) << 16 | (bytes[39] as i32) << 8 | bytes[38] as i32) as i32,
            y_resolution: ((bytes[45] as i32) << 24 | (bytes[44] as i32) << 16 | (bytes[43] as i32) << 8 | bytes[42] as i32) as i32,
            n_colors: ((bytes[49] as u32) << 24 | (bytes[48] as u32) << 16 | (bytes[47] as u32) << 8 | bytes[46] as u32) as u32,
            important_colors: ((bytes[53] as u32) << 24 | (bytes[52] as u32) << 16 | (bytes[51] as u32) << 8 | bytes[50] as u32) as u32
        }
    }
}

impl BmpFile {
    pub fn new(bytes: &Vec<u8>) -> BmpFile {
        let header = BmpHeader::new(&bytes);
        let info_header = InfoHeader::new(&bytes);
        let data = &bytes[header.offset as usize..bytes.len() as usize];

        return Self {
            header,
            info_header,
            data: data.iter().cloned().collect()
        };
    }
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/home/matishadow/.config/JetBrains/Rider2020.3/scratches/bmp-rust/imgs/test-image.bmp";

    let data = fs::read(path).unwrap();

    let bmp_file = BmpFile::new(&data);

    let image_view = ImageView::new(
        ImageInfo::rgb8(bmp_file.info_header.height as u32, bmp_file.info_header.height as u32),
        &bmp_file.data);

    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image_view)?;

    io::stdin().read_line(&mut String::new()).unwrap();
    Ok(())
}