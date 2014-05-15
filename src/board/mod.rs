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

mod test;

#[deriving(Clone, Show, Eq)]
pub enum Color {
    White,
    Black,
    Empty
}

#[deriving(Clone, Eq)]
struct Coord {
    col: u8,
    row: u8
}

#[deriving(Clone, Eq)]
struct Stone {
    coord: Coord,
    color: Color,

    // This is only meaningful if the Stone's color is not Empty. If it is, then chain_id could be anything.
    chain_id: uint 
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
    board: Vec<Stone>,
    chains: Vec<Chain>
}

impl Stone {
    fn with_color(c: Color, chain_id: uint, col: u8, row: u8) -> Stone {
        Stone {color: c, chain_id: chain_id, coord: Coord {col: col, row: row}}
    }
}

impl Chain {
    fn new(first_stone: &Stone) -> Chain {
        Chain {coords: vec!(first_stone.coord), color: first_stone.color}
    }

    fn add_stone(&mut self, s: &Stone) {
        self.coords.push(Coord {col: s.coord.col, row: s.coord.row})
    }

    fn merge(&mut self, c: &Chain) {
        for coord in c.coords.iter() {
            self.coords.push(coord.clone())
        }
    }
}

impl Board {
    pub fn new(size: uint, komi: f32) -> Board {
        let empty_board = Vec::from_fn(size*size, |i| Stone::with_color(Empty, 0, (i%size+1) as u8, (i/size+1) as u8));

        Board {
            komi: komi,
            size: size as u8,
            board: empty_board,
            chains: Vec::new()
        }
    }

    // Note: This method uses 1-1 as the origin point, not 0-0. 19-19 is a valid coordinate in a 19-sized board, while 0-0 is not.
    //       this is done because I think it makes more sense in the context of go. (Least surprise principle, etc...)
    pub fn get<'a>(&'a self, col: u8, row: u8) -> Option<&'a Stone> {
        if 1 <= col && col <= self.size && 1 <= row && row <= self.size {
            Some(self.board.get((row as uint - 1) * self.size as uint + (col as uint - 1)))
        } else {
            None
        }
    }

    fn get_mut<'a>(&'a mut self, col: u8, row: u8) -> Option<&'a mut Stone> {
        if 1 <= col && col <= self.size && 1 <= row && row <= self.size {
            Some(self.board.get_mut((row as uint - 1) * self.size as uint + (col as uint - 1)))
        } else {
            None
        }
    }
    
    fn get_chain<'a>(&'a self, id: uint) -> Option<&'a Chain> {
        if id < self.chains.len() {
            Some(self.chains.get(id))
        } else {
            None
        }
    }

    fn get_mut_chain<'a>(&'a mut self, id: uint) -> Option<&'a mut Chain> {
        if id < self.chains.len()  {
            Some(self.chains.get_mut(id))
        } else {
            None
        }
    }

    fn add_chain(&mut self, s: &Stone) -> uint {
        self.chains.push(Chain::new(s));
        self.chains.len() - 1
    }

    pub fn komi(&self) -> f32 {
        self.komi
    }

    // Note: Same as get(), the board is indexed starting at 1-1
    pub fn play(&self, c: Color, col: u8, row: u8) -> Board {
        // We check the validity of the coords.
        let mut new_board = if self.get(col, row).is_some() {
            self.clone()
        } else {
            fail!("The coordinate you have entered ({} {}) are invalid", col, row);
        };

        // We update the color of the Stone.
        new_board.get_mut(col, row).unwrap().color = c;


        // We find each neighbouring chain of the same color.
        let mut chains_ids_to_merge: Vec<uint> = new_board.neighbours(new_board.get(col, row).unwrap()).iter()
            .filter_map(|n| if n.color == c {Some(n.chain_id)} else {None})
            .collect();
        chains_ids_to_merge.sort();
        chains_ids_to_merge.dedup();

        /*
         * If there is 0 friendly neighbouring chain, we create one, and assign the Stone played to that new chain.
         * If there is 1, we assign the stone to that chain.
         * If there are more, we assign the stone to one chain, then merge the others into that chain, then remove the old chains from
         * board.chains, then we lower by 1 the ids of all stones with chain ids higher than the removed chains,
         * and finally we reassign the correct chain_id to each stone in the final chain.
        */
        match chains_ids_to_merge.len() {
            0 => {
                // The cloning is needed as the next line borrows new_board mutably with add_chain(), preventing also borrowing elsewhere
                let stone_copy = new_board.get(col, row).unwrap().clone();

                let new_id = new_board.add_chain(&stone_copy);
                new_board.get_mut(col, row).unwrap().chain_id = new_id;
            }
            1 => {
                // Same thing as higher
                let stone_copy = new_board.get(col, row).unwrap().clone();
                let new_id     = *chains_ids_to_merge.get(0);

                new_board.get_mut_chain(new_id).unwrap().add_stone(&stone_copy);
                new_board.get_mut(col, row).unwrap().chain_id = new_id;
            }
            _ => {
                // We assign the stone to the final chain
                let stone_copy = new_board.get(col, row).unwrap().clone();
                let new_id     = *chains_ids_to_merge.get(0);
                new_board.get_mut_chain(new_id).unwrap().add_stone(&stone_copy);
                new_board.get_mut(col, row).unwrap().chain_id = new_id;

                
                for &other_chain_id in chains_ids_to_merge.slice(1, chains_ids_to_merge.len()).iter() {
                    // We merge the other chains into the final chain
                    let chain_copy = new_board.get_chain(other_chain_id).unwrap().clone();
                    new_board.get_mut_chain(new_id).unwrap().merge(&chain_copy);

                    // Remove the old chain.
                    new_board.chains.remove(other_chain_id);

                    // Decrement the chain_id of Stones with a chain_id >= to the id of the removed chain (preserves integrity)
                    for stone in new_board.board.mut_iter() {
                        if stone.color != Empty && stone.chain_id >= other_chain_id { stone.chain_id -= 1; }
                    }

                    // We update each stone pointed to by the final chain to have the correct id
                    let coords_to_update = new_board.get_chain(new_id).unwrap().coords.clone();
                    for c in coords_to_update.iter() {
                        new_board.get_mut(c.col, c.row).unwrap().chain_id = new_id;
                    }
                }
            }
        }

        new_board
    }

    fn neighbours<'a>(&'a self, p: &Stone) -> Vec<&'a Stone> {
        let mut neighbours = Vec::new();

        for i in range(-1,2) {
            for j in range(-1,2) {
                if (i == 0 && j !=0) || (i != 0 && j == 0) {
                    let n = self.get(p.coord.col+i as u8, p.coord.row+j as u8);

                    if n.is_some() { neighbours.push(n.unwrap()); }
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
                if self.get(col, row).unwrap().color == Empty {
                    let hoshis = &[4u8,10,16];
                    if   hoshis.contains(&row) && hoshis.contains(&col) {print!("+ ")}
                    else                                                {print!(". ")}
                } else if self.get(col, row).unwrap().color == White {print!("O ")}
                  else if self.get(col, row).unwrap().color == Black {print!("X ")}
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
