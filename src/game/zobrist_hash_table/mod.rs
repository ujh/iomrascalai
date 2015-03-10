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
use board::Board;
use board::Coord;
use board::Empty;
use board::Move;
use board::Play;
use board::White;

use rand::random;

#[derive(Clone)]
pub struct ZobristHashTable {
    black: Vec<u64>,
    hashes: Vec<u64>,
    size: u8,
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
            hashes: vec!(0),
            size: size,
            white: white,
        }
    }

    pub fn check_and_update_super_ko(&mut self, m: &Move, b: &Board) -> Result<(),()> {
        let hash = self.compute_hash(m, b);
        if self.hashes.contains(&hash) {
            Err(())
        } else {
            self.hashes.push(hash);
            Ok(())
        }
    }

    fn compute_hash(&self, m: &Move, b: &Board) -> u64 {
        let mut hash = self.hashes[self.hashes.len()-1];
        hash = self.change_hash(hash, m);
        for coord in b.adv_stones_removed() {
            hash = self.change_hash(hash, &Play(m.color().opposite(), coord.col, coord.row));
        }
        for coord in b.friend_stones_removed() {
            hash = self.change_hash(hash, &Play(*m.color(), coord.col, coord.row));
        }
        hash
    }

    fn change_hash(&self, hash: u64, m: &Move) -> u64 {
        hash ^ self.hash_for(m)
    }

    fn hash_for(&self, m: &Move) -> u64 {
        match *m.color() {
            Empty => 0,
            Black => self.black[m.coord().to_index(self.size)],
            White => self.white[m.coord().to_index(self.size)]
        }
    }
}
