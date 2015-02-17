/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Thomas Poinsot                                        *
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

use engine::Engine;
use ruleset::Ruleset;
use super::Command;
use super::GTPInterpreter;
use version;

use std::old_io::stdio::stdin;

pub struct Driver;

impl Driver {
    pub fn new(ruleset: Ruleset, engine: Box<Engine>) {
        let engine_name = "Iomrascálaí";
        let engine_version = version::version();
        let protocol_version = "2";

        let mut interpreter = GTPInterpreter::new(ruleset, engine);
        let mut reader = stdin();

        loop {
            // let result = interpreter.read(reader.read_line().unwrap().as_slice());

            // match result {
            //     Ok(s) => print!("= {}\n\n", s),
            //     Err(s) => print!("? {}\n\n", s),
            //     Quit   => {
            //         print!("= \n\n");
            //         return;
            //     }
            // }

            let command = interpreter.read(reader.read_line().unwrap().as_slice());

            match command {
                Command::Name               => print!("= {}\n\n", engine_name),
                Command::Version            => print!("= {}\n\n", engine_version),
                Command::ProtocolVersion    => print!("= {}\n\n", protocol_version),
                Command::ListCommands(s)    => print!("= {}\n\n", s),
                Command::KnownCommand(b)    => print!("= {}\n\n", b),
                Command::BoardSize          => print!("= \n\n"),
                Command::ClearBoard         => print!("= \n\n"),
                Command::Komi               => print!("= \n\n"),
                Command::Play               => print!("= \n\n"),
                Command::PlayError(m, e)    => print!("? Illegal move: {:?} ({:?})\n\n", m, e),
                Command::GenMove(s)         => print!("= {}\n\n", s),
                Command::GenMoveError(m, e) => print!("? Illegal move: {:?} ({:?})\n\n", m, e),
                Command::ShowBoard(s)       => print!("= {}\n\n", s),
                Command::Quit               => {print!("= \n\n"); return;},
                Command::FinalScore(s)      => {print!("= {}\n\n", s)},
                Command::TimeSettings       => {print!("= \n\n")},
                Command::TimeLeft           => {print!("= \n\n")},
                _                           => {print!("? unknown command\n\n");}
            }
        }

    }
}
