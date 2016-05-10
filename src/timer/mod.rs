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
use time::Duration;
use time::PreciseTime;

mod test;

#[derive(Clone)]
pub struct Timer {
    byo_stones: i32,
    byo_stones_left: i32,
    byo_time: i64,
    byo_time_left: i64,
    config: Arc<Config>,
    current_budget: Duration,
    main_time_left: i64,
    time_stamp: PreciseTime,
}

impl Timer {

    pub fn new(config: Arc<Config>) -> Timer {
        Timer {
            byo_stones: 0,
            byo_stones_left: 0,
            byo_time: 0,
            byo_time_left: 0,
            config: config,
            current_budget: Duration::milliseconds(0),
            main_time_left: 0,
            time_stamp: PreciseTime::now(),
        }

    }

    pub fn setup(&mut self, main_in_s: i64, byo_in_s: i64, stones: i32) {
        self.set_main_time(main_in_s * 1000);
        self.set_byo_time(byo_in_s * 1000);
        self.set_byo_stones(stones);
        self.reset_time_stamp();
    }

    pub fn update(&mut self, time_in_s: i64, stones: i32) {
        if stones == 0 {
            self.main_time_left = time_in_s * 1000;
        } else {
            self.main_time_left  = 0;
            self.byo_time_left   = time_in_s * 1000;
            self.byo_stones_left = stones;
        }
        self.reset_time_stamp();
    }

    pub fn start<T: Info>(&mut self, game: &T) {
        self.reset_time_stamp();
        let budget = self.budget(game);
        self.current_budget = budget;
        let msg = format!(
            "Thinking for {}ms ({}ms time left)",
            budget.num_milliseconds(),
            self.main_time_left());
        self.config.log(msg);
    }

    pub fn ran_out_of_time(&self, win_ratio: f32) -> bool {
        let fastplay_budget = (1.0 / self.config.time_control.fastplay_budget).floor() as i32;
        let budget5 = self.current_budget / fastplay_budget;
        let elapsed = self.elapsed();
        if elapsed > budget5 && win_ratio > self.config.time_control.fastplay_threshold {
            self.config.log(format!("Search stopped early. Fastplay rule triggered."));
            true
        } else {
            elapsed > self.current_budget
        }
    }

    pub fn stop(&mut self) {
        self.adjust_time();
    }

    pub fn reset(&mut self) {
        // Do nothing
    }

    pub fn byo_stones_left(&self) -> i32 {
        self.byo_stones_left
    }

    pub fn byo_time_left(&self) -> i64 {
        self.byo_time_left
    }

    pub fn main_time_left(&self) -> i64 {
        self.main_time_left
    }

    fn set_main_time(&mut self, time: i64) {
        self.main_time_left = time;
    }

    fn set_byo_time(&mut self, time: i64) {
        self.byo_time = time;
        self.byo_time_left = time;
    }

    fn set_byo_stones(&mut self, stones: i32) {
        self.byo_stones = stones;
        self.byo_stones_left = stones;
    }

    fn elapsed(&self) -> Duration {
        self.time_stamp.to(PreciseTime::now())
    }

    fn reset_time_stamp(&mut self) {
        self.time_stamp = PreciseTime::now();
    }

    fn adjust_time(&mut self) {
        let time_elapsed = self.elapsed().num_milliseconds();

        if time_elapsed > self.main_time_left {
            let overtime_spent = time_elapsed - self.main_time_left;
            self.main_time_left = 0;

            if overtime_spent > self.byo_time_left() {
                self.byo_time_left = 0;
                self.byo_stones_left = 0;
            } else {
                self.byo_time_left -= overtime_spent;
                self.byo_stones_left -= 1;
                if self.byo_stones_left() == 0 {
                    self.byo_time_left = self.byo_time;
                    self.byo_stones_left = self.byo_stones;
                }
            }
        } else {
            self.main_time_left -= time_elapsed;
        }
    }

    fn c(&self) -> f32 {
        self.config.time_control.c
    }

    fn budget<T: Info>(&self, game: &T) -> Duration {
        // If there's still main time left
        let ms = if self.main_time_left > 0 {
            let min_stones = self.config.time_control.min_stones as u16;
            let vacant = max(game.vacant_point_count(), min_stones) as f32;
            (self.main_time_left as f32 / (self.c() * vacant)).floor() as i64
        } else if self.byo_time_left() == 0 {
            0
        } else {
            // Else use byoyomi time
            (self.byo_time_left() as f32 / self.byo_stones_left() as f32).floor() as i64
        };
        Duration::milliseconds(ms)
    }
}
