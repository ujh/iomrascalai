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

use getopts::optopt;
use getopts::getopts;
use std::ascii::OwnedStrAsciiExt;
use std::os::args;

mod board;
mod cli;
mod engine;
mod game;
mod gtp;
mod playout;
mod ruleset;
mod sgf;

fn main() {

    let opts = [optopt("m", "mode", "set control mode", "MODE")];
    let matches = match getopts(args().tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_string()) }
    };
    if matches.opt_present("m") {
        let mode = matches.opt_str("m").unwrap().into_ascii_lower();
        match mode.as_slice() {
            "gtp" => gtp::driver::Driver::new(),
            _     => cli::Driver::new()
        }
    } else {
        cli::Driver::new()
    }
}
