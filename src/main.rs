/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Urban Hafner, Thomas Poinsot, Igor Polyakov, Ben Fu   *
 * Copyright 2016 Urban Hafner                                          *
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
#![feature(mpsc_select)]
#![feature(plugin)]
#![feature(test)]
#![plugin(regex_macros)]
#![cfg_attr(test, plugin(stainless))]

#[cfg(test)]
extern crate hamcrest;

extern crate core;
#[macro_use] extern crate enum_primitive;
extern crate getopts;
extern crate num;
extern crate num_cpus;
extern crate quicksort;
extern crate rand;
extern crate regex;
#[no_link] extern crate regex_macros;
extern crate smallvec;
extern crate test;
extern crate time;
extern crate toml;

// Use everything in config publicly to force the generation of
// documentation.
pub use config::*;
use engine::Engine;
use gtp::driver::Driver;
use patterns::Matcher;
use ruleset::Ruleset;

use getopts::Options;
use std::sync::Arc;
use std::env::args;
use std::process::exit;

mod board;
mod config;
mod engine;
mod fixtures;
mod game;
mod gtp;
mod ownership;
mod patterns;
mod playout;
mod ruleset;
mod score;
mod sgf;
mod timer;
mod version;

fn main() {
    let mut opts = Options::new();
    let default_ruleset = Ruleset::KgsChinese;
    opts.optflag("d", "dump", "Dump default config to stdout");
    opts.optflag("g", "gfx", "Ouput GoGui live graphics");
    opts.optflag("h", "help", "Print this help menu");
    opts.optflag("l", "log", "Print logging information to STDERR");
    opts.optflag("v", "version", "Print the version number");
    opts.optopt("c", "config", "Config file", "FILE");
    opts.optopt(
        "t",
        "threads",
        "Number of worker threads (overrides value set in the config file)",
        "INTEGER"
    );
    let r_expl = format!("cgos|chinese|tromp-taylor (defaults to {})", default_ruleset);
    opts.optopt("r", "rules", "Pick ruleset", &r_expl);
    let args : Vec<String> = args().collect();

    let (_, tail) = args.split_first().unwrap();
    let matches = match opts.parse(tail) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            exit(1);
        }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", args[0]);
        println!("{}", opts.usage(brief.as_ref()));
        exit(0);
    }
    if matches.opt_present("v") {
        println!("Iomrascálaí {}", version::version());
        exit(0);
    }
    if matches.opt_present("d") {
        println!("{}", Config::toml());
        exit(0);
    }
    let log = matches.opt_present("l");
    let gfx = matches.opt_present("g");
    let ruleset = match matches.opt_str("r") {
        Some(r) => match r.parse() {
            Ok(ruleset) => ruleset,
            Err(error) => {
                println!("{}", error);
                exit(1);
            }
        },
        None => default_ruleset
    };
    let threads = match matches.opt_str("t") {
        Some(ts) => match ts.parse() {
            Ok(threads) => Some(threads),
            Err(error) => {
                println!("{}", error);
                exit(1);
            }
        },
        None => None
    };

    let config_file_opt = matches.opt_str("c");
    let config = match config_file_opt {
        Some(filename) => {
            Config::from_file(filename, log, gfx, ruleset, threads)
        },
        None => {
            Config::default(log, gfx, ruleset, threads)
        }
    };

    let config = Arc::new(config);
    // Instantiate only one matcher as it does a lot of computation
    // during setup.
    let matcher = Arc::new(Matcher::new());

    let engine = Engine::new(config.clone(), matcher);

    config.log(format!("Current configuration: {:#?}", config));

    Driver::new(config, engine);
}
