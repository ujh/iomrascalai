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

use std::io::stdin;
use std::sync::Arc;

pub struct Driver;

impl Driver {
    pub fn new(config: Arc<Config>, engine: Engine) {
        let mut interpreter = GTPInterpreter::new(config, engine);
        let reader = stdin();
        let mut command = String::new();
        let regex = regex!(r"^quit");
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
