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

use std::num::Float;
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
            main_time: 300000, // 5min
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

    fn C(&self) -> f32 {
        0.5
    }

    pub fn budget<T: Info>(&self, game: &T) -> u64 {
        // If there's still main time left
        if self.main_time() > 0 {
            // TODO: Are there issues with all these values being ints?
            (self.main_time() as f32 / (self.C() * game.vacant_point_count() as f32)).floor() as u64
        } else {
            // Else use byoyomi time
            // TODO: Does that need to be adjusted wrt to time_left?
            (self.byo_time() as f32 / self.byo_stones() as f32).floor() as u64
        }
    }
}
