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

use board::{Board, Chain, Coord, Move, Play};

use smallvec::SmallVec4;
use std::collections::HashMap;

impl Board {

    ///returns all the possible moves that save the group, 
    ///returns no move if it's not in danger
    pub fn save_group(&self, group: &Chain) -> Vec<Move> {
        match group.coords().len() {
            1 => self.clone().fix_atari(group),
            2 => {
                let m = self.clone().read_ladder(group);
                let mut ms = Vec::new();
                if let Some(solution) = m {
                    ms.push(solution); //get one step ahead of the ladder, we could also return the other liberty maybe
                }
                ms
            },
            _ => vec![] //return just the forced moves
        }
    }
    
    
    //if one liberty
    //returns None if can't fix
    fn fix_atari(&mut self, group: &Chain) -> Vec<Move> {
        //try capturing any neighbouring groups
        let mut solutions = vec![];
        let player = group.color();
        {
            let enemy = group.color().opposite();
            
            let mut it;
            let mut one_liberty_enemy_groups: HashMap<usize, Coord> = HashMap::new();
            for &coord in group.coords().iter() {
                it = self.neighbours(coord).iter()
                        .filter(|c| self.color(c) == enemy)
                        .map(|c| self.chain_id(c));
                for id in it {
                    one_liberty_enemy_groups.insert(id, coord);
                }
            }
            
            for (_, &atari) in one_liberty_enemy_groups.iter() {
                solutions.push(Play(player, atari.col, atari.row));
            }
        }

        
        //escaping
        let liberty = group.liberties().iter().next().unwrap();

        let m = Play(player, liberty.col, liberty.row);
        if self.play(m).is_ok() {
            let liberties = group.liberties().len();
            match liberties {
                2 => if self.read_ladder(group).is_none() {
                    solutions.push(m)
                },
                1 => {},
                _ => solutions.push(m),
            };
        }
        
        solutions
    }
    
    //if two liberties read ladder
    //returns None if can't capture
    fn read_ladder(&mut self, group: &Chain) -> Option<Move> {
        for liberty in group.liberties().iter() {
            let player = group.color().opposite();
            let m = Play(player, liberty.col, liberty.row);
            if self.play(m).is_ok() {
                if group.liberties().len() == 1 && self.fix_atari(group).len() == 0 {
                    return Some(m);
                }
            }
        }
        
        None
    }

}