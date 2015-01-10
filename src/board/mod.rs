/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
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
pub use board::coord::Coord;
pub use board::movement::Move;
pub use board::movement::Pass;
pub use board::movement::Play;
pub use self::Color::Black;
pub use self::Color::Empty;
pub use self::Color::White;
use board::chain::Chain;
use ruleset::Ruleset;
use score::Score;

use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;
use std::rc::Rc;

mod test;

mod chain;
pub mod coord;
pub mod movement;

#[derive(Show, Eq, PartialEq)]
pub enum IllegalMove {
    GameAlreadyOver,
    IntersectionNotEmpty,
    Ko,
    PlayOutOfBoard,
    SamePlayerPlayedTwice,
    SuicidePlay,
    SuperKo
}

#[derive(Clone, Show, Eq, PartialEq, Hash, Copy)]
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

#[derive(Show)]
pub struct Board<'a> {
    adv_stones_removed:    Vec<Coord>,
    board:                 Vec<usize>,
    chains:                Vec<Chain>,
    consecutive_passes:    u8,
    friend_stones_removed: Vec<Coord>,
    ko:                    Option<Coord>,
    komi:                  f32,
    neighbours:            Rc<HashMap<Coord, Vec<Coord>>>,
    previous_player:       Color,
    ruleset:               Ruleset,
    size:                  u8,
}

impl<'a> Clone for Board<'a> {
    fn clone(&self) -> Board<'a> {
        Board {
            adv_stones_removed:    self.adv_stones_removed.clone(),
            board:                 self.board.clone(),
            chains:                self.chains.clone(),
            consecutive_passes:    self.consecutive_passes,
            friend_stones_removed: self.friend_stones_removed.clone(),
            ko:                    self.ko,
            komi:                  self.komi,
            neighbours:            self.neighbours.clone(),
            previous_player:       self.previous_player,
            ruleset:               self.ruleset.clone(),
            size:                  self.size,
        }
    }
}

impl<'a> Board<'a> {
    pub fn new(size: u8, komi: f32, ruleset: Ruleset) -> Board<'a> {
        Board {
            adv_stones_removed:    Vec::new(),
            board:                 range(0, size as usize*size as usize).map(|_| 0).collect(),
            chains:                vec!(Chain::new(0, Empty)),
            consecutive_passes:    0,
            friend_stones_removed: Vec::new(),
            ko:                    None,
            komi:                  komi,
            neighbours:            Board::setup_neighbours(size),
            previous_player:       White,
            ruleset:               ruleset,
            size:                  size,
        }
    }

    fn setup_neighbours(size: u8) -> Rc<HashMap<Coord, Vec<Coord>>> {
        let mut neighbours = HashMap::new();
        for coord in Coord::for_board_size(size).iter() {
            neighbours.insert(*coord, coord.neighbours(size));
        }
        Rc::new(neighbours)
    }

    fn neighbours(&self, c: Coord) -> &Vec<Coord> {
        &self.neighbours[c]
    }

    pub fn color(&self, c: Coord) -> Color {
        if c.is_inside(self.size) {
            self.get_chain(c).color
        } else {
            panic!("You have requested a stone outside of the board");
        }
    }

    pub fn get_chain<'b>(&'b self, c: Coord) -> &'b Chain {
        if c.is_inside(self.size) {
            let chain_id = self.board[c.to_index(self.size)];
            &self.chains[chain_id]
        } else {
            panic!("You have requested a chain outside of the board");
        }
    }

    pub fn komi(&self) -> f32 {
        self.komi
    }

    pub fn set_komi(&mut self, komi: f32) {
        self.komi = komi;
    }

    pub fn next_player(&self) -> Color {
        self.previous_player.opposite()
    }

    fn is_same_player(&self, m: &Move) -> bool {
        self.previous_player == *m.color()
    }

    pub fn adv_stones_removed(&self) -> &Vec<Coord> {
        &self.adv_stones_removed
    }

    pub fn friend_stones_removed(&self) -> &Vec<Coord> {
        &self.friend_stones_removed
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let color = self.next_player();
        let mut moves : Vec<Move> = Coord::for_board_size(self.size).iter().map(
            |coord| Play(color.clone(), coord.col, coord.row)).filter(
            |m| self.play(*m).is_ok()).collect();
        moves.push(Pass(color.clone()));
        moves
    }

    // Note: Same as get(), the board is indexed starting at 1-1
    pub fn play(&self, m: Move) -> Result<Board, IllegalMove> {
        // We check is the player is trying to play on a finished game (which is illegal in TT rules)
        if self.is_game_over() && !self.ruleset.game_over_play() {
            return Err(IllegalMove::GameAlreadyOver);
        }

        // We check that the same player didn't play twice (except in the minimal ruleset, which is useful for tests)
        if self.is_same_player(&m) && !self.ruleset.same_player() {
            return Err(IllegalMove::SamePlayerPlayedTwice);
        }

        // Then we check if the player passed
        if m.is_pass() {
            let mut new_board = self.clone();
            new_board.consecutive_passes += 1;
            new_board.previous_player    = *m.color();
            return Ok(new_board);
        }

        // We check if the new move is inside the board (and if it is, if there is no stone there)
        if m.coord().is_inside(self.size) {
            if self.color(m.coord()) != Empty {
                return Err(IllegalMove::IntersectionNotEmpty);
            }
        } else {
            return Err(IllegalMove::PlayOutOfBoard);
        }

        if self.ko.is_some() && m.coord() == self.ko.unwrap() {
            return Err(IllegalMove::Ko);
        }

        let mut new_board = self.clone();

        new_board.previous_player    = *m.color();

        new_board.consecutive_passes = 0;

        new_board.merge_or_create_chain(&m);
        // We then update the libs of all chains of the opposite color
        new_board.update_chains_libs_of(m.color().opposite());

        let adv_stones_removed = new_board.remove_adv_chains_with_no_libs_close_to(&m);
        new_board.update_all();

        let final_chain_id = new_board.get_chain(m.coord()).id;
        new_board.update_libs(final_chain_id);

        let mut friend_stones_removed = Vec::new();

        if adv_stones_removed.len() > 0 {
            for i in range(1, new_board.chains.len()) {
                new_board.update_libs(i);
            }
        } else if new_board.get_chain(m.coord()).is_captured() {
            if self.ruleset.suicide_allowed() {
                friend_stones_removed.push_all(new_board.get_chain(m.coord()).coords().as_slice());
                let to_remove_id = new_board.get_chain(m.coord()).id;
                new_board.remove_chain(to_remove_id);
                new_board.update_all_after_id(to_remove_id);
            } else {
                return Err(IllegalMove::SuicidePlay)
            }
        }
        if adv_stones_removed.len() == 1 && friend_stones_removed.len() == 0 {
            let coord = adv_stones_removed[0];
            new_board.ko = Some(coord);
        } else {
            new_board.ko = None;
        }

        new_board.friend_stones_removed = friend_stones_removed;
        new_board.adv_stones_removed = adv_stones_removed;
        Ok(new_board)
    }

    fn find_neighbouring_friendly_chains_ids(&self, m: &Move) -> Vec<usize> {
        let mut friend_neigh_chains_id: Vec<usize> = self.neighbours(m.coord())
                  .iter()
                  .filter(|&c| c.is_inside(self.size) && self.color(*c) == *m.color())
                  .map(|&c| self.get_chain(c).id)
                  .collect();

        // We need to sort the chain by ascending id so that later we know that friend_neigh_chains_id[0] has the lowest id.
        // It also helps with keeping track of the ids of the chain yet-to-merge as their ids will always decrease by nb of chains
        // merged before them.
        friend_neigh_chains_id.sort();
        friend_neigh_chains_id.dedup();
        friend_neigh_chains_id
    }

    fn merge_or_create_chain(&mut self, m: &Move) {
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
                self.add_coord_to_chain(m.coord(), final_chain_id);
            },
            _ => {
                // Note: We know that friend_neigh_chains_id is sorted, so whatever chains we remove,
                // we know that the id of the final_chain is still valid.
                let final_chain_id        = friend_neigh_chains_id[0];
                let mut nb_removed_chains = 0;

                // We assign the stone to the final chain
                self.add_coord_to_chain(m.coord(), final_chain_id);

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

    fn update_libs(&mut self, chain_id: usize) {
        for &coord in self.chains[chain_id].coords().clone().iter() {
            for &n in self.neighbours(coord).clone().iter() {
                if self.color(n) == Empty {
                    self.chains[chain_id].add_liberty(n);
                } else {
                    self.chains[chain_id].remove_liberty(n);
                }
            }
        }
    }

    fn update_chains_libs_of(&mut self, color: Color) {
        let adv_chains_ids: HashSet<usize> = self.chains
                  .iter()
                  .filter(|c| c.color == color)
                  .map(|c| c.id)
                  .collect();

        for id in adv_chains_ids.iter() {
            self.update_libs(*id);
        }
    }

    fn update_board_ids_after_id(&mut self, id: usize) {
        for i in range(id, self.chains.len()) {
            for &coord in self.chains[i].coords().iter() {
                self.board[coord.to_index(self.size)] = i;
            }
        }
    }

    fn update_chains_ids_after_id(&mut self, removed_chain_id: usize) {
        for i in range(removed_chain_id, self.chains.len()) {
            self.chains[i].id = i;
        }
    }

    fn update_all_after_id(&mut self, id: usize) {
        self.update_board_ids_after_id(id);
        self.update_chains_ids_after_id(id);
    }

    fn update_all(&mut self) {
        self.update_all_after_id(0);
    }

    // Returns a vector of the coords where stones where removed.
    fn remove_adv_chains_with_no_libs_close_to(&mut self, m: &Move) -> Vec<Coord> {
        let color = m.color().opposite();
        let coords_to_remove = self.neighbours(m.coord()).iter()
            .map(|&coord| self.get_chain(coord))
            .filter(|chain| chain.is_captured() && chain.color == color)
            .fold(Vec::new(), |acc, chain| acc + chain.coords().as_slice());


        let mut chain_to_remove_ids: Vec<usize> = self.neighbours(m.coord())
            .iter()
            .map(|&coord| self.get_chain(coord))
            .filter(|chain| chain.is_captured() && chain.color == color)
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

    fn remove_chain(&mut self, id: usize) {
        let coords_to_remove = self.chains[id].coords().clone();

        for &coord in coords_to_remove.iter() {
            self.remove_stone(coord)
        }

        self.chains.remove(id);
        self.update_chains_ids_after_id(id);
    }

    fn create_new_chain(&mut self, m: &Move) {
        let new_chain_id    = self.chains.len();
        let mut new_chain   = Chain::new(new_chain_id, *m.color());
        new_chain.add_stone(m.coord());
        self.chains.push(new_chain);
        self.board[m.coord().to_index(self.size)] = new_chain_id;
        self.update_libs(new_chain_id);
    }

    fn add_coord_to_chain(&mut self, coord: Coord, chain_id: usize) {
        self.chains[chain_id].add_stone(coord);
       self.board[coord.to_index(self.size)] = chain_id;
    }

    fn remove_stone(&mut self, c: Coord) {
        self.board[c.to_index(self.size)] = 0;
    }

    pub fn score(&self) -> Score {
        Score::new(self.score_tt(), self.komi())
    }

    pub fn winner(&self) -> Color {
        self.score().color()
    }

    fn score_tt(&self) -> (usize, usize) {
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

            for &coord in self.neighbours(current_coord).iter() {
                match self.color(coord) {
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

    pub fn as_string(&self) -> String {
        let mut s = String::new();
                // First we print the board
        for row in range(1u8, self.size()+1).rev() {

            // Prints the row number
            s.push_str(format!("{:2} ", row).as_slice());

            // Prints the actual row
            for col in range(1u8, self.size()+1) {
                let current_coords = Coord::new(col, row);

                match self.color(current_coords) {
                    Empty => {
                        let hoshis = &[4u8,10,16];
                        if  hoshis.contains(&row) && hoshis.contains(&col) {
                            s.push_str("+ ");
                        } else {
                            s.push_str(". ");
                        }
                    },
                    White => { s.push_str("O "); },
                    Black => { s.push_str("X "); }
                }
            }
            s.push_str("\n");
        }
        s
    }
}
