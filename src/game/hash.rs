/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot                                        *
 * Copyright 2015 Urban Hafner                                          *
 *                                                                      *
 * This file is part of Iomrascálaí.                                    *
 *                                                                      *
 * Iomrascálaí is free software: you can redistribute it and/or modify  *
 * it under the terms of the GNU General Public License as published by *
 * the Free Software Foundation, either version 3 of the License, or    *
 * (at your option) any later version.                                  *
 *                                                                      *
 * Iomrascálaí is distributed in the hope that it will be useful,       *
 * but WITHOUT ANY WARRANTY; without even the implied warranty of       *
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the        *
 * GNU General Public License for more details.                         *
 *                                                                      *
 * You should have received a copy of the GNU General Public License    *
 * along with Iomrascálaí.  If not, see <http://www.gnu.org/licenses/>. *
 *                                                                      *
 ************************************************************************/

use board::Black;
use board::Empty;
use board::Move;
use board::Play;
use board::White;

use std::rand::random;

#[derive(Show)]
pub struct ZobristHashTable {
    table: Vec<u64>,
    size : u8
}

impl ZobristHashTable {
    pub fn new(size: u8) -> ZobristHashTable {
        let mut table = Vec::new();

        for _ in range(0i8, 3) {
            for _ in range(0, (size as uint)*(size as uint)) {
                table.push(random::<u64>());
            }
        }

        ZobristHashTable {table: table, size: size}
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn init_hash(&self) -> u64 {
        let mut init_hash = 0;

        for i in range(0, self.table.len()/3) {       // We xor together all the hashes corresponding to the Empty color
            init_hash ^= self.table[i]
        }

        init_hash
    }

    pub fn add_stone_to_hash(&self, hash: u64, m: &Move) -> u64 {
        hash ^ self.get_hash_for(&Play(Empty, m.coords().col, m.coords().row)) ^ self.get_hash_for(m)
    }

    pub fn remove_stone_from_hash(&self, hash: u64, m: &Move) -> u64 {
        // As A^B == B^A, removing or adding is the same operation. This method is only added to express intent.
        self.add_stone_to_hash(hash, m)
    }

    fn get_hash_for(&self, m: &Move) -> u64 {
        let color_as_index = match *m.color() {
            Empty => 0,
            Black => 1,
            White => 2
        };

        self.table[color_as_index*self.size as uint + (m.coords().row-1) as uint * self.size as uint + m.coords().col as uint - 1]
    }
}
