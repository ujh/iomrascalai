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
use std::num::SignedInt;
use time::precise_time_ns;

mod test;

struct Clock {
    start: Option<i64>,
    end:   Option<i64>,
}

impl Clock {

    pub fn new() -> Clock {
        Clock {
            start: None,
            end:   None,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(precise_time_ns() as i64);
        self.end   = None;
    }

    pub fn stop(&mut self) {
        self.end = Some(precise_time_ns() as i64);
    }

    fn time_elapsed_in_ms(&self) -> i64 {
        match self.start {
            Some(start) => {
                match self.end {
                    Some(end) => (end - start) / 1000000,
                    None      => 0
                }
            },
            None => 0
        }
    }

    pub fn stopped(&self) -> bool {
        !self.running()
    }

    pub fn running(&self) -> bool {
        self.start.is_some() && self.end.is_none()
    }
}

pub struct Timer {
    byo_stones: i32, // stones per byo yomi period
    byo_stones_left: i32,
    byo_time: i64, // byo yomi time in ms
    byo_time_left: i64,
    main_time: i64, // main time in ms
    main_time_left: i64,
    clock: Clock,
}

impl Timer {

    pub fn new() -> Timer {
        Timer {
            byo_stones: 0,
            byo_stones_left: 0,
            byo_time: 0,
            byo_time_left: 0,
            main_time: 300000, // 5min
            main_time_left: 300000,
            clock: Clock::new(),
        }

    }

    pub fn reset(&mut self) {
        self.main_time_left  = self.main_time;
        self.byo_time_left   = self.byo_time;
        self.byo_stones_left = self.byo_stones;
        self.clock.stop();
    }

    pub fn setup(&mut self, main_in_s: i64, byo_in_s: i64, stones: i32) {
        self.set_main_time(main_in_s * 1000);
        self.set_byo_time(byo_in_s * 1000);
        self.set_byo_stones(stones);
        self.clock.stop();
    }

    pub fn update(&mut self, time_in_s: i64, stones: i32) {
        if stones == 0 {
            self.main_time_left = time_in_s * 1000;
        } else {
            self.main_time_left  = 0;
            self.byo_time_left   = time_in_s * 1000;
            self.byo_stones_left = stones;
        }
        self.start();
    }

    pub fn start(&mut self) {
        self.clock.start();
    }

    pub fn stop(&mut self) {
        self.clock.stop();
        self.adjust_time();
    }

    fn new_time_left(&self) -> i64 {
        self.main_time_left - self.clock.time_elapsed_in_ms()
    }

    fn adjust_time(&mut self) {
        let new_time_left = self.new_time_left();
        if new_time_left < 0 {
            let overtime_spent = new_time_left.abs();
            self.main_time_left = 0;
            self.byo_time_left -= overtime_spent;
            if self.byo_time_left < 0 {
                self.byo_time_left = 0;
                self.byo_stones_left = 0;
            } else {
                self.byo_stones_left -= 1;
                if self.byo_stones_left == 0 {
                    self.byo_time_left = self.byo_time;
                    self.byo_stones_left = self.byo_stones;
                }
            }
        } else {
            self.main_time_left = new_time_left;
        }
    }

    pub fn main_time(&self) -> i64 {
        self.main_time
    }

    pub fn main_time_left(&self) -> i64 {
        self.main_time_left
    }

    fn set_main_time(&mut self, time: i64) {
        self.main_time = time;
        self.main_time_left = time;
    }

    pub fn byo_time(&self) -> i64 {
        self.byo_time
    }

    fn set_byo_time(&mut self, time: i64) {
        self.byo_time = time;
        self.byo_time_left = time;
    }

    pub fn byo_stones(&self) -> i32 {
        self.byo_stones
    }

    fn set_byo_stones(&mut self, stones: i32) {
        self.byo_stones = stones;
        self.byo_stones_left = stones;
    }

    fn C(&self) -> f32 {
        0.5
    }

    pub fn budget<T: Info>(&self, game: &T) -> i64 {
        // If there's still main time left
        if self.main_time_left > 0 {
            // TODO: Are there issues with all these values being ints?
            (self.main_time_left as f32 / (self.C() * game.vacant_point_count() as f32)).floor() as i64
        } else if self.byo_time_left == 0 {
            0
        } else {
            // Else use byoyomi time
            // TODO: Does that need to be adjusted wrt to time_left?
            (self.byo_time_left as f32 / self.byo_stones_left as f32).floor() as i64
        }
    }
}
