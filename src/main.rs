use show_image::{ImageView, ImageInfo, create_window};
use image::io::Reader as ImageReader;
use std::io;
use image::GenericImageView;
use std::fs;

#[repr(packed)]
struct BmpHeader {
    header_type: [char; 2],
    size: u32,
    reserved1: u16,
    reserved2: u16,
    offset: u32,
}

impl BmpHeader {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            header_type: [bytes[0] as char, bytes[1] as char],
            size: ((bytes[5] as u32) << 24 | (bytes[4] as u32) << 16 | (bytes[3] as u32) << 8 | bytes[2] as u32) as u32,
            reserved1: 0,
            reserved2: 0,
            offset: ((bytes[13] as u32) << 24 | (bytes[12] as u32) << 16 | (bytes[11] as u32) << 8 | bytes[10] as u32) as u32,
        }
    }
}

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "/home/matishadow/.config/JetBrains/Rider2020.3/scratches/bmp-rust/imgs/test-image.bmp";

    let data = fs::read(path).unwrap();
    let header_size = std::mem::size_of::<BmpHeader>();

    let header = BmpHeader::new(data);

    let dynamic_image = ImageReader::open(path)?.decode()?;

    let rgb_image = dynamic_image.to_rgb8();

    let image_view = ImageView::new(ImageInfo::rgb8(dynamic_image.width(), dynamic_image.height()), rgb_image.as_raw());

    // Create a window with default options and display the image.
    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image_view)?;

    io::stdin().read_line(&mut String::new()).unwrap();
    Ok(())
}