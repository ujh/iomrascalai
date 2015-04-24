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
#![feature(plugin)]
#![feature(scoped)]
#![feature(std_misc)]
#![feature(test)]
#![plugin(regex_macros)]
extern crate core;
#[macro_use] extern crate enum_primitive;
extern crate getopts;
extern crate num;
extern crate quicksort;
extern crate rand;
extern crate regex;
#[no_link] extern crate regex_macros;
extern crate smallvec;
extern crate test;
extern crate time;
#[macro_use(strenum)] extern crate strenum;

use config::Config;
use gtp::driver::Driver;
use ruleset::KgsChinese;
use ruleset::Ruleset;
use version::version;

use getopts::Options;
use std::ascii::OwnedAsciiExt;
use std::env::args;
use std::io::Write;
use std::process::exit;

macro_rules! log(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

mod board;
mod config;
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
    let mut config = Config::default();
    let mut opts = Options::new();
    let args : Vec<String> = args().collect();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("l", "log", "log to stderr (defaults to false)");
    opts.optflag("v", "version", "print the version number");

    opts.optopt("", "empty-area-prior", format!("prior value for empty areas (defaults to {})", config.uct.priors.empty).as_ref(), "NUM");
    opts.optopt("", "reuse-subtree", "reuse the subtree from the previous search (defaults to true)", "true|false");
    opts.optopt("", "use-atari-check-in-playouts", format!("Check for atari in the playouts (defaults to {}", config.playout.ladder_check).as_ref(), "true|false");
    opts.optopt("", "use-empty-area-prior", format!("use a prior for empty areas on the board (defaults to {:?})", config.uct.priors.use_empty).as_ref(), "true|false");
    opts.optopt("", "use-ladder-check-in-playouts", format!("Check for ladders in the playouts (defaults to {}", config.playout.ladder_check).as_ref(), "true|false");
    opts.optopt("P", "policies", "choose which policy to use (defaults to tuned)", "tuned|ucb1");
    opts.optopt("e", "engine", "select an engine (defaults to uct)", "amaf|mc|random|uct");
    opts.optopt("p", "playout", "type of playout to use (defaults to no-self-atari)", "light|no-self-atari");
    opts.optopt("r", "ruleset", "select the ruleset (defaults to chinese)", "cgos|chinese|tromp-taylor|minimal");
    opts.optopt("t", "threads", "number of threads to use (defaults to 1)", "NUM");

    let matches = match opts.parse(args.tail()) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", f.to_string());
            exit(1);
        }
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options]", args[0]);
        print!("{}", opts.usage(brief.as_ref()));
        return;
    }
    if matches.opt_present("v") {
        println!("Iomrascálaí {}", version::version());
        return;
    }
    if matches.opt_present("empty-area-prior") {
        let arg = matches.opt_str("empty-area-prior").unwrap();
        config.uct.priors.empty = match arg.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Unknown value ({}) as argument to --empty-area-prior", arg);
                exit(1);
            }
        }
    }
    
    if matches.opt_present("use-atari-check-in-playouts") {
        let arg = matches.opt_str("use-atari-check-in-playouts").map(|s| s.into_ascii_lowercase()).unwrap();
        config.playout.atari_check = match arg.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Unknown value ({}) as argument to --use-atari-check-in-playouts", arg);
                exit(1);
            }
        }
    }

    if matches.opt_present("use-empty-area-prior") {
        let arg = matches.opt_str("use-empty-area-prior").map(|s| s.into_ascii_lowercase()).unwrap();
        config.uct.priors.use_empty = match arg.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Unknown value ({}) as argument to --use-empty-area-prior", arg);
                exit(1);
            }
        }
    }
    if matches.opt_present("use-ladder-check-in-playouts") {
        let arg = matches.opt_str("use-ladder-check-in-playouts").map(|s| s.into_ascii_lowercase()).unwrap();
        config.playout.ladder_check = match arg.parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Unknown value ({}) as argument to --use-ladder-check-in-playouts", arg);
                exit(1);
            }
        }
    }

    let reuse_subtree_arg = matches.opt_str("reuse-subtree").map(|s| s.into_ascii_lowercase());
    let reuse_subtree = match reuse_subtree_arg {
        Some(arg) => {
            match arg.parse() {
                Ok(v) => v,
                Err(_) => panic!("Unknown value ({}) as argument to --reuse-subtree", arg)
            }
        },
        None => true
    };
    let log = matches.opt_present("l");

    let threads = match matches.opt_str("t") {
        Some(s) => {
            match s.parse() {
                Ok(n)  => n,
                Err(_) => 1
            }
        },
        None => 1
    };
    let rules_arg = matches.opt_str("r").map(|s| s.into_ascii_lowercase());
    let ruleset = match rules_arg {
        Some(r) => Ruleset::from_string(r),
        None    => KgsChinese
    };

    let policy = matches.opt_str("P").map(|s| s.into_ascii_lowercase());
    config.log = log;
    config.ruleset = ruleset;
    config.threads = threads;
    config.uct.tuned = match policy {
        Some(str) => if str == "ucb1" { false } else { true},
        _ => true
    };
    config.uct.reuse_subtree = reuse_subtree;
    let playout = playout::factory(matches.opt_str("p"), config);
    let engine = engine::factory(matches.opt_str("e"), config, playout);

    log!("Current configuration: {:?}", config);

    Driver::new(config, engine);
}
