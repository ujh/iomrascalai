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

use board::{Board, Chain, Move, Play};

use smallvec::SmallVec4;

impl Board {

    ///returns all the possible moves that save the group, 
    ///returns no move if it's not in danger
    pub fn save_group(&self, group: &Chain) -> Vec<Move> {
        match group.liberties().len() {
            1 => self.fix_atari(group),
            2 => {
                let m = self.escape_ladder(group);

                let mut ms = Vec::new();
                if let Some(solution) = m {
                    ms.push(solution); //get one step ahead of the ladder, we could also return the other liberty maybe
                }
                ms
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
            let mut ids = SmallVec4::new();
            let mut one_liberty_enemy_groups = SmallVec4::new();
            for &coord in group.coords().iter() {
                it = self.neighbours(coord).iter()
                        .filter(|c| self.color(c) == enemy)
                        .map(|c| self.chain_id(c));
                for id in it {
                    if !ids.contains(&id) {
                        ids.push(id);
                        one_liberty_enemy_groups.push(coord);
                    }
                }
            }

            for &atari in one_liberty_enemy_groups.iter() {
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
                if self.new_chain_liberties_greater_than_two(m) {
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
        let group_coord = group.coords().iter().next().unwrap();
        for liberty in group.liberties().iter() {
            let m = Play(player, liberty.col, liberty.row);
            if self.is_legal(m).is_ok() {
                if group.liberties().len() == 2 {
                    let mut cloned = self.clone();
                    cloned.play_legal_move(m);

                    let gr = cloned.get_chain(*group_coord).cloned();

                    if let Some(g) = gr {
                        if cloned.fix_atari(&g).len() == 0 {
                            return Some(m); //if atari can't be fixed this group is captured in a ladder
                        }
                    };
                }

                if group.liberties().len() == 1 {
                    return Some(m);
                }
            }
        }
        
        None
    }
    
    //we should probably return a vec because there could be multiple solutions
    pub fn escape_ladder(&self, group: &Chain) -> Option<Move> {
        for liberty in group.liberties().iter() {
            let player = group.color();
            let m = Play(player, liberty.col, liberty.row);
            
            if self.is_legal(m).is_ok() {
                if self.new_chain_liberties_greater_than_one(m) {
                    return Some(m);
                }

                let mut cloned = self.clone();
                cloned.play_legal_move(m);
                
                if self.capture_ladder(group).is_none() {
                    return Some(m);
                }
            }
        }
        
        None
    }

}