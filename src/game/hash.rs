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
use board::Coord;
use board::Empty;
use board::Move;
use board::Play;
use board::White;

use std::rand::random;

#[derive(Debug)]
pub struct ZobristHashTable {
    black: Vec<u64>,
    size:  u8,
    white: Vec<u64>,
}

impl ZobristHashTable {
    pub fn new(size: u8) -> ZobristHashTable {
        let mut black = Vec::new();
        let mut white = Vec::new();
        for _ in Coord::for_board_size(size) {
            black.push(random::<u64>());
            white.push(random::<u64>());
        }
        ZobristHashTable {
            black: black,
            size:  size,
            white: white,
        }
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn init_hash(&self) -> u64 {
        0
    }

    pub fn add_stone_to_hash(&self, hash: u64, m: &Move) -> u64 {
        hash ^ self.get_hash_for(m)
    }

    pub fn remove_stone_from_hash(&self, hash: u64, m: &Move) -> u64 {
        self.add_stone_to_hash(hash, m)
    }

    fn get_hash_for(&self, m: &Move) -> u64 {
        match *m.color() {
            Empty => 0,
            Black => { self.black[m.coord().to_index(self.size)]},
            White => { self.white[m.coord().to_index(self.size)]},
        }
    }
}
