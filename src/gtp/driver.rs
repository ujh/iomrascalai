/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Thomas Poinsot, Igor Polyakov                         *
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

use config::Config;
use engine::Engine;
use super::GTPInterpreter;

use regex::Regex;
use std::io::stdin;
use std::sync::Arc;

pub struct Driver;

impl Driver {
    pub fn new(config: Arc<Config>, engine: Engine) {
        let mut interpreter = GTPInterpreter::new(config, engine);
        let reader = stdin();
        let mut command = String::new();
        let regex = Regex::new(r"^quit").unwrap();
        loop {
            command.clear();
            reader.read_line(&mut command).unwrap();
            if command.is_empty() {
                return; // EOF or Ctrl-D
            }
            let response = interpreter.read(&*command);

            match response {
                Ok(s)  => print!("= {}\n\n", s),
                Err(s) => print!("? {}\n\n", s)
            }
            if regex.is_match(&command) {
                return;
            }
        }

    }
}

pub struct BenchmarkDriver;

impl BenchmarkDriver {
    pub fn new(config: Arc<Config>, engine: Engine, board_size: usize) {
        let mut interpreter = GTPInterpreter::new(config, engine);
        let time_settings = match board_size {
            9 => 300,
            13 => 600,
            15 => 1020,
            17 => 1440,
            19 => 1800,
            _ => unreachable!("board size not supported"),
        };
        interpreter.read(&format!("boardsize {}\n", board_size)).unwrap();
        interpreter.read("clear_board\n").unwrap();
        interpreter.read(&format!("time_settings {} 0 0 \n", time_settings)).unwrap();
        interpreter.read("genmove b\n").unwrap();
    }
}
