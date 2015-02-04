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
#[plugin] #[no_link] extern crate regex_macros;
extern crate "rustc-serialize" as rustc_serialize;
extern crate test;
extern crate time;

use engine::Engine;
use engine::McEngine;
use engine::RandomEngine;
use gtp::driver::Driver;
use ruleset::KgsChinese;
use ruleset::Ruleset;
use version::version;

use getopts::Options;
use std::ascii::OwnedAsciiExt;
use std::os::args;

mod board;
mod engine;
mod game;
mod gtp;
mod playout;
mod ruleset;
mod score;
mod sgf;
mod timer;
mod version;

fn main() {
    let mut opts = Options::new();
    opts.optopt("e", "engine", "select an engine (defaults to random)", "mc|random");
    opts.optopt("r", "ruleset", "select the ruleset (defaults to chinese)", "cgos|chinese|tromp-taylor|minimal");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print the version number");

    let matches = match opts.parse(args().tail()) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        let program = args()[0].clone();
        let brief = format!("Usage: {} [options]", program);
        print!("{}", opts.usage(brief.as_slice()));
        return;
    }
    if matches.opt_present("v") {
        println!("Iomrascálaí {}", version::version());
        return;
    }
    let engine_arg = matches.opt_str("e").map(|s| s.into_ascii_lowercase());
    let engine = match engine_arg {
        Some(ref s) if s.as_slice() == "mc" => Box::new(McEngine::new()) as Box<Engine>,
        _                                   => Box::new(RandomEngine::new()) as Box<Engine>
    };
    let rules_arg = matches.opt_str("r").map(|s| s.into_ascii_lowercase());
    let ruleset = match rules_arg {
        Some(r) => Ruleset::from_string(r),
        None    => KgsChinese
    };
    Driver::new(ruleset, engine);
}
