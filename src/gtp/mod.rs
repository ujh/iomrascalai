/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot, Urban Hafner                          *
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
#![allow(non_camel_case_types)]
use std::path::Path;
use board::Color;
use board::IllegalMove;
use board::Move;
use config::Config;
use engine::Engine;
use engine::EngineController;
use game::Game;
use ruleset::Ruleset;
use sgf::parser::Parser;
use timer::Timer;
use strenum::Strenum;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::thread;

pub mod driver;
mod test;

strenum! {
    KnownCommands =>
        boardsize,
        clear_board,
        final_score,
        genmove,
        known_command,
        komi,
        list_commands,
        loadsgf,
        name,
        play,
        protocol_version,
        quit,
        showboard,
        time_left,
        time_settings,
        version
}

pub enum Command {
    BoardSize,
    ClearBoard,
    Empty,
    Error,
    ErrorMessage(String),
    FinalScore(String),
    GenMove(String),
    GenMoveError(Move, IllegalMove),
    KnownCommand(bool),
    Komi,
    ListCommands(String),
    LoadSgf,
    Name,
    Play,
    PlayError(Move, IllegalMove),
    ProtocolVersion,
    Quit,
    ShowBoard(String),
    TimeLeft,
    TimeSettings,
    Version,
}

pub struct GTPInterpreter<'a> {
    config: Config,
    game: Game,
    _guard: thread::JoinGuard<'a, ()>,
    receive_move_from_controller: Receiver<Move>,
    send_game_to_controller: Sender<(Game, Color, Timer)>,
    send_halt_to_controller: Sender<()>,
    timer: Timer,
}

impl<'a> GTPInterpreter<'a> {
    pub fn new(config: Config, engine: Box<Engine>) -> GTPInterpreter<'a> {
        let komi      = 6.5;
        let boardsize = 19;
        let (send_game_to_controller, receive_game_from_interpreter) = channel::<(Game, Color, Timer)>();
        let (send_move_to_interpreter, receive_move_from_controller) = channel::<Move>();
        let (send_halt_to_controller, receive_halt_from_interpreter) = channel::<()>();
        let controller_config = config;
        let guard = thread::scoped(move || {
            let controller = EngineController::new(controller_config, engine);
            loop {
                select!(data = receive_game_from_interpreter.recv() => {
                    let (game, color, timer) = data.unwrap();
                    controller.run_and_return_move(color, &game, &timer, send_move_to_interpreter.clone());
                },
                        _ = receive_halt_from_interpreter.recv() => {
                            break;
                        })
            }
        });
        GTPInterpreter {
            config: config,
            game: Game::new(boardsize, komi, config.ruleset),
            _guard: guard,
            receive_move_from_controller: receive_move_from_controller,
            send_game_to_controller: send_game_to_controller,
            send_halt_to_controller: send_halt_to_controller,
            timer: Timer::new(config),
        }
    }

    pub fn quit(&self) {
        self.send_halt_to_controller.send(()).unwrap();
    }

    pub fn game<'b>(&'b self) -> &'b Game {
        &self.game
    }

    pub fn komi(&self) -> f32 {
        self.game.komi()
    }

    pub fn ruleset(&self) -> Ruleset {
        self.config.ruleset
    }

    pub fn main_time(&self) -> u32 {
        self.timer.main_time()
    }

    pub fn byo_time(&self) -> u32 {
        self.timer.byo_time()
    }

    pub fn byo_stones(&self) -> i32 {
        self.timer.byo_stones()
    }

    pub fn boardsize(&self) -> u8 {
        self.game.size()
    }

    pub fn read(&mut self, input: &str) -> Command {
        let preprocessed = self.preprocess(input);
        if preprocessed.len() == 0 { return Command::Empty };

        let command: Vec<&str> = preprocessed.split(' ').collect();

        //command[0] is never empty because a split always has at least one part
        let command_name = match <KnownCommands>::enumify(command[0]) {
        	Some(comm)     => comm,
        	None           => return Command::Error
    	};

        match command_name {
            KnownCommands::name             => Command::Name,
            KnownCommands::version          => Command::Version,
            KnownCommands::protocol_version => Command::ProtocolVersion,
            KnownCommands::list_commands    => Command::ListCommands(<KnownCommands>::stringify()),
            KnownCommands::known_command    => match command.get(1) {
            	Some(comm) => Command::KnownCommand(<KnownCommands>::enumify(&comm).is_some()),
            	None => Command::KnownCommand(false)
        	},
            KnownCommands::boardsize        => match command.get(1) {
            	Some(comm) => match comm.parse::<u8>() {
                    Ok(size) => {
                        self.game = Game::new(size, self.komi(), self.ruleset());
                        Command::BoardSize
                    },
                    Err(_) => Command::Error
                },
            	None => Command::Error
        	},
            KnownCommands::clear_board      => {
                self.game = Game::new(self.boardsize(), self.komi(), self.ruleset());
                self.timer.reset();
                Command::ClearBoard
            },
            KnownCommands::komi             => match command.get(1) {
                Some(comm) =>
                    match comm.parse::<f32>() {
                        Ok(komi) => {
                            self.game.set_komi(komi);
                            Command::Komi
                        },
                        Err(_) => Command::Error
                    },
                None => Command::Error
            },
            KnownCommands::genmove          => match command.get(1) {
        	Some(comm) => {
                    self.timer.start();
        	    let color = Color::from_gtp(comm);
                    self.send_game_to_controller.send((self.game.clone(), color, self.timer.clone())).unwrap();
                    let m = self.receive_move_from_controller.recv().unwrap();
                    match self.game.play(m) {
                        Ok(g) => {
                            self.game = g;
                            self.timer.stop();
                            Command::GenMove(m.to_gtp())
                        },
                        Err(e) => {
                            Command::GenMoveError(m, e)
                        }
                    }
                },
                None => Command::Error
    		},
            KnownCommands::play             => match command.get(2) {
            	Some(second) => {
                    let m = Move::from_gtp(command[1], second); //command[1] should be there
                    match self.game.play(m) {
                        Ok(g) => {
                            self.game = g;
                            Command::Play
                        },
                        Err(e) => {
                            Command::PlayError(m, e)
                        }
                    }
                },
            	None => Command::Error
        	},
            KnownCommands::showboard        => Command::ShowBoard(format!("\n{}", self.game)),
            KnownCommands::quit             => {
                self.quit();
                Command::Quit
            },
            KnownCommands::final_score      => Command::FinalScore(format!("{}", self.game.score())),
            KnownCommands::time_settings    => match command.get(3) {
            	Some(third) => {
            		//command[1] and command[2] should be there
                    match (command[1].parse::<u32>(), command[2].parse::<u32>(), third.parse::<i32>()) {
                        (Ok(main), Ok(byo), Ok(stones)) => {
                            self.timer.setup(main, byo, stones);
                            Command::TimeSettings
                        }
                        _ => Command::Error
                    }
                },
            	None => Command::Error
        	},
            KnownCommands::time_left        => match command.get(3) {
            	Some(third) => {
            		//command[2] should be there
            		//TODO: seems wrong, missing a color
                    match (command[2].parse::<u32>(), third.parse::<i32>()) {
                        (Ok(time), Ok(stones)) => {
                            self.timer.update(time, stones);
                            Command::TimeLeft
                        },
                        _ => Command::Error
                    }
                },
            	None => Command::Error
        	},
            KnownCommands::loadsgf          => match command.get(1) {
            	Some(comm) => {
                    let filename = comm;

                    match Parser::from_path(Path::new(filename)) {
                    	Ok(parser) => {
                    		let game = parser.game();
                            match game {
                                Ok(g) => {
                                    self.game = g;
                                    Command::LoadSgf
                                },
                                Err(_) => Command::ErrorMessage(String::from_str("cannot load file"))
                            }
                		},
                    	Err(_) => Command::ErrorMessage(String::from_str("cannot load file"))
                	}
                },
            	None => Command::Error
        	}
        }
    }

    fn preprocess(&self, input: &str) -> String {
        // Convert tab to space
        let horizontal_tab = regex!(r"\t");
        let without_tabs = horizontal_tab.replace_all(input, " ");
        // Remove all control characters
        let cntrls = regex!(r"[:cntrl:]");
        let without_ctrls = cntrls.replace_all(without_tabs.as_ref(), "");
        // Then we remove anything after a #
        let comment = regex!(r"#.*");
        let without_comment = comment.replace(without_ctrls.as_ref(), "");
        // We remove the whitespaces before/after the string
        without_comment.trim().to_string()
    }
}
