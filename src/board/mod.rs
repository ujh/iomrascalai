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
use collections::hashmap::HashMap;

mod test;

#[deriving(Clone, Show, Eq)]
pub enum Color {
    White,
    Black,
    Empty
}

#[deriving(Clone, Eq, TotalEq, Hash)]
struct Coord {
    col: u8,
    row: u8
}

#[deriving(Clone)]
struct Chain{
    color: Color,
    coords: Vec<Coord>
}

#[deriving(Clone)]
pub struct Board {
    komi: f32,
    size: u8,
    board: HashMap<Coord, uint>,
    chains: Vec<Chain>
}

impl Coord {
    fn new(col: u8, row: u8) -> Coord {
        Coord {col: col, row: row}
    }
}

impl Chain {
    fn new(color: Color, first_coord: Coord) -> Chain {
        Chain {coords: vec!(first_coord), color: color}
    }

    fn add_stone(&mut self, coord: Coord) {
        self.coords.push(coord);
    }

    fn merge(&mut self, c: &Chain) {
        for coord in c.coords.iter() {
            self.coords.push(*coord)
        }
    }
}

impl Board {
    pub fn new(size: uint, komi: f32) -> Board {
        Board {
            komi: komi,
            size: size as u8,
            board: HashMap::new(),
            chains: Vec::new()
        }
    }

    // Note: This method uses 1-1 as the origin point, not 0-0. 19-19 is a valid coordinate in a 19-sized board, while 0-0 is not.
    //       this is done because I think it makes more sense in the context of go. (Least surprise principle, etc...)
    pub fn get(&self, col: u8, row: u8) -> Color {
        if self.is_inside(col, row) {
            let c = Coord::new(col, row);

            if self.board.contains_key(&c) {
                self.get_chain(*self.board.get(&c)).color
            } else {
                Empty
            }
        } else {
            fail!("You have requested a stone outside of the board");
        }
    }
    
    fn get_chain<'a>(&'a self, id: uint) -> &'a Chain {
        if id < self.chains.len() {
            self.chains.get(id)
        } else {
            fail!("You have requested a chain with an invalid id");
        }
    }

    fn get_mut_chain<'a>(&'a mut self, id: uint) -> &'a mut Chain {
        if id < self.chains.len()  {
            self.chains.get_mut(id)
        } else {
            fail!("You have requested a chain with an invalid id");
        }
    }

    fn create_chain(&mut self, color: Color, coord: Coord) {
        self.chains.push(Chain::new(color, coord));
        self.board.insert(coord, self.chains.len() - 1);
    }

    fn add_coord_to_chain(&mut self, coord: Coord, chain_id: uint) {
        self.get_mut_chain(chain_id).add_stone(coord);
        self.board.insert(coord, chain_id);
    }

    pub fn komi(&self) -> f32 {
        self.komi
    }

    fn is_inside(&self, col: u8, row: u8) -> bool {
        1 <= col && col <= self.size && 1 <= row && row <= self.size
    }


    // Note: Same as get(), the board is indexed starting at 1-1
    pub fn play(&self, color: Color, col: u8, row: u8) -> Board {
        // We check the validity of the coords.
        let mut new_board = if self.is_inside(col, row) {
            self.clone()
        } else {
            fail!("The coordinate you have entered ({} {}) are invalid", col, row);
        };

        let new_coords = Coord::new(col, row);

        // We find each neighbouring chain of the same color.
        let mut chains_ids_to_merge: Vec<uint> = new_board.neighbours(new_coords).iter()
            .filter_map(|coord|
                if new_board.board.contains_key(coord) && new_board.get_chain(*new_board.board.get(coord)).color == color{
                    Some(*new_board.board.get(coord))    
                } else {
                    None
                })
            .collect();

        chains_ids_to_merge.sort();
        chains_ids_to_merge.dedup();

        /*
         * If there is 0 friendly neighbouring chain, we create one, and assign the coord played to that new chain.
         * If there is 1, we assign the stone to that chain.
         * If there are more, we assign the stone to one chain, then merge the others into that chain, then remove the old chains from
         * board.chains, then we lower by 1 the ids of all stones with chain ids higher than the removed chains,
         * and finally we reassign the correct chain_id to each stone in the final chain.
        */
        match chains_ids_to_merge.len() {
            0 => new_board.create_chain(color, new_coords),
            1 => new_board.add_coord_to_chain(new_coords, *chains_ids_to_merge.get(0)),
            _ => {
                let final_chain_id = *chains_ids_to_merge.get(0);

                // We assign the stone to the final chain
                new_board.add_coord_to_chain(new_coords, final_chain_id);
                
                for &other_chain_id in chains_ids_to_merge.slice(1, chains_ids_to_merge.len()).iter() {
                    // We merge the other chains into the final chain. Clone() is needed as we borrow new_board both mutably
                    // and immutably.
                    let chain_copy = new_board.get_chain(other_chain_id).clone();
                    new_board.get_mut_chain(final_chain_id).merge(&chain_copy);

                    // Remove the old chain.
                    new_board.chains.remove(other_chain_id);

                    // We reduce by 1 every id stored in the board map which has an id higher than the other_chain_id
                    for (_, id) in new_board.board.mut_iter() {
                        if *id >= other_chain_id {
                            *id -= 1;
                        }
                    } 

                    // We update each coord key in the board map with the id of the final chain
                    let coords_to_update = new_board.get_chain(final_chain_id).coords.clone();
                    for &c in coords_to_update.iter() {
                        new_board.board.insert(c, final_chain_id);
                    }
                }
            }
        }

        new_board
    }

    fn neighbours<'a>(&'a self, c: Coord) -> Vec<Coord> {
        let mut neighbours = Vec::new();

        for i in range(-1,2) {
            for j in range(-1,2) {
                if (i == 0 && j !=0) || (i != 0 && j == 0) {
                    let (col, row) = (c.col+i as u8, c.row+j as u8);

                    if self.is_inside(col, row) { neighbours.push(Coord::new(col, row)); }
                }
            }
        }

        neighbours
    }

    pub fn show(&self) {
        // First we print the board
        for row in range(1u8, self.size+1).rev() {

            // Prints the row number
            print!("{:2} ", row);

            // Prints the actual row
            for col in range(1u8, self.size+1) {
                if self.get(col, row) == Empty {
                    let hoshis = &[4u8,10,16];
                    if   hoshis.contains(&row) && hoshis.contains(&col) {print!("+ ")}
                    else                                                {print!(". ")}
                } else if self.get(col, row) == White {print!("O ")}
                  else if self.get(col, row) == Black {print!("X ")}
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
            for p in c.coords.iter() {
                print!("{},{}|", p.col, p.row);
            }
            println!("");
        }
    }
}
