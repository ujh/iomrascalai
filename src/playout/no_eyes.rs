/************************************************************************
 *                                                                      *
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

use board::Board;
use board::Move;
use config::Config;
use super::Playout;

#[derive(Debug)]
pub struct NoEyesPlayout;

impl Playout for NoEyesPlayout {

    fn is_playable(&self, board: &Board, m: &Move) -> bool {
        !board.is_eye(&m.coord(), *m.color())
    }

    fn playout_type(&self) -> &'static str {
        "no-eyes"
    }
}

//don't self atari strings that will make an eye after dying, which is strings of 7+
#[derive(Debug)]
pub struct NoSelfAtariPlayout {
    config: Config
}

impl NoSelfAtariPlayout {

    pub fn new(config: Config) -> NoSelfAtariPlayout {
        NoSelfAtariPlayout { config: config }
    }

    fn cutoff(&self) -> usize {
        self.config.playout.no_self_atari_cutoff
    }

}

impl Playout for NoSelfAtariPlayout {

    fn is_playable(&self, board: &Board, m: &Move) -> bool {
        !board.is_eye(&m.coord(), *m.color())
        && {
            let empty = board.liberty_count(m.coord()); //empty coordinates next to the move
            empty > 1 ||    //then we are definitely going to be fine for now
            {
                let (removes_stone, removes_stones) = board.removes_multiple_enemy_neighbouring_stones(*m); //do we capture at least one stone

                (empty > 0 && removes_stone) || //if we have a liberty and capture a stone we're not in a snapback
                removes_stones || //or if we capture two we're not going to be recaptured immediately
                //unless it's a multiple step snapback
                {
                (removes_stone && board.new_chain_liberties_greater_than_zero(*m)) || //one liberty by connecting and capture one stone is not a snapback
                board.new_chain_liberties_greater_than_one(*m) //two liberties by not connecting so not connect and die
                }
            }
            || board.new_chain_length_less_than(*m, self.cutoff()) //suicide for smaller groups is ok
        }
    }

    fn playout_type(&self) -> &'static str {
        "no-self-atari"
    }

}
