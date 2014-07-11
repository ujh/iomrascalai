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

use engine::Engine;
use engine::random_engine::RandomEngine;
use game::Game;
use ruleset::KgsChinese;
use std::io::stdio::stdin;
use super::BoardSize;
use super::ClearBoard;
use super::FinalScore;
use super::GTPInterpreter;
use super::GenMove;
use super::GenMoveError;
use super::KnownCommand;
use super::Komi;
use super::ListCommands;
use super::Name;
use super::Play;
use super::PlayError;
use super::ProtocolVersion;
use super::Quit;
use super::ShowBoard;
use super::Version;

pub struct Driver;

impl Driver {
    pub fn new() {
        let engine = RandomEngine::new();
        let engine_name = "Iomrascálaí";
        let engine_version = "0.1.0";
        let protocol_version = "2";

        let mut interpreter = GTPInterpreter::new(engine);
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
                Name               => print!("= {}\n\n", engine_name),
                Version            => print!("= {}\n\n", engine_version),
                ProtocolVersion    => print!("= {}\n\n", protocol_version),
                ListCommands(s)    => print!("= {}\n\n", s),
                KnownCommand(b)    => print!("= {}\n\n", b),
                BoardSize          => print!("= \n\n"),
                ClearBoard         => print!("= \n\n"),
                Komi               => print!("= \n\n"),
                Play               => print!("= \n\n"),
                PlayError(move)    => print!("? Illegal move: {}\n\n", move),
                GenMove(s)         => print!("= {}\n\n", s),
                GenMoveError(move) => print!("? Illegal move: {}\n\n", move),
                ShowBoard(s)       => print!("= {}\n\n", s),
                Quit            => {print!("= \n\n"); return;},
                FinalScore(s)   => {print!("= {}\n\n", s)},
                _               => {print!("? unknown command\n\n");}
            }
        }

    }
}
