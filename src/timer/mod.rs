/************************************************************************
 *                                                                      *
 * Copyright 2015 Urban Hafner, Thomas Poinsot                          *
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

use game::Info;

use time::PreciseTime;

mod test;

pub struct Timer {
    main_time: u64, // main time in ms
    byo_time: u64, // byo yomi time in ms
    byo_stones: i32, // stones per byo yomi period
    byo_stones_remaining: i32,
    start_time: PreciseTime,
}

impl Timer {

    pub fn new() -> Timer {
        Timer {
            main_time: 0,
            byo_time: 30,
            byo_stones: 1,
            byo_stones_remaining: 1,
            start_time: PreciseTime::now(),
        }

    }

    pub fn start(&mut self) {
        self.start_time = PreciseTime::now()
    }

    pub fn stop(&mut self) {
        let time_left = self.main_time();
        let new_time_left = if time_left > self.start_time.to(PreciseTime::now()).num_milliseconds() as u64 {
            time_left - self.start_time.to(PreciseTime::now()).num_milliseconds() as u64
        } else {
            0u64
        };
        self.set_main_time(new_time_left);
        self.byo_stones_remaining = if self.byo_stones_remaining == 1 {
            self.byo_stones
        } else {
            self.byo_stones_remaining - 1
        }
    }

    pub fn main_time(&self) -> u64 {
        self.main_time
    }

    pub fn set_main_time(&mut self, time: u64) {
        self.main_time = time
    }

    pub fn byo_time(&self) -> u64 {
        self.byo_time
    }

    pub fn set_byo_time(&mut self, time: u64) {
        self.byo_time = time;
    }

    pub fn byo_stones(&self) -> i32 {
        self.byo_stones
    }

    pub fn set_byo_stones(&mut self, stones: i32) {
        let actual = if stones < 1 { 1 } else { stones };
        self.byo_stones = actual;
        self.byo_stones_remaining = actual;
    }

    pub fn budget<T: Info>(&self, game: &T) -> u64 {
        let max_time = match (self.main_time(), self.byo_time()) {
            (main, byo) if main == 0 => { // We're in byo-yomi
                byo / self.byo_stones() as u64
            }
            (main, byo) if byo == 0  => { // We have an absolute clock
                let weighted_board_size = (game.board_size() * game.board_size()) as f64  * 1.5f64;
                let est_max_nb_move_left = weighted_board_size as u64 - game.move_number() as u64;
                main / est_max_nb_move_left
            }
            (main, _)   if main > 0  => {
                // Dumb strategy for the moment, we use the main time to play about the first half of the game;
                let est_half_game = game.board_size() as u64 * game.board_size()  as u64 / 2 - game.move_number() as u64;
                main / est_half_game
            }
            (main, byo) => panic!("The timer run into a strange configuration: main time: {}, byo time: {}", main, byo)
        };

        let lag_time = 1000;
        if max_time < lag_time {
            // If we have less than lag time to think, try to use half of it.
            lag_time / 2
        } else {
            max_time - lag_time
        }
    }
}
