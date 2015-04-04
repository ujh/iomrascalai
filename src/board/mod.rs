/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
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

pub use self::Color::Black;
pub use self::Color::Empty;
pub use self::Color::White;
pub use self::chain::Chain;
pub use self::coord::Coord;
pub use self::movement::Move;
pub use self::movement::Pass;
pub use self::movement::Play;
pub use self::movement::Resign;
use ruleset::Ruleset;
use score::Score;
use self::point::Point;

use rand::Rng;
use rand::XorShiftRng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::sync::Arc;

mod chain;
mod coord;
mod movement;
mod point;
mod test;

#[derive(Debug, Eq, PartialEq)]
pub enum IllegalMove {
    GameAlreadyOver,
    IntersectionNotEmpty,
    Ko,
    PlayOutOfBoard,
    SamePlayerPlayedTwice,
    SuicidePlay,
    SuperKo
}

impl fmt::Display for IllegalMove {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format!("{:?}", self).fmt(f)
    }
}


#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum Color {
    White,
    Black,
    Empty
}

impl Color {
	#[inline(always)]
    pub fn opposite(&self) -> Color {
        match *self {
            White => Black,
            Black => White,
            Empty => Empty
        }
    }

    pub fn from_gtp(gtp_color: &str) -> Color {
        let lower_gtp_color: String = gtp_color.chars().map(|c| c.to_lowercase().next().unwrap()).collect();
        match lower_gtp_color.as_ref() {
            "w" | "white" => White,
            "b" | "black" => Black,
            err           => panic!("Can't read the GTP color: {}", err)
        }
    }
}

#[derive(Debug)]
struct Cache {
	diagonals: Vec<Vec<Coord>>,
	neighbours: Vec<Vec<Coord>>,
}

impl Cache {
    pub fn new(size: u8) -> Cache {
        Cache {
            diagonals:             Cache::setup_diagonals(size),
            neighbours:            Cache::setup_neighbours(size),
        }
    }

    fn setup_neighbours(size: u8) -> Vec<Vec<Coord>> {
        let mut neighbours = Vec::new();
        for coord in Coord::for_board_size(size).iter() {
            neighbours.push(coord.neighbours(size));
        }
        neighbours
    }

    fn setup_diagonals(size: u8) -> Vec<Vec<Coord>> {
        let mut diagonals = Vec::new();
        for coord in Coord::for_board_size(size).iter() {
            diagonals.push(coord.diagonals(size));
        }
        diagonals
    }
}

#[derive(Debug)]
pub struct Board {
    adv_stones_removed:    Vec<Coord>,
    board:                 Vec<Point>,
    chains:                Vec<Chain>,
    consecutive_passes:    u8,
    cache:                 Arc<Cache>,
    friend_stones_removed: Vec<Coord>,
    ko:                    Option<Coord>,
    komi:                  f32,
    previous_player:       Color,
    resigned_by:           Color,
    ruleset:               Ruleset,
    size:                  u8,
    vacant:                Vec<Coord>,
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board {
            adv_stones_removed:    self.adv_stones_removed.clone(),
            board:                 self.board.clone(),
            chains:                self.chains.clone(),
            cache:                 self.cache.clone(),
            consecutive_passes:    self.consecutive_passes,
            friend_stones_removed: self.friend_stones_removed.clone(),
            ko:                    self.ko,
            komi:                  self.komi,
            previous_player:       self.previous_player,
            resigned_by:           self.resigned_by,
            ruleset:               self.ruleset,
            size:                  self.size,
            vacant:                self.vacant.clone(),
        }
    }
}

impl Board {
    pub fn new(size: u8, komi: f32, ruleset: Ruleset) -> Board {
        Board {
            adv_stones_removed:    Vec::new(),
            board:                 (0..size as usize*size as usize).map(|_| Point::new()).collect(),
            chains:                Vec::new(),
            consecutive_passes:    0,
            cache:                 Arc::new(Cache::new(size)),
            friend_stones_removed: Vec::new(),
            ko:                    None,
            komi:                  komi,
            previous_player:       White,
            resigned_by:           Empty,
            ruleset:               ruleset,
            size:                  size,
            vacant:                Coord::for_board_size(size),
        }
    }

    #[inline(always)]
    pub fn neighbours(&self, c: Coord) -> &Vec<Coord> {
        &self.cache.neighbours[c.to_index(self.size)]
    }
    #[inline(always)]
    pub fn diagonals(&self, c: Coord) -> &Vec<Coord> {
        &self.cache.diagonals[c.to_index(self.size)]
    }

    pub fn points(&self) -> &Vec<Point> {
        &self.board
    }

    pub fn vacant(&self) -> &Vec<Coord> {
        &self.vacant
    }
    #[inline(always)]
    pub fn color(&self, c: &Coord) -> Color {
        self.board[c.to_index(self.size)].color
    }
    #[inline(always)]
    pub fn chain_id(&self, c: &Coord) -> usize {
        self.board[c.to_index(self.size)].chain_id
    }
    #[inline(always)]
    pub fn get_chain<'b>(&'b self, c: Coord) -> Option<&'b Chain> {
        let ref point = self.board[c.to_index(self.size)];
        if point.color != Empty {
            Some(&self.chains[point.chain_id])
        } else {
            None
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
    //#[inline(never)] //turn off for profiling
    pub fn is_eye(&self, coord: &Coord, color: Color) -> bool {
        let neighbours = self.neighbours(*coord);
        if neighbours.iter().all(|c| self.color(c) == color) {
            let diagonals = self.diagonals(*coord);
            if diagonals.len() < 4 {
                diagonals.iter().all(|c| self.color(c) != color.opposite())
            } else {
                diagonals.iter().filter(|c| self.color(c) == color.opposite()).count() <= 1
            }
        } else {
            false
        }
    }
    #[inline(always)]
    fn is_same_player(&self, m: &Move) -> bool {
        self.previous_player == *m.color()
    }

    pub fn adv_stones_removed(&self) -> &Vec<Coord> {
        &self.adv_stones_removed
    }

    pub fn friend_stones_removed(&self) -> &Vec<Coord> {
        &self.friend_stones_removed
    }
    //#[inline(never)] //turn off for profiling
    pub fn legal_moves_without_superko_check(&self) -> Vec<Move> {
        if self.is_game_over() {
            vec!()
        } else {
            let color = self.next_player();
            self.vacant
                .iter()
                .map(|coord| Play(color, coord.col, coord.row))
                .filter(|m| self.is_legal(*m).is_ok())
                .collect()
        }
    }

    //#[inline(never)] //turn off for profiling
    pub fn legal_moves_without_eyes(&self) -> Vec<Move> {
        self.legal_moves_without_superko_check()
            .into_iter()
            .filter(|m| m.is_pass() || !self.is_eye(&m.coord(), *m.color()))
            .collect()
    }
    //#[inline(never)] //turn off for profiling
    pub fn is_legal(&self, m: Move) -> Result<(), IllegalMove> {
        // Can't play if the game is already over
        if self.is_game_over() && !self.ruleset.game_over_play() {
            return Err(IllegalMove::GameAlreadyOver);
        }
        // Player can't play twice
        if self.is_same_player(&m) && !self.ruleset.same_player() {
            return Err(IllegalMove::SamePlayerPlayedTwice);
        }
        // Pass is always allowed
        if m.is_pass() {
            return Ok(());
        }
        // Resigning is allowed here, as the game over check has
        // already been passed successfully
        if m.is_resign() {
            return Ok(());
        }
        // Can't play outside of the board or on an occupied coord
        if m.coord().is_inside(self.size) {
            if self.color(&m.coord()) != Empty {
                return Err(IllegalMove::IntersectionNotEmpty);
            }
        } else {
            return Err(IllegalMove::PlayOutOfBoard);
        }
        // Can't play on a Ko point
        if self.ko.is_some() && m.coord() == self.ko.unwrap() {
            if self.neighbours(m.coord()) //neighbours of the coordinate of the ko point
                .iter()
                .filter(|&c| self.color(c) == m.color().opposite()) //accept coordinates of opposite stones
                .map(|&c| self.get_chain(c).unwrap()) //get the chain of those opposite stones
                .any(|chain| chain.liberties().len() == 1 && chain.coords().len() == 1) { //if any of them has one liberty and one stone
                    return Err(IllegalMove::Ko);
                }
        }
        // Can't play suicide move
        if !self.ruleset.suicide_allowed() {
            // All neighbours must be occupied
            if self.neighbours(m.coord()).iter().all(|c| self.color(c) != Empty) {
                // A move is a suicide move if all of the opposing,
                // neighbouring chain has more than one liberty and all of
                // our own chains have only one liberty.
                let enemy_chains_with_other_libs = self.neighbours(m.coord())
                    .iter()
                    .filter(|&c| self.color(c) == m.color().opposite())
                    .all(|&c| self.get_chain(c).unwrap().liberties().len() > 1);
                let own_chains_without_other_libs = self.neighbours(m.coord())
                    .iter()
                    .filter(|&c| self.color(c) == *m.color())
                    .all(|&c| self.get_chain(c).unwrap().liberties().len() <= 1);
                if enemy_chains_with_other_libs && own_chains_without_other_libs {
                    return Err(IllegalMove::SuicidePlay);
                }
            }
        }
        Ok(())
    }
    //#[inline(never)] //turn off for profiling
    // Note: Same as get(), the board is indexed starting at 1-1
    pub fn play(&mut self, m: Move) -> Result<(), IllegalMove> {
        try!(self.is_legal(m));
        self.play_legal_move(m);
        Ok(())
    }
    
    //always called on moves that are already known to be legal
    pub fn play_legal_move(&mut self, m: Move) {
        self.previous_player = *m.color();
        
        if m.is_pass() {
            self.consecutive_passes += 1;
            return;
        } else {
            self.consecutive_passes = 0;
        }
        
        if m.is_resign() {
            self.resigned_by = *m.color();
            return;
        }
        
        // Create new chain or merge it with the neighbouring ones. It
        // removes coord from the list of liberties of the
        // neighbouring chains.
        self.merge_or_create_chain(&m);
        // Updates the liberties of the opposing neighbouring chains
        self.update_libs_of_adjacent_opposing_chains(&m);
        // Removes captured opposing chains
        self.adv_stones_removed = self.remove_captured_opponent_stones(&m);
        // Adds removed stones as liberties to the neighbouring chains
        self.add_removed_adv_stones_as_libs(&m);
        // Checks for suicide play
        if self.get_chain(m.coord()).unwrap().is_captured() {
            self.friend_stones_removed = self.remove_suicide_chain(&m);
            self.add_removed_friendly_stones_as_libs(&m);
        }
        if self.adv_stones_removed.len() == 1 && self.friend_stones_removed.len() == 0 {
            let coord = self.adv_stones_removed[0];
            self.ko = Some(coord);
        } else {
            self.ko = None;
        }
        self.update_vacant(&m);
    }
    
    
    //#[inline(never)] //turn off for profiling
    fn update_vacant(&mut self, m: &Move) {
        let pos = self.vacant.iter().position(|&c| c == m.coord()).unwrap();
        self.vacant.swap_remove(pos);
        self.vacant.push_all(self.adv_stones_removed.as_ref());
        self.vacant.push_all(self.friend_stones_removed.as_ref());
    }
    //#[inline(never)] //turn off for profiling
    fn add_removed_adv_stones_as_libs(&mut self, m: &Move) {
        let color = *m.color();
        let mut libs: HashMap<Coord, Vec<usize>> = HashMap::new();
        for &coord in self.adv_stones_removed.iter() {
            let chain_ids = self.neighbours(coord)
                .iter()
                .filter(|&c| self.color(c) == color)
                .map(|c| self.chain_id(c))
                .collect();
            libs.insert(coord, chain_ids);
        }
        for (&coord, chain_ids) in libs.iter() {
            for &chain_id in chain_ids.iter() {
                self.chains[chain_id].add_liberty(coord);
            }
        }
    }
    //#[inline(never)] //turn off for profiling
    fn add_removed_friendly_stones_as_libs(&mut self, m: &Move) {
        let color = m.color().opposite();
        let mut libs: HashMap<Coord, Vec<usize>> = HashMap::new();
        for &coord in self.adv_stones_removed.iter() {
            let chain_ids = self.neighbours(coord)
                .iter()
                .filter(|&c| self.color(c) == color)
                .map(|c| self.chain_id(c))
                .collect();
            libs.insert(coord, chain_ids);
        }
        for (&coord, chain_ids) in libs.iter() {
            for &chain_id in chain_ids.iter() {
                self.chains[chain_id].add_liberty(coord);
            }
        }
    }
    //#[inline(never)] //turn off for profiling
    fn update_libs_of_adjacent_opposing_chains(&mut self, m: &Move) {
        let coord = m.coord();
        let color = m.color().opposite();
        let adv_chains_ids: HashSet<usize> = self.neighbours(coord)
            .iter()
            .filter(|&c| self.color(c) == color)
            .map(|c| self.chain_id(c))
            .collect();
        for &id in adv_chains_ids.iter() {
            self.chains[id].remove_liberty(coord);
        }
    }
    //#[inline(never)] //turn off for profiling
    fn find_neighbouring_friendly_chains_ids(&self, m: &Move) -> Vec<usize> {
        let mut friend_neigh_chains_id: Vec<usize> = self.neighbours(m.coord())
            .iter()
            .filter(|&c| self.color(c) == *m.color())
            .map(|c| self.chain_id(c))
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
    //#[inline(never)] //turn off for profiling
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
                let other_chain = self.chains.remove(other_chain_id);
                for &coord in other_chain.coords().iter() {
                    self.board[coord.to_index(self.size)].chain_id = final_chain_id;
                    self.chains[final_chain_id].add_coord(coord);
                }
                for &lib in other_chain.liberties().iter() {
                    self.chains[final_chain_id].add_liberty(lib);
                }
                self.update_all_after_id(other_chain_id);
                nb_removed_chains += 1;
            }
        }
        // Removes the played stone from the liberty
        self.chains[final_chain_id].remove_liberty(m.coord());
    }
    //#[inline(never)] //turn off for profiling
    fn update_board_ids_after_id(&mut self, id: usize) {
        for i in id..self.chains.len() {
            for &coord in self.chains[i].coords().iter() {
                self.board[coord.to_index(self.size)].chain_id = i;
            }
        }
    }
    //#[inline(never)] //turn off for profiling
    fn update_chains_ids_after_id(&mut self, removed_chain_id: usize) {
        for i in removed_chain_id..self.chains.len() {
            self.chains[i].set_id(i);
        }
    }
    //#[inline(never)] //turn off for profiling
    fn update_all_after_id(&mut self, id: usize) {
        self.update_board_ids_after_id(id);
        self.update_chains_ids_after_id(id);
    }
    //#[inline(never)] //turn off for profiling
    fn remove_captured_opponent_stones(&mut self, m: &Move) -> Vec<Coord> {
        let coord = m.coord();
        let color = m.color().opposite();
        let mut coords_to_remove: Vec<Coord> = self.neighbours(coord)
            .iter()
            .filter(|&c| self.color(c) == color)
            .filter(|&c| self.get_chain(*c).unwrap().is_captured())
            .flat_map(|&c| self.get_chain(c).unwrap().coords().iter())
            .cloned()
            .collect();
        coords_to_remove.sort();
        coords_to_remove.dedup();
        let mut chains_to_remove: Vec<usize> = self.neighbours(coord)
            .iter()
            .filter(|&c| self.color(c) == color)
            .filter(|&c| self.get_chain(*c).unwrap().is_captured())
            .map(|c| self.chain_id(c))
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
    //#[inline(never)] //turn off for profiling
    fn remove_suicide_chain(&mut self, m: &Move) -> Vec<Coord> {
        let coords_to_remove = self.get_chain(m.coord()).unwrap().coords().clone();
        let chain_id = self.chain_id(&m.coord());
        self.remove_chain(chain_id);
        coords_to_remove
    }
    //#[inline(never)] //turn off for profiling
    fn remove_chain(&mut self, id: usize) {
        let coords_to_remove = self.chains[id].coords().clone();

        for &coord in coords_to_remove.iter() {
            self.remove_stone(coord)
        }

        self.chains.remove(id);
        self.update_all_after_id(id);
    }
    //#[inline(never)] //turn off for profiling
    fn create_new_chain(&mut self, m: &Move) -> usize {
        let new_chain_id = self.chains.len();
        let new_chain    = Chain::new(
            new_chain_id, *m.color(), m.coord(), self.liberties(&m.coord()));
        self.chains.push(new_chain);
        self.board[m.coord().to_index(self.size)].chain_id = new_chain_id;
        self.board[m.coord().to_index(self.size)].color = *m.color();
        new_chain_id
    }
    //#[inline(never)] //turn off for profiling
    fn liberties(&self, c: &Coord) -> HashSet<Coord> {
        self.neighbours(*c).iter().filter(|&c| self.color(c) == Empty).cloned().collect()
    }
    #[inline(always)]
    fn remove_stone(&mut self, c: Coord) {
        // Resetting the chain_id is not strictly necessary, but will
        // make debugging easier.
        self.board[c.to_index(self.size)].chain_id = -1;
        self.board[c.to_index(self.size)].color = Empty;
    }
    
    pub fn liberty_count(&self, c: Coord) -> usize {
        self.neighbours(c).iter().filter(|c| self.color(c) == Empty).count()
    }
    
    pub fn removes_enemy_neighbouring_stones(&self, m: Move) -> usize {
        let enemy = m.color().opposite();
        self.neighbours(m.coord()).iter()                           //for all neighbours
            .filter(|c| {                                           //take only
                self.color(c) == enemy &&                           //the enemy stones
                self.get_chain(**c).unwrap().liberties().len() == 1 //that have 1 liberty
            })
            .count()
    }
    
    pub fn new_chain_liberties(&self, m: Move) -> usize {
        let mut set: HashSet<Coord> = HashSet::new();
        
        for &c in self.neighbours(m.coord()).iter() {
            if(self.color(&c) == *m.color()) {
                //add the liberties the chain
                for &liberty in self.get_chain(c).unwrap().liberties() {
                    set.insert(liberty);
                }
            } else if(self.color(&c) == Empty)  {
                set.insert(c);
            }
        };
        set.len() - 1 //minus the stone we're about to play
    }
    
    //the length of all merged chains after the current move
    pub fn new_chain_length(&self, m: Move) -> usize {
        let set: HashSet<&Coord> = self.neighbours(m.coord()).iter()
            .filter(|c| self.color(c) == *m.color())
            .flat_map(|c| self.get_chain(*c).unwrap().coords().iter())
            .collect();
        set.len() + 1 //plus the stone we're about to play
    }

    pub fn score(&self) -> Score {
        Score::new(self)
    }

    pub fn winner(&self) -> Color {
        match self.resigned_by {
            Empty => self.score().color(),
            color => color.opposite(),
        }
    }

    #[inline(always)]
    pub fn is_game_over(&self) -> bool {
        self.consecutive_passes == 2 || self.resigned_by != Empty
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

    pub fn vacant_point_count(&self) -> u16 {
        self.vacant.len() as u16
    }

    pub fn as_string(&self) -> String {
        let mut s = String::new();
                // First we print the board
        for row in (1u8..self.size()+1).rev() {

            // Prints the row number
            s.push_str(format!("{:2} ", row).as_ref());

            // Prints the actual row
            for col in (1u8..self.size()+1) {
                let current_coords = Coord::new(col, row);

                match self.color(&current_coords) {
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
