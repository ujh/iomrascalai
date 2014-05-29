use rand::random;
use board::coord::Coord;
use board::{Color, Empty, Black, White};

#[deriving(Clone)]
pub struct ZobristHashTable {
    table: Vec<u64>,
    size : u8
}

impl ZobristHashTable {
    pub fn new(size: u8) -> ZobristHashTable {
        let mut table = Vec::new();

        for _ in range(0, 3) {
            for _ in range(0, (size as uint)*(size as uint)) {
                table.push(random::<u64>());
            }
        }

        ZobristHashTable {table: table, size: size}
    }

    pub fn add_move_to_hash(&self, hash: u64, color: Color, coord: Coord) -> u64 {
        hash ^ self.get_hash_for(color, coord)
    }

    fn get_hash_for(&self, color: Color, coord: Coord) -> u64 {
        let color_as_index = match color {
            Empty => 0,
            Black => 1,
            White => 2
        };

        *self.table.get(color_as_index*self.size as uint + (coord.row-1) as uint * self.size as uint + coord.col as uint - 1)
    }
}