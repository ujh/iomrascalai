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
 
use board::{Board, Move};
use board::Color::Empty;
use board::coord::Coord;

use smallvec::SmallVec4;
 
 impl Board {
 
    pub fn is_not_self_atari(&self, m: &Move) -> bool {
        let empty = self.liberty_count(m.coord()); //empty coordinates next to the move
        
        empty > 1 ||    //then we are definitely going to be fine for now
        {
            let (removes_stone, removes_stones) = self.removes_multiple_enemy_neighbouring_stones(*m); //do we capture at least one stone

            (empty > 0 && removes_stone) || //if we have a liberty and capture a stone we're not in a snapback
            removes_stones || //or if we capture two we're not going to be recaptured immediately
            //unless it's a multiple step snapback
            {
                (removes_stone && self.new_chain_liberties_greater_than_zero(*m)) || //one liberty by connecting and capture one stone is not a snapback
                self.new_chain_liberties_greater_than_one(*m) //two liberties by not connecting so not connect and die
            }
        }
    }

    pub fn liberty_count(&self, c: Coord) -> usize {
        self.neighbours(c).iter().filter(|c| self.color(c) == Empty).count()
    }
    
    pub fn removes_multiple_enemy_neighbouring_stones(&self, m: Move) -> (bool, bool) {
        let enemy = m.color().opposite();
        let mut found_one = false;
        
        let chains = self.neighbours(m.coord()).iter()
            .filter(|c| self.color(c) == enemy)
            .map(|&c| self.get_chain(c).unwrap())
            .filter(|chain| chain.liberties().len() == 1);
        
        for chain in chains {
            if found_one || chain.coords().len() > 1 {
                return (true, true);
            } else {
                found_one = true;
            }
        }
        
        (found_one, false)
    }

    pub fn new_chain_liberties_greater_than_zero(&self, m: Move) -> bool {
        for &c in self.neighbours(m.coord()).iter() {
            if self.color(&c) == *m.color() {
                for &liberty in self.get_chain(c).unwrap().liberties() {
                    if liberty != m.coord() {
                        return true;
                    }
                }
            } else if self.color(&c) == Empty {
                return true;
            }
        }
        
        false
    }

    pub fn new_chain_liberties_greater_than_one(&self, m: Move) -> bool {
        let mut first_liberty: Option<Coord> = None;
        for &c in self.neighbours(m.coord()).iter() {
            if self.color(&c) == *m.color() {
                for &liberty in self.get_chain(c).unwrap().liberties() {
                    if liberty != m.coord() && first_liberty.is_none() {
                        first_liberty = Some(liberty);
                    } else if liberty != m.coord() && first_liberty.is_some() {
                        if Some(liberty) != first_liberty {
                            return true;
                        }
                    }
                }
            } else if self.color(&c) == Empty {
                if first_liberty.is_none() {
                    first_liberty = Some(c);
                } else if Some(c) != first_liberty {
                    return true;
                }
            }
        }
        
        false
    }
    
    pub fn new_chain_liberties_greater_than(&self, m: Move, limit: usize) -> bool {
        let liberty_iterator = self.neighbours(m.coord()).iter()
            .filter(|c| self.color(&c) == *m.color())
            .flat_map(|&c| self.get_chain(c).unwrap().liberties())
            .filter(|&liberty| *liberty != m.coord());
         
         let empty_iterator = self.neighbours(m.coord()).iter()
            .filter(|c| self.color(&c) == Empty);
         
         let mut liberties = SmallVec4::new();
         for liberty in liberty_iterator.chain(empty_iterator) {
            if !liberties.contains(liberty) {
                liberties.push(*liberty);
            }
            
            if liberties.len() > limit {
                return true;
            }
         }
         
         false
    }
    
    pub fn new_chain_length_less_than(&self, m: Move, limit: usize) -> bool {
        let mut chain_ids = SmallVec4::new();
        let mut length = 0;
        
        for &c in self.neighbours(m.coord()).iter() {
            if self.color(&c) == *m.color() {
                let chain = self.get_chain(c).unwrap();
                
                if !chain_ids.contains(&chain.id()) {
                    length += chain.coords().len();
                    chain_ids.push(chain.id());
                }
   
                if length + 1 > limit {
                    return false;
                }
            }
        }
        
        true
    }
 
 }