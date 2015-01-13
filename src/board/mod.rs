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
pub use board::chain::Chain;
pub use board::coord::Coord;
pub use board::movement::Move;
pub use board::movement::Pass;
pub use board::movement::Play;
pub use self::Color::Black;
pub use self::Color::Empty;
pub use self::Color::White;
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

struct Territory {
    color:  Color,
    coords: Vec<Coord>,
}

impl Territory {

    pub fn new() -> Territory {
        Territory { color: Empty, coords: Vec::new() }
    }
}

#[derive(Show)]
pub struct Board<'a> {
    adv_stones_removed:    Vec<Coord>,
    board:                 Vec<Option<usize>>,
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
            board:                 range(0, size as usize*size as usize).map(|_| None).collect(),
            chains:                Vec::new(),
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
            match self.get_chain(c) {
                Some(chain) => chain.color(),
                None        => Empty,
            }
        } else {
            panic!("You have requested a stone outside of the board");
        }
    }

    pub fn get_chain<'b>(&'b self, c: Coord) -> Option<&'b Chain> {
        if c.is_inside(self.size) {
            let possible_chain_id = self.board[c.to_index(self.size)];
            match possible_chain_id {
                Some(chain_id) => Some(&self.chains[chain_id]),
                None           => None
            }
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
        // Create new chain or merge it with the neighbouring ones. It
        // removes coord from the list of liberties of the
        // neighbouring chains.
        new_board.merge_or_create_chain(&m);
        // Updates the liberties of the opposing neighbouring chains
        new_board.update_libs_of_adjacent_opposing_chains(&m);
        // Removes captured opposing chains
        new_board.adv_stones_removed = new_board.remove_captured_opponent_stones(&m);
        // Adds removed stones as liberties to the neighbouring chains
        new_board.add_removed_adv_stones_as_libs(&m);
        // Checks for suicide play
        if new_board.get_chain(m.coord()).unwrap().is_captured() {
            if new_board.ruleset.suicide_allowed() {
                new_board.friend_stones_removed = new_board.remove_suicide_chain(&m);
                new_board.add_removed_friendly_stones_as_libs(&m);
            } else {
                return Err(IllegalMove::SuicidePlay)
            }
        }
        if new_board.adv_stones_removed.len() == 1 && new_board.friend_stones_removed.len() == 0 {
            let coord = new_board.adv_stones_removed[0];
            new_board.ko = Some(coord);
        } else {
            new_board.ko = None;
        }
        Ok(new_board)
    }

    fn add_removed_adv_stones_as_libs(&mut self, m: &Move) {
        let color = *m.color();
        let mut libs: HashMap<Coord, Vec<usize>> = HashMap::new();
        for &coord in self.adv_stones_removed.iter() {
            let chain_ids = self.neighbours(coord).
                iter()
                .map(|&c| self.get_chain(c))
                .filter(|possible_chain| possible_chain.is_some())
                .map(|possible_chain| possible_chain.unwrap())
                .filter(|chain| chain.color() == color)
                .map(|chain| chain.id())
                .collect();
            libs.insert(coord, chain_ids);
        }
        for (&coord, chain_ids) in libs.iter() {
            for &chain_id in chain_ids.iter() {
                self.chains[chain_id].add_liberty(coord);
            }
        }
    }

    fn add_removed_friendly_stones_as_libs(&mut self, m: &Move) {
        let color = m.color().opposite();
        let mut libs: HashMap<Coord, Vec<usize>> = HashMap::new();
        for &coord in self.adv_stones_removed.iter() {
            let chain_ids = self.neighbours(coord).
                iter()
                .map(|&c| self.get_chain(c))
                .filter(|possible_chain| possible_chain.is_some())
                .map(|possible_chain| possible_chain.unwrap())
                .filter(|chain| chain.color() == color)
                .map(|chain| chain.id())
                .collect();
            libs.insert(coord, chain_ids);
        }
        for (&coord, chain_ids) in libs.iter() {
            for &chain_id in chain_ids.iter() {
                self.chains[chain_id].add_liberty(coord);
            }
        }
    }

    fn update_libs_of_adjacent_opposing_chains(&mut self, m: &Move) {
        let coord = m.coord();
        let color = m.color().opposite();
        let adv_chains_ids: HashSet<usize> = self.neighbours(coord)
            .iter()
            .map(|&c| self.get_chain(c))
            .filter(|possible_chain| possible_chain.is_some())
            .map(|possible_chain| possible_chain.unwrap())
            .filter(|chain| chain.color() == color)
            .map(|chain| chain.id())
            .collect();
        for &id in adv_chains_ids.iter() {
            self.chains[id].remove_liberty(coord);
        }
    }

    fn find_neighbouring_friendly_chains_ids(&self, m: &Move) -> Vec<usize> {
        let mut friend_neigh_chains_id: Vec<usize> = self.neighbours(m.coord())
            .iter()
            .map(|&c| self.get_chain(c))
            .filter(|possible_chain| possible_chain.is_some())
            .map(|possible_chain| possible_chain.unwrap())
            .filter(|chain| chain.color() == *m.color())
            .map(|chain| chain.id())
            .collect();
        // We need to sort the chain by ascending id so that later we
        // know that friend_neigh_chains_id[0] has the lowest id. It
        // also helps with keeping track of the ids of the chain
        // yet-to-merge as their ids will always decrease by nb of
        // chains merged before them.
        friend_neigh_chains_id.sort();
        friend_neigh_chains_id.dedup();
        friend_neigh_chains_id
    }

    fn merge_or_create_chain(&mut self, m: &Move) {
        let mut chain_ids = self.find_neighbouring_friendly_chains_ids(m);
        let new_chain_id = self.create_new_chain(m);
        chain_ids.push(new_chain_id);
        let final_chain_id = chain_ids[0];
        let mut nb_removed_chains = 0;
        for &other_chain_old_id in chain_ids.iter() {
            if other_chain_old_id != final_chain_id {
                let other_chain_id = other_chain_old_id - nb_removed_chains;
                // We merge the other chain into the final chain.
                let other_chain = self.chains[other_chain_id].clone();
                self.chains[final_chain_id].merge(&other_chain);
                // We remove the old chain.
                self.move_stones_to_chain_and_remove(other_chain_id, final_chain_id);
                nb_removed_chains += 1;
            }
        }
        // Removes the played stone from the liberty
        self.chains[final_chain_id].remove_liberty(m.coord());
    }

    fn update_board_ids_after_id(&mut self, id: usize) {
        for i in range(id, self.chains.len()) {
            for &coord in self.chains[i].coords().iter() {
                self.board[coord.to_index(self.size)] = Some(i);
            }
        }
    }

    fn update_chains_ids_after_id(&mut self, removed_chain_id: usize) {
        for i in range(removed_chain_id, self.chains.len()) {
            self.chains[i].set_id(i);
        }
    }

    fn update_all_after_id(&mut self, id: usize) {
        self.update_board_ids_after_id(id);
        self.update_chains_ids_after_id(id);
    }

    fn remove_captured_opponent_stones(&mut self, m: &Move) -> Vec<Coord> {
        let coord = m.coord();
        let color = m.color().opposite();
        let coords_to_remove = self.neighbours(coord)
            .iter()
            .map(|&c| self.get_chain(c))
            .filter(|possible_chain| possible_chain.is_some())
            .map(|possible_chain| possible_chain.unwrap())
            .filter(|chain| chain.is_captured() && chain.color() == color)
            .flat_map(|chain| chain.coords().iter())
            .cloned()
            .collect();
        let mut chains_to_remove: Vec<usize> = self.neighbours(coord)
            .iter()
            .map(|&c| self.get_chain(c))
            .filter(|possible_chain| possible_chain.is_some())
            .map(|possible_chain| possible_chain.unwrap())
            .filter(|chain| chain.is_captured() && chain.color() == color)
            .map(|chain| chain.id())
            .collect();
        chains_to_remove.sort();
        chains_to_remove.dedup();
        let mut nb_of_removed_chains = 0;
        for &id in chains_to_remove.iter() {
            self.remove_chain(id-nb_of_removed_chains);
            nb_of_removed_chains += 1;
        }
        coords_to_remove
    }

    fn remove_suicide_chain(&mut self, m: &Move) -> Vec<Coord> {
        let coords_to_remove = self.get_chain(m.coord()).unwrap().coords().clone();
        let chain_id = self.get_chain(m.coord()).unwrap().id();
        self.remove_chain(chain_id);
        coords_to_remove
    }

    fn remove_chain(&mut self, id: usize) {
        let coords_to_remove = self.chains[id].coords().clone();

        for &coord in coords_to_remove.iter() {
            self.remove_stone(coord)
        }

        self.chains.remove(id);
        self.update_all_after_id(id);
    }

    fn move_stones_to_chain_and_remove(&mut self, old_chain_id: usize, new_chain_id: usize) {
        let coords_to_move = self.chains[old_chain_id].coords().clone();

        for &coord in coords_to_move.iter() {
            self.board[coord.to_index(self.size)] = Some(new_chain_id);
        }

        self.chains.remove(old_chain_id);
        self.update_all_after_id(new_chain_id);

    }

    fn create_new_chain(&mut self, m: &Move) -> usize {
        let new_chain_id    = self.chains.len();
        let mut new_chain   = Chain::new(
            new_chain_id, *m.color(), m.coord(), self.liberties(&m.coord()));
        self.chains.push(new_chain);
        self.board[m.coord().to_index(self.size)] = Some(new_chain_id);
        new_chain_id
    }

    fn liberties(&self, c: &Coord) -> Vec<Coord> {
        self.neighbours(*c).iter().filter(|&c| self.color(*c) == Empty).cloned().collect()
    }

    fn remove_stone(&mut self, c: Coord) {
        self.board[c.to_index(self.size)] = None;
    }

    pub fn score(&self) -> Score {
        Score::new(self.score_tt(), self.komi())
    }

    pub fn winner(&self) -> Color {
        self.score().color()
    }

    fn score_tt(&self) -> (usize, usize) {
        let mut black_score = 0;
        let mut white_score = 0;
        for &possible_id in self.board.iter() {
            match possible_id {
                Some(id) => {
                    if self.chains[id].color() == Black {
                        black_score += 1;
                    } else {
                        white_score += 1;
                    }
                },
                None => {}
            }
        }
        let mut empty_intersections = Vec::<Coord>::new();
        for i in range(0, self.board.len()) {
            let possible_chain_id = self.board[i];
            match possible_chain_id {
                None => {
                    let c = Coord::from_index(i, self.size);
                    empty_intersections.push(c);
                },
                Some(_) => {}
            }
        }
        while empty_intersections.len() > 0 {
            let territory = self.build_territory_chain(empty_intersections[0]);

            match territory.color {
                Black => black_score += territory.coords.len(),
                White => white_score += territory.coords.len(),
                Empty => () // This territory is not enclosed by a single color
            }

            empty_intersections = empty_intersections.into_iter().filter(|coord| !territory.coords.contains(coord)).collect();
        }
        (black_score, white_score)
    }

    fn build_territory_chain(&self, first_intersection: Coord) -> Territory {
        let mut territory_chain = Territory::new();
        let mut to_visit = Vec::new();
        let mut neutral  = false;

        to_visit.push(first_intersection);

        while to_visit.len() > 0 {
            let current_coord = to_visit.pop().unwrap();
            if !territory_chain.coords.contains(&current_coord) {territory_chain.coords.push(current_coord);}

            for &coord in self.neighbours(current_coord).iter() {
                match self.color(coord) {
                    Empty => if !territory_chain.coords.contains(&coord) {to_visit.push(coord)},
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
