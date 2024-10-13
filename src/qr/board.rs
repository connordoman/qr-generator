use std::{
    cmp::{max, min},
    collections::HashMap,
};

use crate::{
    consts::{
        BLACK_TILE, BLUE_TILE, CHECKERED_DEFAULT, COLORS, GREEN_TILE, POSITION_MARKER_SIZE,
        RED_TILE, WHITE_TILE,
    },
    format_radix,
};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Tile {
    pub value: i32,
    pub visited: bool,
    pub structural: bool,
    pub row: i32,
    pub col: i32,
}

impl Tile {
    pub fn new(value: i32, row: i32, col: i32) -> Self {
        return Tile {
            value,
            visited: false,
            structural: false,
            row,
            col,
        };
    }

    fn get_tile_str(&self) -> &str {
        let color_len = COLORS.len() as i32;

        if self.value >= 0 && self.value < color_len {
            return COLORS[self.value as usize];
        }

        match self.value {
            1 => BLACK_TILE,
            _ => WHITE_TILE,
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct Position {
    row: i32,
    col: i32,
}

impl Position {
    pub fn new(row: i32, col: i32) -> Self {
        return Position { row, col };
    }
}

enum TimingMark {
    Vertical,
    Horiztonal,
}

pub struct QRBoard {
    pub version: i32,
    tiles: HashMap<Position, Tile>,
}

impl QRBoard {
    pub fn new(version: i32) -> Self {
        QRBoard {
            version,
            tiles: HashMap::new(),
        }
    }

    pub fn init(&mut self) {
        for i in 0..self.size() {
            for j in 0..self.size() {
                if CHECKERED_DEFAULT && ((i % 2) + j % 2) % 2 == 0 {
                    self.insert_tile(i, j, 1);
                } else {
                    self.insert_tile(i, j, 0);
                }
            }
        }
    }

    pub fn draw(&mut self) {
        self.draw_position_marks();
        self.draw_timing_marks();
        self.draw_alignment_marks();
        self.draw_format_info();
    }

    pub fn size(&self) -> i32 {
        (4 * self.version) + 17
    }

    fn get_tile_mut(&mut self, row: i32, col: i32) -> Option<&mut Tile> {
        self.tiles.get_mut(&Position::new(row, col))
    }

    pub fn get_tile(&self, row: i32, col: i32) -> Option<&Tile> {
        self.tiles.get(&Position::new(row, col))
    }

    pub fn is_structural(&self, row: i32, col: i32) -> Result<bool, &str> {
        match self.get_tile(row, col) {
            Some(t) => Ok(t.structural),
            None => Err("Error collecting requested tile."),
        }
    }

    fn insert_tile(&mut self, row: i32, col: i32, value: i32) {
        let tile = Tile::new(value, row, col);
        self.tiles.insert(Position::new(row, col), tile);
    }

    pub fn update_tile(&mut self, row: i32, col: i32, value: i32) {
        self.insert_tile(row, col, value);
        self.visit_tile(row, col);
    }

    fn visit_tile(&mut self, row: i32, col: i32) -> Option<&Tile> {
        let tile: &mut Tile = match self.get_tile_mut(row, col) {
            Some(t) => t,
            None => return None,
        };

        if tile.visited == false {
            tile.visited = true;
        }

        return Some(tile);
    }

    pub fn walk_grid(&mut self) {
        // let size = 1;
        let size = self.size();

        let slices = (0..size).rev();

        println!("width: {}", size);
        print!("slices: [");
        for s in slices.clone() {
            print!("{}, ", s);
        }
        print!("]\n");

        for slice in slices.clone() {
            let double_sliced = slice % 4 == 0 || (slice - 1) % 4 == 0;
            let doubled_sliced_tile = if double_sliced { 3 } else { 2 };

            let x = if slice <= 6 { slice - 1 } else { slice };

            // let x = if col
            // print!("x: {}\ty: ", x);

            for y in (0..size).rev() {
                // print!("{}, ", y);
                // if y == x {
                //     self.update_tile(y, x, 4);
                //     self.update_tile(y, x + 1, doubled_sliced_tile);
                //     self.update_tile(y + 1, x + 1, 4);
                // }

                {
                    self.update_tile(y, x, doubled_sliced_tile);
                    self.update_tile(y, x + 1, doubled_sliced_tile);
                }
            }
            // print!("\n");
        }
    }

    fn draw_square(&mut self, row: i32, col: i32, width: i32, value: i32) {
        for i in 0..width {
            let row = i + row;

            for j in 0..width {
                let col = j + col;

                self.update_tile(row, col, value);
            }
        }
    }

    fn draw_timing_mark(&mut self, orientation: TimingMark) {
        let start = POSITION_MARKER_SIZE;
        let end = self.size() - POSITION_MARKER_SIZE;

        for i in start..end {
            let row = match orientation {
                TimingMark::Horiztonal => POSITION_MARKER_SIZE - 2,
                TimingMark::Vertical => i,
            };
            let col = match orientation {
                TimingMark::Horiztonal => i,
                TimingMark::Vertical => POSITION_MARKER_SIZE - 2,
            };

            let tile = Tile::new(
                match i % 2 {
                    0 => 1,
                    _ => 0,
                },
                row,
                col,
            );

            match orientation {
                TimingMark::Horiztonal => {
                    self.tiles
                        .insert(Position::new(POSITION_MARKER_SIZE - 2, i), tile);
                }
                TimingMark::Vertical => {
                    self.tiles
                        .insert(Position::new(i, POSITION_MARKER_SIZE - 2), tile);
                }
            }
        }
    }

    fn draw_timing_marks(&mut self) {
        self.draw_timing_mark(TimingMark::Horiztonal);
        self.draw_timing_mark(TimingMark::Vertical);
    }

    fn draw_alignment_mark(&mut self, x: i32, y: i32) {
        self.draw_square(x, y, 5, 1);
        self.draw_square(x + 1, y + 1, 3, 0);
        self.draw_square(x + 2, y + 2, 1, 1);
    }

    fn draw_alignment_marks(&mut self) {
        if self.version <= 1 {
            return;
        }

        self.draw_alignment_mark(self.size() - 9, self.size() - 9);
    }

    fn draw_position_marks(&mut self) {
        let size = self.size();

        // top left identifying mark
        self.draw_square(0, 0, POSITION_MARKER_SIZE, 0);
        self.draw_square(0, 0, 7, 1);
        self.draw_square(1, 1, 5, 0);
        self.draw_square(2, 2, 3, 1);
        // top right identifying mark
        self.draw_square(self.size() - 8, 0, POSITION_MARKER_SIZE, 0);
        self.draw_square(self.size() - 7, 0, 7, 1);
        self.draw_square(self.size() - 6, 1, 5, 0);
        self.draw_square(self.size() - 5, 2, 3, 1);
        // bottom left identifying mark
        self.draw_square(
            0,
            self.size() - POSITION_MARKER_SIZE,
            POSITION_MARKER_SIZE,
            0,
        );
        self.draw_square(0, size - 7, 7, 1);
        self.draw_square(1, size - 6, 5, 0);
        self.draw_square(2, size - 5, 3, 1);
        // last dot near bottom left
        self.update_tile(size - POSITION_MARKER_SIZE, POSITION_MARKER_SIZE, 1);
    }

    fn draw_format_info(&mut self) {
        let size = self.size();

        // vertical
        let top_y = (0..8).filter(|i| *i != 6);
        let bottom_y = (size - 7)..size;

        for y in top_y.chain(bottom_y) {
            self.update_tile(y, 8, 7);
        }

        // horizontal
        let left_x = (0..9).filter(|i| *i != 6);
        let right_x = (size - 8)..size;

        for x in left_x.chain(right_x) {
            self.update_tile(8, x, 6);
        }
    }
}

impl std::fmt::Display for QRBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // draw board contents
        for i in 0..self.size() {
            for j in 0..self.size() {
                let board_value = match self.get_tile(i, j) {
                    Some(b) => b.get_tile_str(),
                    None => "",
                };

                write!(f, "{}", board_value).unwrap_or_else(|_| return);
            }

            write!(f, " {}\n", i).unwrap_or_else(|_| return);
        }

        Ok(())
    }
}
