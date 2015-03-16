/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
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
#![feature(collections)]
#![feature(core)]
#![feature(io)]
#![feature(old_io)]
#![feature(old_path)]
#![feature(plugin)]
#![feature(std_misc)]
#![feature(test)]
#![plugin(regex_macros)]
extern crate core;
extern crate getopts;
extern crate rand;
extern crate regex;
#[no_link] extern crate regex_macros;
extern crate "rustc-serialize" as rustc_serialize;
extern crate test;
extern crate time;
 #[macro_use(strenum)] extern crate strenum;

use engine::AmafMcEngine;
use engine::Engine;
use engine::SimpleMcEngine;
use engine::RandomEngine;
use gtp::driver::Driver;
use ruleset::KgsChinese;
use ruleset::Ruleset;
use version::version;

use getopts::Options;
use std::ascii::OwnedAsciiExt;
use std::env::args;
use std::os::num_cpus;

macro_rules! log(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::old_io::stdio::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

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

pub fn main() {
    let mut opts = Options::new();
    let args : Vec<String> = args().collect();
    opts.optopt("e", "engine", "select an engine (defaults to amaf)", "amaf|mc|random");
    opts.optopt("r", "ruleset", "select the ruleset (defaults to chinese)", "cgos|chinese|tromp-taylor|minimal");
    opts.optopt("t", "threads", format!("number of threads to use (defaults to {})", num_cpus()).as_slice(), "NUM");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print the version number");

    let matches = match opts.parse(args.tail()) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", args[0]);
        print!("{}", opts.usage(brief.as_slice()));
        return;
    }
    if matches.opt_present("v") {
        println!("Iomrascálaí {}", version::version());
        return;
    }
    let threads = match matches.opt_str("t") {
        Some(s) => {
            match s.parse::<usize>() {
                Ok(n)  => n,
                Err(_) => num_cpus()
            }
        },
        None    => num_cpus()
    };
    let engine_arg = matches.opt_str("e").map(|s| s.into_ascii_lowercase());
    let engine: Box<Engine> = match engine_arg {
        Some(s) => {
            match s.as_slice() {
                "random" => Box::new(RandomEngine::new()),
                "mc"     => Box::new(SimpleMcEngine::new(threads)),
                _        => Box::new(AmafMcEngine::new(threads)),
            }
        },
        None => Box::new(AmafMcEngine::new(threads))
    };
    let rules_arg = matches.opt_str("r").map(|s| s.into_ascii_lowercase());
    let ruleset = match rules_arg {
        Some(r) => Ruleset::from_string(r),
        None    => KgsChinese
    };
    Driver::new(ruleset, engine);
}
