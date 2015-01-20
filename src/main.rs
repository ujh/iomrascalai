/************************************************************************
 *                                                                      *
 * Copyright 2014-2015 Urban Hafner, Thomas Poinsot                     *
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
#![feature(plugin)]
extern crate core;
extern crate getopts;
extern crate rand;
extern crate regex;
#[plugin]
extern crate regex_macros;
extern crate "rustc-serialize" as rustc_serialize;
extern crate test;

use engine::Engine;
use engine::McEngine;
use engine::RandomEngine;
use getopts::getopts;
use getopts::optflag;
use getopts::optopt;
use getopts::usage;
use std::ascii::OwnedAsciiExt;
use std::os::args;

mod board;
mod cli;
mod engine;
mod game;
mod gtp;
mod playout;
mod ruleset;
mod score;
mod sgf;
mod version;

fn main() {

    let opts = [
        optopt("m", "mode", "set control mode (defaults to cli)", "cli|gtp"),
        optopt("e", "engine", "select an engine (defaults to random)", "mc|random"),
        optflag("h", "help", "print this help menu"),
        ];

    let matches = match getopts(args().tail(), &opts) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        let program = args()[0].clone();
        let brief = format!("Usage: {} [options]", program);
        print!("{}", usage(brief.as_slice(), &opts));
        return;
    }

    let engine_arg = matches.opt_str("e").map(|s| s.into_ascii_lowercase());
    let engine = match engine_arg {
        Some(ref s) if s.as_slice() == "mc" => Box::new(McEngine::new()) as Box<Engine>,
        _                                   => Box::new(RandomEngine::new()) as Box<Engine>
    };

    let mode_arg = matches.opt_str("m").map(|s| s.into_ascii_lowercase());
    match mode_arg {
        Some(ref s) if s.as_slice() == "gtp" => gtp::driver::Driver::new(engine),
        _                                    => cli::Driver::new()
    };
}
