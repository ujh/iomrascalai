use core::fmt::{Show, Formatter, FormatError};

#[deriving(Clone, Eq, TotalEq, Hash)]
pub struct Coord {
    pub col: u8,
    pub row: u8,
    board_size: u8
}

impl Coord {
    pub fn new(board_size: u8, col: u8, row: u8) -> Coord {
        Coord {board_size: board_size, col: col, row: row}
    }

    pub fn neighbours(&self) -> Vec<Coord> {
        let mut neighbours = Vec::new();

        for i in range(-1,2) {
            for j in range(-1,2) {
                let (col, row) = (self.col+i as u8, self.row+j as u8);
                let potential_neighbour = Coord::new(self.board_size, col, row);
                if (i == 0 && j !=0) || (i != 0 && j == 0) && (potential_neighbour.is_inside()) {
                    neighbours.push(potential_neighbour);
                }
            }
        }
        neighbours    
    }

    pub fn to_index(&self) -> uint {
        (self.col as uint-1 + (self.row as uint-1)*self.board_size as uint)
    }

    pub fn is_inside(&self) -> bool {
        1 <= self.col && self.col <= self.board_size && 1 <= self.row && self.row <= self.board_size
    }
}

impl Show for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        let s = format!("{}, {}", self.col, self.row);
        s.fmt(f)
    }
}
