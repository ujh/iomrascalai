use core::fmt::{Show, Formatter, FormatError};

#[deriving(Clone, Eq, TotalEq, Hash)]
pub struct Coord {
    pub col: u8,
    pub row: u8
}

impl Coord {
    pub fn new(col: u8, row: u8) -> Coord {
        Coord {col: col, row: row}
    }

    pub fn neighbours(&self) -> Vec<Coord> {
        let mut neighbours = Vec::new();

        for i in range(-1,2) {
            for j in range(-1,2) {
                if (i == 0 && j !=0) || (i != 0 && j == 0) {
                    let (col, row) = (self.col+i as u8, self.row+j as u8);
                    neighbours.push(Coord::new(col, row))
                }
            }
        }
        neighbours    
    }

    pub fn to_index(&self, size:u8) -> uint {
        (self.col as uint-1 + (self.row as uint-1)*size as uint)
    }
}

impl Show for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        let s = format!("{}, {}", self.col, self.row);
        s.fmt(f)
    }
}
