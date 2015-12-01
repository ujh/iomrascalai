/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
 * Copyright 2015 Thomas Poinsot, Igor Polyakov                         *
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
use super::Command;
use super::GTPInterpreter;
use version;

use std::io::stdin;
use std::sync::Arc;

pub struct Driver;

impl Driver {
    pub fn new(config: Arc<Config>, engine: Box<Engine>) {
        let engine_name = "Iomrascalai";
        let engine_version = version::version();
        let protocol_version = "2";

        let mut interpreter = GTPInterpreter::new(config, engine);
        let reader = stdin();
        let mut command = String::new();

        loop {
            command.clear();
            reader.read_line(&mut command).unwrap();

            let gtp_command = interpreter.read(&*command);

            match gtp_command {
                Command::BoardSize               => print!("= \n\n"),
                Command::ClearBoard              => print!("= \n\n"),
                Command::Empty                   => print!("? empty command\n\n"),
                Command::Error                   => print!("? unknown command\n\n"),
                Command::ErrorMessage(e)         => print!("? {}\n\n", e),
                Command::FinalScore(s)           => print!("= {}\n\n", s),
                Command::GenMove(s)              => print!("= {}\n\n", s),
                Command::GenMoveError(m, e)      => print!("? Illegal move: {:?} ({:?})\n\n", m, e),
                Command::GoGuiAnalyzeCommands(s) => print!("= {}\n\n", s),
                Command::KnownCommand(b)         => print!("= {}\n\n", b),
                Command::Komi                    => print!("= \n\n"),
                Command::ListCommands(s)         => print!("= {}\n\n", s),
                Command::LoadSgf                 => print!("= \n\n"),
                Command::Name                    => print!("= {}\n\n", engine_name),
                Command::Play                    => print!("= \n\n"),
                Command::PlayError(m, e)         => print!("? Illegal move: {:?} ({:?})\n\n", m, e),
                Command::ProtocolVersion         => print!("= {}\n\n", protocol_version),
                Command::Quit                    => { print!("= \n\n"); return; },
                Command::ShowBoard(s)            => print!("= {}\n\n", s),
                Command::TimeLeft                => print!("= \n\n"),
                Command::TimeSettings            => print!("= \n\n"),
                Command::Version                 => print!("= {}\n\n", engine_version),
            }
        }

    }
}
