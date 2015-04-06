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
use super::Playout;

#[derive(Debug)]
pub struct NoEyesPlayout;

impl Playout for NoEyesPlayout {

    fn is_playable(&self, board: &Board, m: &Move) -> bool {
        !board.is_eye(&m.coord(), *m.color())
    }

    fn playout_type(&self) -> String {
        format!("{:?}", self)
    }
}

//strings of 7 or more don't play self-atari in this playout
#[derive(Debug)]
pub struct NoSelfAtariPlayout;

impl Playout for NoSelfAtariPlayout {

    fn is_playable(&self, board: &Board, m: &Move) -> bool {
        !board.is_eye(&m.coord(), *m.color())
        && {
            let empty = board.liberty_count(m.coord());
            empty > 1 ||
            {
                let removed_enemies = board.removes_enemy_neighbouring_stones(*m);
                
                empty + removed_enemies > 1 ||
                {
                (removed_enemies > 0 && board.new_chain_liberties_greater_than(*m, 0)) ||
                board.new_chain_liberties_greater_than(*m, 1)
                }
            }
            || board.new_chain_length_less_than(*m, 3) //you can self-atari one or two stones in playouts
        }
    }
    
    fn playout_type(&self) -> String {
        format!("{:?}", self)
    }
}
