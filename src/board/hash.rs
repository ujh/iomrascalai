use std::rand::random;
use board::move::{Move, Play};
use board::{Empty, Black, White};

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

    pub fn init_hash(&self) -> u64 {
        let mut init_hash = 0;

        for i in range(0, self.table.len()/3) {       // We xor together all the hashes corresponding to the Empty color
            init_hash ^= *self.table.get(i)
        }

        init_hash
    }

    pub fn add_stone_to_hash(&self, hash: u64, move: &Move) -> u64 {
        hash ^ self.get_hash_for(&Play(Empty, move.coords().col, move.coords().row)) ^ self.get_hash_for(move)    
    }

    pub fn remove_stone_from_hash(&self, hash: u64, move: &Move) -> u64 {
        // As A^B == B^A, removing or adding is the same operation. This method is only added to express intent.
        self.add_stone_to_hash(hash, move)
    }

    fn get_hash_for(&self, move: &Move) -> u64 {
        let color_as_index = match move.color() {
            Empty => 0,
            Black => 1,
            White => 2
        };

        *self.table.get(color_as_index*self.size as uint + (move.coords().row-1) as uint * self.size as uint + move.coords().col as uint - 1)
    }
}