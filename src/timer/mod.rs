/************************************************************************
 *                                                                      *
 * Copyright 2015 Urban Hafner, Thomas Poinsot, Igor Polyakov           *
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

use config::Config;
use game::Info;

use std::cmp::max;
use std::sync::Arc;
use time::precise_time_ns;

mod test;

#[derive(Clone)]
struct Clock {
    start: Option<u64>,
    end:   Option<u64>,
}

impl Clock {

    pub fn new() -> Clock {
        Clock {
            start: None,
            end:   None,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(precise_time_ns());
        self.end   = None;
    }

    pub fn stop(&mut self) {
        self.end = Some(precise_time_ns());
    }

    fn time_elapsed_in_ms(&self) -> u32 {
        match self.start {
            Some(start) => {
                match self.end {
                    Some(end) => ((end - start) / 1000000) as u32,
                    None      => 0
                }
            },
            None => 0
        }
    }

}

#[derive(Clone)]
pub struct Timer {
    pub byo_stones: i32, // stones per byo yomi period
    byo_stones_left: i32,
    pub byo_time: u32, // byo yomi time in ms
    byo_time_left: u32,
    pub main_time: u32, // main time in ms
    main_time_left: u32,
    clock: Clock,
    config: Arc<Config>,
}

impl Timer {

    pub fn new(config: Arc<Config>) -> Timer {
        Timer {
            byo_stones: 0,
            byo_stones_left: 0,
            byo_time: 0,
            byo_time_left: 0,
            main_time: 300000, // 5min
            main_time_left: 300000,
            clock: Clock::new(),
            config: config,
        }

    }

    pub fn reset(&mut self) {
        self.main_time_left  = self.main_time;
        self.byo_time_left   = self.byo_time;
        self.byo_stones_left = self.byo_stones;
        self.clock.stop();
    }

    pub fn setup(&mut self, main_in_s: u32, byo_in_s: u32, stones: i32) {
        self.set_main_time(main_in_s * 1000);
        self.set_byo_time(byo_in_s * 1000);
        self.set_byo_stones(stones);
        self.clock.stop();
    }

    pub fn update(&mut self, time_in_s: u32, stones: i32) {
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

    fn adjust_time(&mut self) {
        let time_elapsed = self.clock.time_elapsed_in_ms();

        if time_elapsed > self.main_time_left {
            let overtime_spent = time_elapsed - self.main_time_left;
            self.main_time_left = 0;

            if overtime_spent > self.byo_time_left {
                self.byo_time_left = 0;
                self.byo_stones_left = 0;
            } else {
                self.byo_time_left -= overtime_spent;
                self.byo_stones_left -= 1;
                if self.byo_stones_left == 0 {
                    self.byo_time_left = self.byo_time;
                    self.byo_stones_left = self.byo_stones;
                }
            }
        } else {
            self.main_time_left -= time_elapsed;
        }
    }

    pub fn main_time_left(&self) -> u32 {
        self.main_time_left
    }

    fn set_main_time(&mut self, time: u32) {
        self.main_time = time;
        self.main_time_left = time;
    }

    fn set_byo_time(&mut self, time: u32) {
        self.byo_time = time;
        self.byo_time_left = time;
    }

    fn set_byo_stones(&mut self, stones: i32) {
        self.byo_stones = stones;
        self.byo_stones_left = stones;
    }

    fn c(&self) -> f32 {
        self.config.time_control.c
    }

    pub fn budget<T: Info>(&self, game: &T) -> u32 {
        // If there's still main time left
        if self.main_time_left > 0 {
            // Assume at least 30 vacant points
            let vacant = max(game.vacant_point_count(), 30) as f32;
            (self.main_time_left as f32 / (self.c() * vacant)).floor() as u32
        } else if self.byo_time_left == 0 {
            0
        } else {
            // Else use byoyomi time
            (self.byo_time_left as f32 / self.byo_stones_left as f32).floor() as u32
        }
    }
}
