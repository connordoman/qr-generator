use std::cmp::{max, min};

use ::image::GenericImageView;
use chrono;
use consts::{DEBUG, QR_VERSION};
use image::{get_image, get_qr_pixels, print_qr_code};
use qr::{
    board::{QRBoard, Tile},
    traversal::{traverse_board, traverse_board_2},
};
// use qr_board::QRBoard;

mod consts;
mod image;
mod macros;
mod qr;
mod traversal_path;
// mod qr_board;

fn print_debug(msg: &str) {
    if DEBUG {
        let date = chrono::offset::Local::now();
        eprint!("[{:?}] {}", date, msg);
    }
}

pub fn format_radix(mut x: u32, radix: u32) -> String {
    let mut result = vec![];

    loop {
        let m = x % radix;
        x = x / radix;

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(std::char::from_digit(m, radix).unwrap());
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}

fn print_grid_headline(size: i32) {
    let mut res = String::new();

    let radix = max(0, min(size, 36)) as u32;

    for i in 0..size {
        let col_str = format_radix(i as u32, radix);
        let digit = if col_str.len() > 1 {
            col_str
        } else {
            format!(" {}", col_str)
        };
        res.push_str(digit.as_str());
    }
    res.push_str("\n");

    print!("{}", res);
}

fn main() {
    // let mut board = QRBoard::new(QR_VERSION);
    // board.init();
    // // board.walk_grid();
    // board.draw();

    // let size = board.size();

    // let traversal = traverse_board_2(board);

    // for tile in traversal {
    //     board.update_tile(tile.row, tile.col, tile.value);
    // }

    // print_grid_headline(size);
    // println!("{}", board);
    let image = get_qr_pixels("qr-2-test.png");
    print_qr_code(image);
}
