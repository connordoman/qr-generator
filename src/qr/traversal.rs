use std::{
    cmp::{max, min},
    collections::HashMap,
};

use super::board::{QRBoard, Tile};

fn toggled(num: i32) -> i32 {
    let num = max(min(1, num), -1);

    if num == 1 {
        -1
    } else {
        1
    }
}

pub fn traverse_board(board: &mut QRBoard) -> Vec<Tile> {
    let mut result = Vec::new();

    let size = board.size() - 1;

    // board.update_tile(size - 1, size, 4);
    // board.update_tile(size - 1, size - 1, 4);
    // board.update_tile(size, size, 4);
    // board.update_tile(size, size - 1, 4);

    let encoding = [
        board.get_tile(size - 1, size),
        board.get_tile(size - 1, size - 1),
        board.get_tile(size, size),
        board.get_tile(size, size - 1),
    ]
    .map(|v| match v {
        Some(t) => t.value,
        None => 0,
    });

    for (index, bit) in encoding.iter().enumerate() {
        print!("{} ", bit);

        if index % 2 == 1 {
            print!("\n");
        }
    }

    let mut data: Vec<Vec<i32>> = Vec::new();

    let mut byte_stack: Vec<i32> = Vec::new();

    //// traverse in full
    let mut direction = -1;

    for col in (0..size + 1).step_by(2).rev() {
        // if col == 6 {
        //     continue;
        // }

        let col = if col <= 6 { col - 1 } else { col };
        let col_other = col - 1;

        let mut row = if col > 8 { size + 1 } else { size - 7 };

        println!(
            "column: {} & {}, row + direction: {}",
            col,
            col_other,
            row + direction
        );

        while row + direction >= 0 && row + direction < size + 1 {
            row += direction;

            println!("row: {}", row);

            let val = if direction == 1 { 3 } else { 2 };

            // println!(
            //     "row: {}, direction: {}, direction == 1: {}, val: {}",
            //     row,
            //     direction,
            //     direction == 1,
            //     val
            // );

            board.update_tile(row, col, val);
            board.update_tile(row, col_other, val);

            // need to break if out of runway

            // 1 = down, -1 = up
            let will_intersect_vertically = col < 9 || col > size - 9;

            let intersects_top = row == 0 || (will_intersect_vertically && row < 10);

            let intersects_bottom = row == size || (will_intersect_vertically && row > size - 9);

            // top right
            // let intersects_right = col > size - 8 && col <= size && row < 9;

            // // top left, bottom left
            // let intersects_left = col < 9 && col >= 0 && (row < 9 || row >= (size - 7));

            // println!("left: {}, right: {}", intersects_left, intersects_right);
            if direction == -1 && intersects_top
            // || intersects_bottom
            {
                direction = 1;
                println!("intersects top. direction now: {}", direction);
                // continue;
                break;
            }

            if direction == 1 && intersects_bottom
            // || intersects_bottom
            {
                direction = -1;
                println!("intersects bottom. direction now: {}", direction);
                // continue;
                break;
            }

            let tile = match board.get_tile(row, col) {
                Some(t) => t,
                None => &Tile::new(0, row, col),
            };
            let tile_other = match board.get_tile(row, col_other) {
                Some(t) => t,
                None => &Tile::new(0, row, col),
            };

            // encoding nibble
            if data.len() == 0 && byte_stack.len() == 4 {
                data.push(byte_stack.clone());

                // board.update_tile(row + 1, col, 4);
                // board.update_tile(row + 1, col_other, 4);
                // board.update_tile(row, col, 4);

                byte_stack = Vec::new();
            } else if byte_stack.len() == 8 {
                data.push(byte_stack.clone());

                byte_stack = Vec::new();
            }

            if !tile.visited {
                byte_stack.push(tile.value);
            }

            if !tile_other.visited {
                byte_stack.push(tile_other.value);
            }
        }
        // for row in (0..=size).rev() {
        //     // board.update_tile(row, col, 3);
        //     // board.update_tile(row, col_other, 3);

        //     println!("{}, {}", col, row);
        // }
    }

    println!("{:?}", data);

    return result;
}

pub fn traverse_board_2(board: QRBoard) -> Vec<Tile> {
    let mut traversal_path = Vec::<Tile>::new();

    let mut direction = -1;
    let mut column = board.size() - 1;

    let size = board.size();

    fn is_function_pattern(board: &QRBoard, row: i32, col: i32) -> bool {
        let size = board.size();

        // finder patterns
        if row < 9 && col < 9 {
            return true;
        };
        if row > size - 9 && col < 9 {
            return true;
        };
        if row < 9 && col > size - 9 {
            return true;
        };

        // timing patterns
        if row == 6 || col == 6 {
            return true;
        }

        // alignment patterns
        if board.version >= 2 {
            let alignment_locations = get_alignment_pattern_locations(board);
            for i in 0..alignment_locations.len() {
                let tile = alignment_locations[i];
                if row >= tile.row - 2
                    && row < tile.row + 2
                    && col >= tile.col - 2
                    && col <= tile.col + 2
                {
                    return true;
                }
            }
        }

        return false;
    }

    fn get_alignment_pattern_locations(board: &QRBoard) -> Vec<Tile> {
        let mut locations_map = HashMap::<i32, Vec<i32>>::new();

        for ver in 2..6 {
            locations_map.insert(ver, vec![6, (18 + (4 * (ver - 2)))]);
        }

        match locations_map.get(&board.version) {
            Some(mark) => {
                let mut result = Vec::<Tile>::new();
                for i in 0..mark.len() {
                    for j in 0..mark.len() {
                        let t = Tile::new(2, i as i32, j as i32);

                        result.push(t);
                    }
                }
                return result;
            }
            None => return Vec::new(),
        };
    }

    while column >= 0 {
        let mut row = if direction == -1 { size - 1 } else { 0 };

        while (direction == -1 && row >= 0) || (direction == 1 && row < size) {
            if !is_function_pattern(&board, row, column) {
                match board.get_tile(row, column) {
                    Some(b) => traversal_path.push(*b),
                    None => (),
                };
            }
            row += direction;
        }
        column -= 2;
        direction += -1;
    }

    return traversal_path;
}
