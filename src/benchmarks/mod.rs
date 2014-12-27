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
extern crate time;

use engine::RandomEngine;
use game::Game;
use ruleset::KgsChinese;
use playout::Playout;
use self::time::get_time;

pub fn pps(size: u8, runtime: uint) {
    let engine = RandomEngine::new();
    let game   = Game::new(size, 6.5, KgsChinese);
    let playout_engine = Playout::new(&engine);
    let mut counter = 0;
    let start = get_time().sec;

    loop {
        playout_engine.run(&game);
        counter += 1;

        if(get_time().sec - start >= runtime as i64) {
            break;
        }
    }

    println!("Playout per second: {}", counter/runtime);
}
