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
use board::chain::Chain;
use board::coord::Coord;
use board::hash::ZobristHashTable;
pub use board::movement::Move;
pub use board::movement::Pass;
pub use board::movement::Play;
use ruleset::Ruleset;

use std::collections::HashSet;
use std::rc::Rc;
use std::vec::Vec;

mod board_test;
mod chain_test;

mod chain;
pub mod coord;
pub mod hash;
pub mod movement;

#[deriving(Show, Eq, PartialEq)]
pub enum IllegalMove {
    PlayOutOfBoard,
    SuicidePlay,
    IntersectionNotEmpty,
    SamePlayerPlayedTwice,
    GameAlreadyOver,
    SuperKoRuleBroken
}

#[deriving(Clone, Show, Eq, PartialEq, Hash)]
pub enum Color {
    White,
    Black,
    Empty
}

impl Color {
    pub fn opposite(&self) -> Color {
        match *self {
            White => Black,
            Black => White,
            Empty => Empty
        }
    }

    pub fn from_gtp(gtp_color: &str) -> Color {
        let lower_gtp_color: String = gtp_color.chars().map(|c| c.to_lowercase()).collect();
        match lower_gtp_color.as_slice() {
            "w" | "white" => White,
            "b" | "black" => Black,
            err           => panic!("Can't read the GTP color: {}", err)
        }
    }
}

#[deriving(Show)]
pub struct Board<'a> {
    size: u8,
    board: Vec<uint>,
    chains: Vec<Chain>,
    ruleset: Ruleset,
    previous_player: Color,
    consecutive_passes: u8,
    zobrist_base_table: Rc<ZobristHashTable>,
    previous_boards_hashes: Vec<u64>
}

impl<'a> Clone for Board<'a> {
    fn clone(&self) -> Board<'a> {
        Board {
            size                  : self.size,
            board                 : self.board.clone(),
            chains                : self.chains.clone(),
            ruleset               : self.ruleset,
            previous_player       : self.previous_player,
            consecutive_passes    : self.consecutive_passes,
            zobrist_base_table    : self.zobrist_base_table.clone(),
            previous_boards_hashes: self.previous_boards_hashes.clone()
        }
    }
}

impl<'a> Board<'a> {
    pub fn new(size: u8, ruleset: Ruleset, zobrist_base_table: Rc<ZobristHashTable>) -> Board<'a> {
        if size != zobrist_base_table.size() {
            panic!("Different sizes for board and Zobrist hash table!");
        }
        Board {
            size: size,
            board: Vec::from_fn(size as uint*size as uint, |_| 0),
            chains: vec!(Chain::new(0, Empty)),
            ruleset: ruleset,
            previous_player: White,
            consecutive_passes: 0,
            zobrist_base_table: zobrist_base_table.clone(),
            previous_boards_hashes: vec!(zobrist_base_table.init_hash())
        }
    }

    pub fn get_coord(&self, c: Coord) -> Color {
        if c.is_inside(self.size) {
            self.get_chain(c).color
        } else {
            panic!("You have requested a stone outside of the board");
        }
    }

    pub fn get_chain<'a>(&'a self, c: Coord) -> &'a Chain {
        if c.is_inside(self.size) {
            let chain_id = self.board[c.to_index(self.size)];
            &self.chains[chain_id]
        } else {
            panic!("You have requested a chain outside of the board");
        }
    }

    pub fn next_player(&self) -> Color {
        self.previous_player.opposite()
    }

    fn is_same_player(&self, m: &Move) -> bool {
        self.previous_player == m.color()
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let color = self.next_player();
        let mut moves : Vec<Move> = Coord::for_board_size(self.size).iter().map(
            |coord| Play(color, coord.col, coord.row)).filter(
            |m| self.play(*m).is_ok()).collect();
        moves.push(Pass(color));
        moves
    }

    // Note: Same as get(), the board is indexed starting at 1-1
    pub fn play(&self, m: Move) -> Result<Board<'a>, IllegalMove> {
        // We check is the player is trying to play on a finished game (which is illegal in TT rules)
        if self.is_game_over() && !self.ruleset.game_over_play() {
            return Err(GameAlreadyOver);
        }

        // We check that the same player didn't play twice (except in the minimal ruleset, which is useful for tests)
        if self.is_same_player(&m) && !self.ruleset.same_player() {
            return Err(SamePlayerPlayedTwice);
        }

        // Then we check if the player passed
        if m.is_pass() {
            let mut new_board = self.clone();
            new_board.consecutive_passes += 1;
            new_board.previous_player    = m.color();
            return Ok(new_board);
        }

        // We check if the new move is inside the board (and if it is, if there is no stone there)
        if m.coords().is_inside(self.size) {
            if self.get_coord(m.coords()) != Empty {
                return Err(IntersectionNotEmpty);
            }
        } else {
            return Err(PlayOutOfBoard);
        }

        let mut new_board = self.clone();

        new_board.previous_player    = m.color();

        new_board.consecutive_passes = 0;

        new_board.merge_or_create_chain(m);

        // We then update the libs of all chains of the opposite color
        new_board.update_chains_libs_of(m.color().opposite());

        let adv_stones_removed = new_board.remove_adv_chains_with_no_libs_close_to(m);
        new_board.update_all();

        let final_chain_id = new_board.get_chain(m.coords()).id;
        new_board.update_libs(final_chain_id);

        let mut friend_stones_removed = Vec::new(); // This is only useful is suicide is legal.

        if adv_stones_removed.len() > 0 {
            for i in range(1, new_board.chains.len()) {
                new_board.update_libs(i);
            }
        } else if new_board.get_chain(m.coords()).libs == 0 {
            if self.ruleset.suicide_allowed() {
                friend_stones_removed.push_all(new_board.get_chain(m.coords()).coords().as_slice());
                let to_remove_id = new_board.get_chain(m.coords()).id;
                new_board.remove_chain(to_remove_id);
                new_board.update_all_after_id(to_remove_id);
            } else {
                return Err(SuicidePlay)
            }
        }

        // We update the hash with the changes to the board, and add it to the list of hashes before returning.
        let hash = new_board.compute_hash(&m, &adv_stones_removed, &friend_stones_removed);

        if new_board.previous_boards_hashes.contains(&hash) {
            return Err(SuperKoRuleBroken)
        }

        new_board.previous_boards_hashes.push(hash);

        Ok(new_board)
    }

    fn find_neighbouring_friendly_chains_ids(&self, m: Move) -> Vec<uint> {
        let mut friend_neigh_chains_id: Vec<uint> = m.coords().neighbours(self.size)
                  .iter()
                  .filter(|&c| c.is_inside(self.size) && self.get_coord(*c) == m.color())
                  .map(|&c| self.get_chain(c).id)
                  .collect();

        // We need to sort the chain by ascending id so that later we know that friend_neigh_chains_id[0] has the lowest id.
        // It also helps with keeping track of the ids of the chain yet-to-merge as their ids will always decrease by nb of chains
        // merged before them.
        friend_neigh_chains_id.sort();
        friend_neigh_chains_id.dedup();
        friend_neigh_chains_id
    }

    fn merge_or_create_chain(&mut self, m: Move) {
        let friend_neigh_chains_id = self.find_neighbouring_friendly_chains_ids(m);

        /*
         * If there is 0 friendly neighbouring chain, we create one, and assign the coord played to that new chain.
         * If there is 1, we assign the stone to that chain.
         * If there are more, we assign the stone to one chain, then merge the others into that chain, then remove the old chains from
         * board.chains, then we lower by 1 the ids of all stones with chain ids higher than the removed chains,
         * and finally we reassign the correct chain_id to each stone in the final chain.
        */
        match friend_neigh_chains_id.len() {
            0 => self.create_new_chain(m),
            1 => {
                let final_chain_id = friend_neigh_chains_id[0];
                self.add_coord_to_chain(m.coords(), final_chain_id);
            },
            _ => {
                // Note: We know that friend_neigh_chains_id is sorted, so whatever chains we remove,
                // we know that the id of the final_chain is still valid.
                let final_chain_id        = friend_neigh_chains_id[0];
                let mut nb_removed_chains = 0;

                // We assign the stone to the final chain
                self.add_coord_to_chain(m.coords(), final_chain_id);

                for &other_chain_old_id in friend_neigh_chains_id.slice(1, friend_neigh_chains_id.len()).iter() {
                    // The ids stored in friend_neigh_chains_id may be out of date since we remove chains from self.chains
                    // These id is the correct one at this step of the removals
                    let other_chain_id = other_chain_old_id - nb_removed_chains;

                    // We merge the other chain into the final chain.
                    let other_chain = self.chains[other_chain_id].clone();
                    self.chains[final_chain_id].merge(&other_chain);

                    // We remove the old chain.
                    self.chains.remove(other_chain_id);

                    // We update the ids inside the chains
                    self.update_chains_ids_after_id(other_chain_id);

                    nb_removed_chains += 1;
                }

                self.update_board_ids_after_id(final_chain_id);
            }
        }
    }

    fn update_libs(&mut self, chain_id: uint) {
        let libs = self.chains[chain_id].coords()
                                            .iter()
                                            .fold(Vec::new(), |mut acc, c| {
                                                for &n in c.neighbours(self.size).iter() {
                                                    if n.is_inside(self.size) && self.get_coord(n) == Empty && !acc.contains(&n) {
                                                        acc.push(n);
                                                    }
                                                }
                                                acc
                                            }).len();
        self.chains[chain_id].libs = libs;
    }

    fn update_chains_libs_of(&mut self, color: Color) {
        let adv_chains_ids: HashSet<uint> = self.chains
                  .iter()
                  .filter(|c| c.color == color)
                  .map(|c| c.id)
                  .collect();

        for id in adv_chains_ids.iter() {
            self.update_libs(*id);
        }
    }

    fn update_board_ids_after_id(&mut self, id: uint) {
        for i in range(id, self.chains.len()) {
            for &coord in self.chains[i].coords().iter() {
                self.board[coord.to_index(self.size)] = i;
            }
        }
    }

    fn update_chains_ids_after_id(&mut self, removed_chain_id: uint) {
        for i in range(removed_chain_id, self.chains.len()) {
            self.chains[i].id = i;
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
    fn remove_adv_chains_with_no_libs_close_to(&mut self, m: Move) -> Vec<Coord> {
        let coords_to_remove = m.coords().neighbours(self.size).iter()
                                      .map(|&coord| self.get_chain(coord))
                                      .filter(|chain| chain.libs == 0 && chain.color != m.color())
                                      .fold(Vec::new(), |acc, chain| acc + chain.coords().as_slice());


        let mut chain_to_remove_ids: Vec<uint> = m.coords().neighbours(self.size)
                                                         .iter()
                                                         .map(|&coord| self.get_chain(coord))
                                                         .filter(|chain| chain.libs == 0 && chain.color != m.color())
                                                         .map(|chain| chain.id)
                                                         .collect();

        chain_to_remove_ids.sort();
        chain_to_remove_ids.dedup();
        let mut nb_of_removed_chains = 0;

        for &id in chain_to_remove_ids.iter() {
            self.remove_chain(id-nb_of_removed_chains);
            nb_of_removed_chains += 1;
        }

        coords_to_remove
    }

    fn remove_chain(&mut self, id: uint) {
        let coords_to_remove = self.chains[id].coords().clone();

        for &coord in coords_to_remove.iter() {
            self.remove_stone(coord)
        }

        self.chains.remove(id);
        self.update_chains_ids_after_id(id);
    }

    fn create_new_chain(&mut self, m: Move) {
        let new_chain_id    = self.chains.len();
        let mut new_chain   = Chain::new(new_chain_id, m.color());
        new_chain.add_stone(m.coords());
        self.chains.push(new_chain);
        self.board[m.coords().to_index(self.size)] = new_chain_id;
        self.update_libs(new_chain_id);
    }

    fn add_coord_to_chain(&mut self, coord: Coord, chain_id: uint) {
        self.chains[chain_id].add_stone(coord);
       self.board[coord.to_index(self.size)] = chain_id;
    }

    fn remove_stone(&mut self, c: Coord) {
        self.board[c.to_index(self.size)] = 0;
    }

    pub fn score(&self) -> (uint, uint) {
        self.score_tt()
    }

    fn score_tt(&self) -> (uint, uint) {
        let mut black_score = self.board.iter()
                                        .filter(|&id| self.chains[*id].color == Black)
                                        .count();


        let mut white_score = self.board.iter()
                                        .filter(|&id| self.chains[*id].color == White)
                                        .count();

        let mut empty_intersections = Vec::<Coord>::new();
        for i in range(0, self.board.len()) {
            let id = self.board[i];

            if self.chains[id].color == Empty {
                let c = Coord::from_index(i, self.size);
                empty_intersections.push(c);
            }
        }

        while empty_intersections.len() > 0 {
            let territory = self.build_territory_chain(empty_intersections[0]);

            match territory.color {
                Black => black_score += territory.coords().len(),
                White => white_score += territory.coords().len(),
                Empty => () // This territory is not enclosed by a single color
            }

            empty_intersections = empty_intersections.into_iter().filter(|coord| !territory.coords().contains(coord)).collect();
        }

        (black_score, white_score)
    }

    fn build_territory_chain(&self, first_intersection: Coord) -> Chain {
        let mut territory_chain = Chain::new(0, Empty);
        let mut to_visit = Vec::new();
        let mut neutral  = false;

        to_visit.push(first_intersection);

        while to_visit.len() > 0 {
            let current_coord = to_visit.pop().unwrap();
            if !territory_chain.coords().contains(&current_coord) {territory_chain.add_stone(current_coord);}

            for &coord in current_coord.neighbours(self.size).iter() {
                match self.get_coord(coord) {
                    Empty => if !territory_chain.coords().contains(&coord) {to_visit.push(coord)},
                    col   => if territory_chain.color != Empty && territory_chain.color != col {
                        neutral = true;
                    } else {
                        territory_chain.color = col;
                    }
                }
            }
        }

        if neutral {
            territory_chain.color = Empty;
        }

        territory_chain
    }

    fn compute_hash(&self, m: &Move, adv_stones_removed: &Vec<Coord>, friend_stones_removed: &Vec<Coord>) -> u64 {
        let mut hash = self.zobrist_base_table.add_stone_to_hash(*self.previous_boards_hashes.last().unwrap(), m);

        for &coord in adv_stones_removed.iter() {
            hash = self.zobrist_base_table.remove_stone_from_hash(hash, &Play(m.color().opposite(), coord.col, coord.row));
        }

        for &coord in friend_stones_removed.iter() {
            hash = self.zobrist_base_table.remove_stone_from_hash(hash, &Play(m.color(), coord.col, coord.row));
        }

        hash
    }

    pub fn is_game_over(&self) -> bool {
        self.consecutive_passes == 2
    }

    pub fn ruleset(&self) -> Ruleset {
        self.ruleset
    }

    pub fn size(&self) -> u8 {
        self.size
    }

    pub fn chains<'b>(&'b self) -> &'b Vec<Chain> {
        &self.chains
    }
}
