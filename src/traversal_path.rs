use std::collections::HashMap;

pub const QR_VERSION: i32 = 2;
pub const QR_SIZE: i32 = (4 * QR_VERSION) + 17;

type Cell = (i32, i32);

pub fn traverse_qr_code(version: i32) -> Vec<Cell> {
    let mut traversal_path = Vec::<Cell>::new();
    let mut direction = -1;
    let mut column = QR_SIZE - 1;

    while column >= 0 {
        let mut row = if direction == -1 { QR_SIZE - 1 } else { 0 };
        while (direction == -1 && row >= 0) || (direction == 1 && row < QR_SIZE) {
            if !is_function_pattern(version, row, column) {
                traversal_path.push((row, column));
            }
            row += direction;
        }
        column -= 2;
        direction *= -1;
    }

    return traversal_path;
}

fn is_function_pattern(version: i32, row: i32, col: i32) -> bool {
    // finder patterns are 9 pixels wide
    let finder_size: i32 = 9;

    // finder patterns
    if row < finder_size && col < finder_size {
        return true;
    }
    if row < finder_size && col >= QR_SIZE - finder_size {
        return true;
    }
    if row >= QR_SIZE - finder_size && col < finder_size {
        return true;
    }

    // timing patterns
    if row == 6 || col == 6 {
        return true;
    }

    // alignment patterns
    if version >= 2 {
        let alignment_locations = get_alignment_pattern_locations(version);
        for loc in alignment_locations {
            let (align_row, align_col) = loc;
            if row >= align_row - 2
                && row <= align_row + 2
                && col >= align_col - 2
                && col <= align_col + 2
            {
                return true;
            }
        }
    }

    return false;
}

fn get_alignment_pattern_locations(version: i32) -> Vec<Cell> {
    let mut locations_map = HashMap::<i32, Vec<i32>>::new();
    // versions 2 to 5
    for v in 2..=5 {
        locations_map.insert(v, vec![6, 18 + ((v - 2) * 4)]);
    }
    // TODO: version 6 ++
    if version >= 6 {
        todo!("Version 6 and up are not yet implemented.");
    }

    let locations = locations_map[&version].clone();
    let mut result = Vec::<Cell>::new();
    let length = locations.len();

    for row in 0..length {
        for col in 0..length {
            result.push((row as i32, col as i32));
        }
    }

    return result;
}
