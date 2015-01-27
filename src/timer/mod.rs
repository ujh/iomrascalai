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

mod test;

pub struct Timer {
    main_time: u64, // main time in ms
    byo_time: u64, // byo yomi time in ms
    byo_stones: i32, // stones per byo yomi period
    byo_stones_remaining: i32,
}

impl Timer {

    pub fn new() -> Timer {
        Timer {
            main_time: 0,
            byo_time: 30,
            byo_stones: 1,
            byo_stones_remaining: 1,
        }

    }

    pub fn play(&mut self) {
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

}
