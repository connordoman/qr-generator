use super::board::Position;

pub struct QRByte {
    start: Position,
    x: i32,
    y: i32,
    len: u8,
}

impl QRByte {
    pub fn new(x: i32, y: i32) -> Self {
        QRByte {
            start: Position::new(y, x),
            x,
            y,
            len: 8,
        }
    }
}
