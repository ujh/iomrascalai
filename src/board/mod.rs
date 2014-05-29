/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
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
use std::vec::Vec;
use board::chain::Chain;
use board::coord::Coord;
use board::hash::ZobristHashTable;

mod board_test;
mod coord_test;
mod chain_test;

mod coord;
mod chain;
pub mod hash;

#[deriving(Show, Eq)]
pub enum IllegalMove {
    PlayOutOfBoard,
    SuicidePlay,
    IntersectionNotEmpty,
    SamePlayerPlayedTwice,
    GameAlreadyOver,
    SuperKoRuleBroken
}

#[deriving(Clone, Show, Eq)]
pub enum Color {
    White,
    Black,
    Empty
}

#[deriving(Clone, Show, Eq)]
pub enum Ruleset {
    TrompTaylor,
    Minimal
}

impl Color {
    fn opposite(&self) -> Color {
        match *self {
            White => Black,
            Black => White,
            Empty => Empty
        }
    }
}

pub struct Board<'a> {
    komi: f32,
    size: u8,
    board: Vec<uint>,
    chains: Vec<Chain>,
    ruleset: Ruleset,
    previous_player: Color,
    consecutive_passes: u8,
    zobrist_base_table: &'a ZobristHashTable,
    previous_boards_hashes: Vec<u64>
}

impl<'a> Clone for Board<'a> {
    fn clone(&self) -> Board<'a> {
        Board {
            komi                  : self.komi,
            size                  : self.size,
            board                 : self.board.clone(),
            chains                : self.chains.clone(),
            ruleset               : self.ruleset,
            previous_player       : self.previous_player,
            consecutive_passes    : self.consecutive_passes,
            zobrist_base_table    : self.zobrist_base_table,
            previous_boards_hashes: self.previous_boards_hashes.clone()
        }
    }
}

impl<'a> Board<'a> {
    pub fn new(size: uint, komi: f32, ruleset: Ruleset, zobrist_base_table: &'a ZobristHashTable) -> Board<'a> {
        if ruleset == TrompTaylor && size != 19 {fail!("You can only play on 19*19 in Tromp Taylor Rules");}

        Board {
            komi: komi,
            size: size as u8,
            board: Vec::from_fn(size*size, |_| 0),
            chains: vec!(Chain::new(0, Empty)),
            ruleset: ruleset,
            previous_player: White,
            consecutive_passes: 0,
            zobrist_base_table: zobrist_base_table,
            previous_boards_hashes: vec!(zobrist_base_table.init_hash())
        }
    }

    // Note: This method uses 1-1 as the origin point, not 0-0. 19-19 is a valid coordinate in a 19-sized board, while 0-0 is not.
    //       this is done because I think it makes more sense in the context of go. (Least surprise principle, etc...)
    pub fn get(&self, col: u8, row: u8) -> Color {
        self.get_coord(Coord::new(col, row))
    }

    fn get_coord(&self, c: Coord) -> Color {
        if c.is_inside(self.size) {
            self.get_chain(c).color
        } else {
            fail!("You have requested a stone outside of the board");
        }
    }

    pub fn get_chain<'a>(&'a self, c: Coord) -> &'a Chain {
        if c.is_inside(self.size) {
            let chain_id = *self.board.get(c.to_index(self.size));
            self.chains.get(chain_id)
        } else {
            fail!("You have requested a chain outside of the board");
        }
    }

    // Note: Same as get(), the board is indexed starting at 1-1
    pub fn play(&self, color: Color, move: Option<(u8, u8)>) -> Result<Board<'a>, IllegalMove> {
        if self.is_game_over() && self.ruleset == TrompTaylor {
            return Err(GameAlreadyOver);
        }

        if move.is_none() {
            let mut new_board = self.clone();
            new_board.consecutive_passes += 1;
            return Ok(new_board);
        }

        let new_coords = match move {
            Some((col, row)) => Coord::new(col, row),
            None             => unreachable!()
        };

        if new_coords.is_inside(self.size) {
            if self.get_coord(new_coords) != Empty {
                return Err(IntersectionNotEmpty);
            }
        } else {
            return Err(PlayOutOfBoard);
        }

        if self.ruleset != Minimal && self.previous_player == color {
            return Err(SamePlayerPlayedTwice);
        }

        let mut new_board = self.clone();
        new_board.consecutive_passes = 0;

        new_board.previous_player  = color;

        let friend_neigh_chains_id = self.find_neighbouring_friendly_chains_ids(new_coords, color);

        /*
         * If there is 0 friendly neighbouring chain, we create one, and assign the coord played to that new chain.
         * If there is 1, we assign the stone to that chain.
         * If there are more, we assign the stone to one chain, then merge the others into that chain, then remove the old chains from
         * board.chains, then we lower by 1 the ids of all stones with chain ids higher than the removed chains,
         * and finally we reassign the correct chain_id to each stone in the final chain.
        */
        match friend_neigh_chains_id.len() {
            0 => new_board.create_new_chain(color, new_coords),
            1 => {
                let final_chain_id = *friend_neigh_chains_id.get(0);
                new_board.add_coord_to_chain(new_coords, final_chain_id);
            },
            _ => {
                // Note: We know that friend_neigh_chains_id is sorted, so whatever chains we remove,
                // we know that the id of the final_chain is still valid.
                let final_chain_id        = *friend_neigh_chains_id.get(0);
                let mut nb_removed_chains = 0;

                // We assign the stone to the final chain
                new_board.add_coord_to_chain(new_coords, final_chain_id);

                for &other_chain_old_id in friend_neigh_chains_id.slice(1, friend_neigh_chains_id.len()).iter() {
                    // The ids stored in friend_neigh_chains_id may be out of date since we remove chains from new_board.chains
                    // These id is the correct one at this step of the removals
                    let other_chain_id = other_chain_old_id - nb_removed_chains;

                    // We merge the other chain into the final chain.
                    let other_chain = new_board.chains.get(other_chain_id).clone();
                    new_board.chains.get_mut(final_chain_id).merge(&other_chain);

                    // We remove the old chain.
                    new_board.chains.remove(other_chain_id);

                    // We update the ids inside the chains
                    new_board.update_chains_ids_after_id(other_chain_id);
                    
                    nb_removed_chains += 1;
                }

                new_board.update_board_ids_after_id(final_chain_id);
            }
        }

        // We then update the libs of all chains of the opposite color
        new_board.update_chains_libs_of(color.opposite());

        let adv_stones_removed = new_board.remove_adv_chains_with_no_libs_close_to(new_coords, color.opposite());
        new_board.update_all();

        let final_chain_id = new_board.get_chain(new_coords).id;
        new_board.update_libs(final_chain_id);

        let mut friend_stones_removed = Vec::new(); // This is only useful is suicide is legal.

        if adv_stones_removed.len() > 0 {
            // We could only re-check the libs of the neighbours of the neighbours of new_coords, but this will do atm.
            // TODO: Restrict the chains updated to the ones that might have been impacted by the last move.
            for i in range(1, new_board.chains.len()) {
                new_board.update_libs(i);
            }
        } else if new_board.get_chain(new_coords).libs == 0 {
            match new_board.ruleset {
                TrompTaylor => {
                    friend_stones_removed.push_all(new_board.get_chain(new_coords).coords().as_slice());
                    let to_remove_id = new_board.get_chain(new_coords).id;
                    new_board.remove_chain(to_remove_id);
                    new_board.update_all_after_id(to_remove_id);
                },
                _           => return Err(SuicidePlay)
            }
        }

        // We update the hash with the changes to the board, and add it to the list of hashes before returning.
        let hash = new_board.compute_hash(color, new_coords, &adv_stones_removed, &friend_stones_removed);

        if new_board.previous_boards_hashes.contains(&hash) {
            return Err(SuperKoRuleBroken)
        }

        new_board.previous_boards_hashes.push(hash);

        Ok(new_board)
    }

    fn find_neighbouring_friendly_chains_ids(&self, c: Coord, color: Color) -> Vec<uint> {
        let mut friend_neigh_chains_id: Vec<uint> = c.neighbours(self.size)
                  .iter()
                  .filter(|&c| c.is_inside(self.size) && self.get_coord(*c) == color)
                  .map(|&c| self.get_chain(c).id)
                  .collect();

        // We need to sort the chain by ascending id so that later we know that friend_neigh_chains_id[0] has the lowest id.
        // It also helps with keeping track of the ids of the chain yet-to-merge as their ids will always decrease by nb of chains
        // merged before them.
        friend_neigh_chains_id.sort();
        friend_neigh_chains_id.dedup();
        friend_neigh_chains_id
    }

    fn update_libs(&mut self, chain_id: uint) {
        let libs = self.chains.get(chain_id).coords()
                                            .iter()
                                            .fold(Vec::new(), |mut acc, c| {
                                                for &n in c.neighbours(self.size).iter() {
                                                    if n.is_inside(self.size) && self.get_coord(n) == Empty && !acc.contains(&n) {
                                                        acc.push(n);
                                                    }
                                                }
                                                acc
                                            }).len();
        self.chains.get_mut(chain_id).libs = libs;
    }

    fn update_chains_libs_of(&mut self, color: Color) {
        let mut adv_chains_ids: Vec<uint> = self.chains
                  .iter()
                  .filter(|c| c.color == color)
                  .map(|c| c.id)
                  .collect();

        adv_chains_ids.sort();
        adv_chains_ids.dedup();

        for &id in adv_chains_ids.iter() {
            self.update_libs(id);
        }
    }

    fn update_board_ids_after_id(&mut self, id: uint) {
        for i in range(id, self.chains.len()) {
            for &coord in self.chains.get(i).coords().iter() {
                *self.board.get_mut(coord.to_index(self.size)) = i;
            }
        }
    }

    fn update_chains_ids_after_id(&mut self, removed_chain_id: uint) {
        for i in range(removed_chain_id, self.chains.len()) {
            self.chains.get_mut(i).id = i;
        }
    }

    fn update_all_after_id(&mut self, id: uint) {
        self.update_board_ids_after_id(id);
        self.update_chains_ids_after_id(id);
    }

    fn update_all(&mut self) {
        self.update_all_after_id(0);
    }

    // Returns a vector of the coords where stones where removed.
    fn remove_adv_chains_with_no_libs_close_to(&mut self, close_to: Coord, color: Color) -> Vec<Coord> {
        let coords_to_remove = close_to.neighbours(self.size).iter()
                                      .map(|&coord| self.get_chain(coord))
                                      .filter(|chain| chain.libs == 0 && chain.color == color)
                                      .fold(Vec::new(), |acc, chain| acc.append(chain.coords().as_slice()));

        let mut chain_to_remove_ids: Vec<uint> = close_to.neighbours(self.size)
                                                         .iter()
                                                         .map(|&coord| self.get_chain(coord))
                                                         .filter(|chain| chain.libs == 0 && chain.color == color)
                                                         .map(|chain| chain.id)
                                                         .collect();

        // We need to sort first to make sure dedup removes all duplicates.                                                 
        chain_to_remove_ids.sort();
        chain_to_remove_ids.dedup();

        for &id in chain_to_remove_ids.iter() {
            self.remove_chain(id);
        }

        coords_to_remove
    }

    fn remove_chain(&mut self, id: uint) {
        let coords_to_remove = self.chains.get(id).coords().clone();

        for &coord in coords_to_remove.iter() {
            self.remove_stone(coord)
        }

        self.chains.remove(id);
        self.update_chains_ids_after_id(id);
    }

    fn create_new_chain(&mut self, color: Color, init_coord: Coord) {
        let new_chain_id    = self.chains.len();
        let mut new_chain   = Chain::new(new_chain_id, color);
        new_chain.add_stone(init_coord);
        self.chains.push(new_chain);
        *self.board.get_mut(init_coord.to_index(self.size)) = new_chain_id;
        self.update_libs(new_chain_id);
    }

    fn add_coord_to_chain(&mut self, coord: Coord, chain_id: uint) {
        self.chains.get_mut(chain_id).add_stone(coord);
       *self.board.get_mut(coord.to_index(self.size)) = chain_id;
    }

    fn remove_stone(&mut self, c: Coord) {
        *self.board.get_mut(c.to_index(self.size)) = 0;
    }

    fn compute_hash(&self, current_color: Color, new_coords: Coord, adv_stones_removed: &Vec<Coord>, friend_stones_removed: &Vec<Coord>) -> u64 {
        let mut hash = self.zobrist_base_table.add_stone_to_hash(*self.previous_boards_hashes.last().unwrap(), current_color, new_coords);

        for &coord in adv_stones_removed.iter() {
            hash = self.zobrist_base_table.remove_stone_from_hash(hash, current_color.opposite(), coord);
        }

        for &coord in friend_stones_removed.iter() {
            hash = self.zobrist_base_table.remove_stone_from_hash(hash, current_color, coord);
        }

        hash
    }

    pub fn is_game_over(&self) -> bool {
        self.consecutive_passes == 2
    }

    pub fn komi(&self) -> f32 {
        self.komi
    }

    pub fn ruleset(&self) -> Ruleset {
        self.ruleset
    }

    pub fn hash(&self) -> u64 {
        *self.previous_boards_hashes.last().unwrap()
    }

    pub fn show(&self) {
        println!("komi: {}", self.komi());

        // First we print the board
        for row in range(1u8, self.size+1).rev() {

            // Prints the row number
            print!("{:2} ", row);

            // Prints the actual row
            for col in range(1u8, self.size+1) {
                let current_coords = Coord::new(col, row);

                if self.get_coord(current_coords) == Empty {
                    let hoshis = &[4u8,10,16];
                    if   hoshis.contains(&row) && hoshis.contains(&col) {print!("+ ")}
                    else                                                {print!(". ")}
                } else if self.get_coord(current_coords) == White {print!("O ")}
                  else if self.get_coord(current_coords) == Black {print!("X ")}
            }
            println!("");
        }

        // Then we print the col numbers under the board
        print!("{:3}", "");
        for col in range(1, self.size+1) {
            print!("{:<2}", col);
        }

        println!("");
    }

    pub fn show_chains(&self) {
        for c in self.chains.iter() {
            println!("{}", c.show());
        }
    }
}
