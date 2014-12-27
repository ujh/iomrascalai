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
#![feature(phase)]
extern crate core;
extern crate getopts;
extern crate rand;
extern crate regex;
#[phase(plugin)]
extern crate regex_macros;

use engine::Engine;
use engine::McEngine;
use engine::RandomEngine;
use getopts::getopts;
use getopts::optopt;
use std::ascii::OwnedAsciiExt;
use std::os::args;

mod board;
mod benchmarks;
mod cli;
mod engine;
mod game;
mod gtp;
mod playout;
mod ruleset;
mod sgf;

fn main() {

    let opts = [
        optopt("m", "mode", "set control mode", "MODE"),
        optopt("e", "engine", "select an engine", "ENGINE"),
        optopt("s", "size", "set the size of the board in the benchmarks", "SIZE"),
        optopt("r", "runtime", "set the run time of the benchmarks (in s)", "RUNTIME")
            ];

    let matches = match getopts(args().tail(), &opts) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    let engine_arg = matches.opt_str("e").map(|s| s.into_ascii_lower());
    let engine = match engine_arg {
        Some(ref s) if s.as_slice() == "mc" => box McEngine::new() as Box<Engine>,
        _                                   => box RandomEngine::new() as Box<Engine>
    };

    let mode_arg = matches.opt_str("m").map(|s| s.into_ascii_lower());
    match mode_arg {
        Some(ref s) if s.as_slice() == "gtp" => gtp::driver::Driver::new(engine),
        Some(ref s) if s.as_slice() == "pps" => {
            let size    = matches.opt_str("s").and_then(|s| s.as_slice().parse::<u8>()).unwrap_or(9);
            let runtime = matches.opt_str("r").and_then(|s| s.as_slice().parse::<uint>()).unwrap_or(30);
            benchmarks::pps(size, runtime)
        },
        _                                    => cli::Driver::new()
    };
}
