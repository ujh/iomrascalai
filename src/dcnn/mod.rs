/************************************************************************
 *                                                                      *
 * Copyright 2016 Urban Hafner                                          *
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

use board::Black;
use board::Board;
use board::Coord;
use board::Empty;
use board::White;
use engine::Prior;

use caffe::Net;
use caffe::Phase;
use std::path::Path;

pub struct DCNN {
    net: Net
}

impl DCNN {

    pub fn new() -> DCNN {
        let mut net = Net::new(Path::new("/Users/uh/Personal/detlefs-caffe-dcnn/movepredict.prototxt"), Phase::Test);
        net.copy_trained_layers_from(Path::new("/Users/uh/Personal/detlefs-caffe-dcnn/movepredict.caffemodel"));
        DCNN { net: net }
    }

    pub fn update_priors(&mut self, priors: Vec<Prior>, board: &Board) -> Vec<Prior> {
        self.set_nn_input(board);
        self.run_net_and_update_priors(priors, board)
    }

    fn run_net_and_update_priors(&mut self, mut priors: Vec<Prior>, board: &Board) -> Vec<Prior> {
        self.net.forward_prefilled();
        let prop_blob = self.net.blob("ip_zw");
        let output = prop_blob.as_slice();
        for coord in &Coord::for_board_size(board.size()) {
            let (j,k) = self.coord_as_dcnn_indexes(coord);
            let value = output[j*19+k];
            // TODO: Move this somewhere else so that we can avoid the search
            match priors.iter_mut().find(|p| !p.pass_move() && p.coord() == *coord) {
                Some(prior) => {
                    prior.record_even_prior((1000.0 * value).round() as usize);
                }
                None => {}
            }
        }
        priors
    }

    fn set_nn_input(&mut self, board: &Board) {
        self.net.blob("data").set_data(&mut self.calculate_nn_input(board));
    }

    fn calculate_nn_input(&self, board: &Board) -> [f32; 722] {
        let mut input = self.empty_input();
        let size = board.size() as usize;
        if board.next_player() == Black {
            for coord in &Coord::for_board_size(board.size()) {
                let (j,k) = self.coord_as_dcnn_indexes(coord);
                match board.color(coord) {
                    Black => {
                        input[j*size+k] = 1.0;
                    },
                    White => {
                        input[size*size+size*j+k] = 1.0;
                    },
                    Empty => {},
                }
            }
        } else {
            for coord in &Coord::for_board_size(board.size()) {
                let (j,k) = self.coord_as_dcnn_indexes(coord);
                match board.color(coord) {
                    White => {
                        input[j*size+k] = 1.0;
                    },
                    Black => {
                        input[size*size+size*j+k] = 1.0;
                    },
                    Empty => {},
                }
            }
        }
        input
    }

    // Our Coords start at 1 whereas the DCNN input starts at zero
    fn coord_as_dcnn_indexes(&self, coord: &Coord) -> (usize, usize) {
        (coord.col as usize - 1, coord.row as usize - 1)
    }

    // Return an empty input of size 19*19*2. That way all inputs for boards smaller than 19x19 and
    // for empty points on the board are set to zero already.
    fn empty_input(&self) -> [f32; 722] {
        [0.0; 722]
    }
}
