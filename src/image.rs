use image::{DynamicImage, GenericImageView, ImageReader};

use crate::{
    consts::{BLACK_TILE, WHITE_TILE},
    traversal_path::QR_SIZE,
};

pub type MonochromePixel = (i32, i32, u8);
pub type QrPixels = Vec<MonochromePixel>;

pub fn get_image(path: &str) -> DynamicImage {
    let img = match ImageReader::open(path) {
        Ok(i) => i,
        Err(e) => panic!("Unable to open image \"{}\": {}", path, e),
    };

    match img.decode() {
        Ok(v) => v,
        Err(e) => panic!("Could not decode image \"{}\": {}", path, e),
    }
}

pub fn get_qr_pixels(img_path: &str) -> QrPixels {
    let image = get_image("qr-2-test.png");
    let mut pixels = QrPixels::new();
    for p in image.pixels() {
        let color = if p.2[3] == 255 { 1 } else { 0 };

        pixels.push((p.0 as i32, p.1 as i32, color));
        // println!("{:?}", p);
    }

    return pixels;
}

pub fn print_qr_code(pixels: QrPixels) -> () {
    for (x, _, color) in pixels {
        if color == 1 {
            print!("{}", BLACK_TILE);
        } else {
            print!("{}", WHITE_TILE);
        }

        if x == QR_SIZE - 1 {
            print!("\n")
        }
    }
}
