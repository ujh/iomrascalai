use core::fmt::{Show, Formatter, FormatError};
use std::cmp::Eq;

#[deriving(Clone, Hash, PartialEq, Eq)]
pub struct Coord {
    pub col: u8,
    pub row: u8
}

impl Coord {
    pub fn new(col: u8, row: u8) -> Coord {
        Coord {col: col, row: row}
    }

    pub fn from_index(id: uint, board_size: u8) -> Coord {
        Coord {col: (id%board_size as uint + 1) as u8, row: (id/board_size as uint + 1) as u8}
    }

    pub fn neighbours(&self, board_size: u8) -> Vec<Coord> {
        let mut neighbours = Vec::new();

        for i in range(-1,2) {
            for j in range(-1,2) {
                let (col, row) = (self.col+i as u8, self.row+j as u8);
                let potential_neighbour = Coord::new(col, row);
                if ((i == 0 && j !=0) || (i != 0 && j == 0)) && (potential_neighbour.is_inside(board_size)) {
                    neighbours.push(potential_neighbour);
                }
            }
        }
        neighbours    
    }

    pub fn to_index(&self, board_size: u8) -> uint {
        (self.col as uint-1 + (self.row as uint-1)*board_size as uint)
    }

    pub fn is_inside(&self, board_size: u8) -> bool {
        1 <= self.col && self.col <= board_size && 1 <= self.row && self.row <= board_size
    }

    pub fn from_gtp(gtp_vertex: &str) -> Coord {
        let lower_case_vertex: String = gtp_vertex.chars().map(|c| c.to_lowercase()).collect();
        let col = lower_case_vertex.as_slice().char_at(0) as u8 - 'a' as u8;
        let row = from_str::<u8>(lower_case_vertex.as_slice().slice(1, lower_case_vertex.len())).expect("you must enter a valid coord (1 < c < 256)");

        Coord::new(col, row)
    }

    pub fn to_gtp(&self) -> String {
        let gtp_col = ('A' as u8 + self.col - 1) as char;
        let gtp_row = self.row;

        String::from_str(gtp_col.to_str().as_slice()).append(gtp_row.to_str().as_slice()) 
    }
}

impl Show for Coord {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        let s = format!("{}, {}", self.col, self.row);
        s.fmt(f)
    }
}
