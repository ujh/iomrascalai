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
use super::GTPInterpreter;
use super::GenMove;
use super::KnownCommand;
use super::Komi;
use super::ListCommands;
use super::Name;
use super::Play;
use super::ProtocolVersion;
use super::Quit;
use super::ShowBoard;
use super::Version;

pub struct Driver;

impl Driver {
    pub fn new() {
        let engine = RandomEngine::new();
        let engine_name = "Iomrascálaí";
        let engine_version = "0.1";
        let protocol_version = "2";

        let interpreter = GTPInterpreter::new();
        let mut reader = stdin();

        let mut komi = 6.5;
        let mut board_size = 19;
        let mut game = Game::new(board_size, komi, KgsChinese);

        loop {
            let command = interpreter.read(reader.read_line().unwrap().as_slice());

            match command {
                Name            => print!("= {}\n\n", engine_name),
                Version         => print!("= {}\n\n", engine_version),
                ProtocolVersion => print!("= {}\n\n", protocol_version),
                ListCommands    => print!("= {}\n\n", interpreter.gen_list_known_commands()),
                KnownCommand(b) => print!("= {}\n\n", b),
                BoardSize(size) => {
                    board_size = size;
                    game = Game::new(board_size, komi, KgsChinese);
                    print!("= \n\n");
                },
                ClearBoard      => {
                    game = Game::new(board_size, komi, KgsChinese);
                    print!("= \n\n");
                },
                Komi(k)         => {
                    komi = k;
                    game.set_komi(k);
                    print!("= \n\n");
                },
                Play(move) => {
                    game = match game.play(move) {
                        Ok(g)  => {print!("= \n\n"); g},
                        Err(e) => {print!("? Illegal Move: {}\n\n", e); game}
                    }
                },
                GenMove(c)      => {
                    let generated_move = engine.gen_move(c, &game);
                    game = match game.play(generated_move) {
                        Ok(g)  => {print!("= {}\n\n", generated_move.to_gtp()); g},
                        Err(e) => {print!("? Illegal Move: {}\n\n", e); game}
                    }
                },
                ShowBoard       => {
                    print!("= \n");
                    println!("{}", game);
                    print!("\n\n");
                }
                Quit            => {print!("= \n\n"); return;},
                _               => {print!("? unknown command\n\n");}
            }
        }

    }
}
