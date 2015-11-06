/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Urban Hafner, Thomas Poinsot, Igor Polyakov, Ben Fu   *
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
#![feature(core)]
#![feature(mpsc_select)]
#![feature(plugin)]
#![feature(test)]
#![feature(vec_push_all)]
#![plugin(regex_macros)]
#![plugin(stainless)]

#[cfg(test)]
extern crate hamcrest;

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
extern crate thread_scoped;
extern crate time;
extern crate toml;
#[macro_use(strenum)] extern crate strenum;

use config::Config;
use gtp::driver::Driver;
use patterns::Matcher;

use getopts::Options;
use std::sync::Arc;
use std::env::args;
use std::process::exit;

mod board;
mod config;
mod engine;
mod game;
mod gtp;
mod patterns;
mod playout;
mod ruleset;
mod score;
mod sgf;
mod timer;
mod version;

pub fn main() {
    let mut opts = Options::new();
    opts.optflag("d", "dump", "Dump default config to stdout");
    opts.optflag("h", "help", "Print this help menu");
    opts.optflag("v", "version", "Print the version number");
    opts.optopt("c", "config", "Config file", "FILE");
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
    let config_file_opt = matches.opt_str("c");
    let config = match config_file_opt {
        Some(filename) => {
            Config::from_file(filename)
        },
        None => {
            Config::default()
        }
    };

    match config.check() {
        Ok(_) => {},
        Err(s) => {
            println!("{}", s);
            exit(1);
        }
    }

    let config = Arc::new(config);
    // Instantiate only one matcher as it does a lot of computation
    // during setup.
    let matcher = Arc::new(Matcher::new());

    let engine = engine::factory(config.clone(), matcher);

    config.log(format!("Current configuration: {:#?}", config));

    Driver::new(config, engine);
}
