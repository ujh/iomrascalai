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
use board::chain::Chain;
use board::coord::Coord;

mod board_test;
mod coord_test;
mod chain_test;

mod coord;
mod chain;

#[deriving(Clone, Show, Eq)]
pub enum Color {
    White,
    Black,
    Empty
}

#[deriving(Clone)]
pub struct Board {
    komi: f32,
    size: u8,
    board: Vec<uint>,
    chains: Vec<Chain>
}

impl Board {
    pub fn new(size: uint, komi: f32) -> Board {
        Board {
            komi: komi,
            size: size as u8,
            board: Vec::from_fn(size*size, |_| 0),
            chains: vec!(Chain::new(0, Empty))
        }
    }

    // Note: This method uses 1-1 as the origin point, not 0-0. 19-19 is a valid coordinate in a 19-sized board, while 0-0 is not.
    //       this is done because I think it makes more sense in the context of go. (Least surprise principle, etc...)
    pub fn get(&self, col: u8, row: u8) -> Color {
        if self.is_inside(col, row) {
            self.get_chain(col, row).color
        } else {
            fail!("You have requested a stone outside of the board");
        }
    }

    pub fn get_chain<'a>(&'a self, col: u8, row: u8) -> &'a Chain {
        if self.is_inside(col, row) {
            let chain_id = *self.board.get(Coord::new(col, row).to_index(self.size));
            self.chains.get(chain_id)
        } else {
            fail!("You have requested a chain outside of the board");
        }
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

        let mut neighbouring_chains_ids = Vec::new();
        for coord in new_coords.neighbours().iter().filter(|c| new_board.is_inside(c.col, c.row) && new_board.get(c.col, c.row) == color) {
            let candidate_chain_id = new_board.get_chain(coord.col, coord.row).id;
            if !neighbouring_chains_ids.contains(&candidate_chain_id) {neighbouring_chains_ids.push(candidate_chain_id);}
        }

        /*
         * If there is 0 friendly neighbouring chain, we create one, and assign the coord played to that new chain.
         * If there is 1, we assign the stone to that chain.
         * If there are more, we assign the stone to one chain, then merge the others into that chain, then remove the old chains from
         * board.chains, then we lower by 1 the ids of all stones with chain ids higher than the removed chains,
         * and finally we reassign the correct chain_id to each stone in the final chain.
        */
        match neighbouring_chains_ids.len() {
            0 => {
                let new_chain_id  = new_board.chains.len();
                let mut new_chain = Chain::new(new_chain_id, color);
                new_chain.add_stone(new_coords);
                new_board.chains.push(new_chain);
                *new_board.board.get_mut(new_coords.to_index(new_board.size)) = new_chain_id;
            },
            1 => {
                let final_chain_id = *neighbouring_chains_ids.get(0);
                new_board.chains.get_mut(final_chain_id).add_stone(new_coords);
                *new_board.board.get_mut(new_coords.to_index(new_board.size)) = final_chain_id;
            },
            _ => {
                let final_chain_id        = *neighbouring_chains_ids.get(0);
                let mut nb_removed_chains = 0;

                // We assign the stone to the final chain
                new_board.chains.get_mut(final_chain_id).add_stone(new_coords);
                *new_board.board.get_mut(new_coords.to_index(new_board.size)) = final_chain_id;
                
                for &other_chain_id in neighbouring_chains_ids.slice(1, neighbouring_chains_ids.len()).iter() {
                    // We merge the other chain into the final chain.
                    let other_chain = &new_board.chains.get(other_chain_id-nb_removed_chains).clone();
                    new_board.chains.get_mut(final_chain_id-nb_removed_chains).merge(other_chain);

                    // We remove the old chain.
                    new_board.chains.remove(other_chain_id-nb_removed_chains);

                    // We decrease by one every index in board that is higher than other_chain_id
                    for ind in new_board.board.mut_iter() {
                        if *ind > other_chain_id-nb_removed_chains {*ind -= 1;}
                    }

                    // We decrease by one every index in chains that is higher than other_chain_id
                    for chain in new_board.chains.mut_iter() {
                        if chain.id > other_chain_id-nb_removed_chains {chain.id -= 1;}
                    }

                    // Now that there is one less chain in the index, we have to decrease final_chain_id as well
                    nb_removed_chains += 1;
                }

                // We update each coord key in the board map with a ref of the final chain
                for &c in new_board.chains.get(final_chain_id-nb_removed_chains).coords().clone().iter() {
                    *new_board.board.get_mut(c.to_index(new_board.size)) = final_chain_id-nb_removed_chains;
                }
            }
        }

        new_board
    }

    pub fn show(&self) {
        println!("komi: {}", self.komi());

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
            println!("{}", c.show());
        }
    }
}
