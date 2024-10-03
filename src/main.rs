use chrono;
use std::collections::HashMap;

const DEBUG: bool = true;

const QR_VERSION: i32 = 1;

const CHECKERED_DEFAULT: bool = false;

const BLACK_TILE: &str = "⬛️";
const WHITE_TILE: &str = "⬜️";
const RED_TILE: &str = "🟥";
const GREEN_TILE: &str = "🟩";
const BLUE_TILE: &str = "🟦";

const POSITION_MARKER_SIZE: i32 = 8;

type QRBoard = HashMap<(i32, i32), i32>;

enum TimingMark {
    Vertical,
    Horiztonal,
}

fn print_debug(msg: &str) {
    if DEBUG {
        let date = chrono::offset::Local::now();
        eprint!("[{:?}] {}", date, msg);
    }
}

fn version_size(ver: i32) -> i32 {
    (4 * ver) + 17
}

fn draw_square(board: &mut QRBoard, x: i32, y: i32, size: i32, value: i32) {
    for i in 0..size {
        let row = i + y;

        for j in 0..size {
            let col = j + x;

            board.insert((row, col), value);
        }
    }
}

fn draw_indicator_marks(board: &mut QRBoard, size: i32) {
    // top left identifying mark
    draw_square(board, 0, 0, POSITION_MARKER_SIZE, 0);
    draw_square(board, 0, 0, 7, 1);
    draw_square(board, 1, 1, 5, 0);
    draw_square(board, 2, 2, 3, 1);
    // top right identifying mark
    draw_square(board, size - 8, 0, POSITION_MARKER_SIZE, 0);
    draw_square(board, size - 7, 0, 7, 1);
    draw_square(board, size - 6, 1, 5, 0);
    draw_square(board, size - 5, 2, 3, 1);
    // bottom left identifying mark
    draw_square(
        board,
        0,
        size - POSITION_MARKER_SIZE,
        POSITION_MARKER_SIZE,
        0,
    );
    draw_square(board, 0, size - 7, 7, 1);
    draw_square(board, 1, size - 6, 5, 0);
    draw_square(board, 2, size - 5, 3, 1);
}

fn draw_alignment_mark(board: &mut QRBoard, x: i32, y: i32) {
    draw_square(board, x, y, 5, 1);
    draw_square(board, x + 1, y + 1, 3, 0);
    draw_square(board, x + 2, y + 2, 1, 1);
}

fn draw_timing_mark(board: &mut QRBoard, size: i32, orientation: TimingMark) {
    let start = POSITION_MARKER_SIZE;
    let end = size - POSITION_MARKER_SIZE;

    for i in start..end {
        let tile = match i % 2 {
            0 => 1,
            _ => 0,
        };

        match orientation {
            TimingMark::Horiztonal => {
                board.insert((POSITION_MARKER_SIZE - 2, i), tile);
            }
            TimingMark::Vertical => {
                board.insert((i, POSITION_MARKER_SIZE - 2), tile);
            }
        }
    }
}

fn draw_timing_marks(board: &mut QRBoard, size: i32) {
    draw_timing_mark(board, size, TimingMark::Horiztonal);
    draw_timing_mark(board, size, TimingMark::Vertical);
}

fn walk_grid(board: &mut QRBoard, size: i32) {
    println!("width: {}", size);
    print!("slices: [");
    for s in (1..size).rev() {
        print!("{}, ", s);
    }
    print!("]\n");

    for slice in (0..size).rev() {
        let double_sliced = slice % 4 == 0 || (slice - 1) % 4 == 0;

        let mut x = slice;

        // let x = if col

        for y in 0..size {
            if y == x {
                board.insert((y, x), 4);
                board.insert((y, x + 1), 4);
                continue;
            }

            if double_sliced {
                board.insert((y, x), 3);
                board.insert((y, x + 1), 3);
            } else {
                board.insert((y, x), 2);
                board.insert((y, x + 1), 2);
            }
        }
    }
}

fn format_radix(mut x: u32, radix: u32) -> String {
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
    for i in 0..size {
        print!("{} ", format_radix(i as u32, size as u32));
    }
    print!("\n");
}

fn main() {
    let size = version_size(QR_VERSION);

    let mut board: QRBoard = HashMap::new();

    // insert alternating squares
    for i in 0..size {
        for j in 0..size {
            if CHECKERED_DEFAULT && ((i % 2) + j % 2) % 2 == 0 {
                board.insert((i, j), 1);
            } else {
                board.insert((i, j), 0);
            }
        }
    }

    walk_grid(&mut board, size);

    // draw_indicator_marks(&mut board, size);
    // draw_timing_marks(&mut board, size);
    // draw_alignment_mark(&mut board, size - 9, size - 9);

    print_grid_headline(size);

    // display board
    for i in 0..size {
        for j in 0..size {
            let board_value = match board.get(&(i, j)) {
                Some(b) => b.clone(),
                None => 0,
            };

            let board_tile: &str = match board_value {
                1 => BLACK_TILE,
                2 => RED_TILE,
                3 => BLUE_TILE,
                4 => GREEN_TILE,
                _ => WHITE_TILE,
            };

            print!("{}", board_tile);
        }
        print!("\n")
    }
}
