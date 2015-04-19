/************************************************************************
 *                                                                      *
 * Copyright 2015 Igor Polyakov                                         *
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

use board::{Board, Chain, Coord, Move, Pass, Play};

use smallvec::SmallVec4;

impl Board {

    ///returns all the possible moves that save the group, 
    ///returns no move if it's not in danger
    pub fn save_group(&self, group: &Chain) -> Vec<Move> {
        match group.liberties().len() {
            1 => self.fix_atari(group),
            2 => if let Some(m) = self.escape_ladder(group) {
                m
            } else {
                vec![]
            },
            _ => vec![] //return just the forced moves when we have two liberties
        }
    }

    //if one liberty
    //returns no moves if can't fix
    pub fn fix_atari(&self, group: &Chain) -> Vec<Move> {
        //try capturing any neighbouring groups
        let mut solutions = vec![];
        let player = group.color();
        {
            let enemy = group.color().opposite();
            
            let mut it;
            let mut one_liberty_enemy_groups: SmallVec4<Coord> = SmallVec4::new();
            for &coord in group.coords().iter() {
                it = self.neighbours(coord).iter()
                    .filter(|c| self.color(c) == enemy)
                    .map(|&c| self.get_chain(c).unwrap())
                    .filter(|chain| chain.liberties().len() == 1)
                    .flat_map(|chain| chain.liberties());
                for liberty in it {
                    if !one_liberty_enemy_groups.contains(&liberty) {
                        one_liberty_enemy_groups.push(*liberty);
                    }
                }
            }

            for atari in one_liberty_enemy_groups.iter() {
                let m = Play(player, atari.col, atari.row);
                if self.is_legal(m).is_ok() {
                    solutions.push(m);
                }
            }
        }

        //escaping
        if let Some(liberty) = group.liberties().iter().next() {
            let m = Play(player, liberty.col, liberty.row);
            if self.is_legal(m).is_ok() {
                if self.new_chain_liberties_greater_than(m, 2) {
                    solutions.push(m);
                } else if self.new_chain_liberties_greater_than_one(m) {
                    let mut cloned = self.clone();
                    cloned.play_legal_move(m);
                    let gr = cloned.get_chain(*liberty).cloned();
                    
                    if let Some(g) = gr {
                        if cloned.capture_ladder(&g).is_none() {
                            solutions.push(m)
                        }
                    }
                }
            }
        }

        solutions
    }

    //if two liberties read ladder
    //returns None if can't capture
    pub fn capture_ladder(&self, group: &Chain) -> Option<Move> {
        let player = group.color().opposite();
        
        if group.liberties().len() > 2 {
            return None;
        }
        
        if group.liberties().len() == 1 {
            let liberty = group.liberties().iter().next().unwrap();
            let m = Play(player, liberty.col, liberty.row);
            return Some(m);
        }
        
        let mut liberties = group.liberties().iter();

        let liberty1 = liberties.next().unwrap();
        let liberty2 = liberties.next().unwrap();
        
        let lib2_move = Play(group.color(), liberty2.col, liberty2.row);
        
        //if lib2 move gives more than 3 liberties, forget about reading out lib1
        if !self.new_chain_liberties_greater_than(lib2_move, 3) { 
            let m = Play(player, liberty1.col, liberty1.row);
            let mut cloned = self.clone();

            if self.next_player() != player {
                cloned.play_legal_move(Pass(self.next_player()));
            }
            
            let cap = cloned.try_capture(group, m);
            if cap.is_some() {
                return cap;
            }
        }
        
        let lib1_move = Play(group.color(), liberty1.col, liberty1.row);
        
        //same as above, but reversed
        if !self.new_chain_liberties_greater_than(lib1_move, 3) {
        
            let m = Play(player, liberty2.col, liberty2.row);
            let mut cloned = self.clone();

            if self.next_player() != player {
                cloned.play_legal_move(Pass(self.next_player()));
            }
            
            let cap = cloned.try_capture(group, m);
            if cap.is_some() {
                return cap;
            }
        }

        None
    }
    
    fn try_capture(&mut self, group: &Chain, m: Move) -> Option<Move> {
        let group_coord = group.coords().iter().next().unwrap();

        if self.is_legal(m).is_ok() {
            if group.liberties().len() == 2 {
                self.play_legal_move(m);

                let gr = self.get_chain(*group_coord).cloned();

                if let Some(g) = gr {
                    if self.fix_atari(&g).len() == 0 {
                        return Some(m); //if atari can't be fixed this group is captured in a ladder
                    }
                }
            }
        }
        
        None
    }
    
    //we should probably return a vec because there could be multiple solutions
    //return None if can't capture
    //should probably just move Option the logic into the save_group function
    pub fn escape_ladder(&self, group: &Chain) -> Option<Vec<Move>> {
        if self.capture_ladder(group).is_some() {
            
            let mut solutions = vec![];
            
            for liberty in group.liberties().iter() {
                let player = group.color();
                let m = Play(player, liberty.col, liberty.row);
                    
                if self.is_legal(m).is_ok() {
                    if self.new_chain_liberties_greater_than_one(m) {
                        solutions.push(m);
                    }
                }
            }
            
            Some(solutions)
        } else {
            None
        }
    }

}