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
use board::Move;
use config::Config;
use engine::Engine;
use engine::EngineController;
use game::Game;
use ruleset::Ruleset;
use sgf::parser::Parser;
use timer::Timer;
use strenum::Strenum;
use version;

use num::traits::FromPrimitive;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use thread_scoped::JoinGuard;
use thread_scoped::scoped;
use time::precise_time_ns;

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

pub enum ControllerCommand {
    GenMove(Game, Color, Timer),
    Reset,
    ShutDown,
}

pub struct GTPInterpreter<'a> {
    _guard: JoinGuard<'a, ()>,
    config: Arc<Config>,
    game: Game,
    receive_move_from_controller: Receiver<Move>,
    running: bool,
    send_command_to_controller: Sender<ControllerCommand>,
    timer: Timer,
}

impl<'a> GTPInterpreter<'a> {
    pub fn new(config: Arc<Config>, engine: Box<Engine>) -> GTPInterpreter<'a> {
        let komi      = 6.5;
        let boardsize = 19;
        let (send_command_to_controller, receive_command_from_interpreter) = channel::<ControllerCommand>();
        let (send_move_to_interpreter, receive_move_from_controller) = channel::<Move>();
        let controller_config = config.clone();
        let genmove_config = config.clone();
        unsafe {
            let guard = scoped(move || {
                let mut controller = EngineController::new(controller_config, engine);
                loop {
                    match receive_command_from_interpreter.recv() {
                        Ok(command) => {
                            match command {
                                ControllerCommand::GenMove(game, color, timer) => {
                                    let started_at = precise_time_ns();
                                    let playouts = controller.run_and_return_move(color, &game, &timer, send_move_to_interpreter.clone());
                                    Self::measure_playout_speed(started_at, playouts, &genmove_config);
                                },
                                ControllerCommand::Reset => {
                                    controller.reset();
                                }
                                ControllerCommand::ShutDown => { break; },
                            }
                        },
                        Err(_) => { break; }
                    }
                }
            });
            GTPInterpreter {
                _guard: guard,
                config: config.clone(),
                game: Game::new(boardsize, komi, config.ruleset),
                receive_move_from_controller: receive_move_from_controller,
                running: true,
                send_command_to_controller: send_command_to_controller,
                timer: Timer::new(config),
            }
        }
    }

    pub fn quit(&mut self) {
        if self.running {
            self.send_command_to_controller.send(ControllerCommand::ShutDown).unwrap();
        }
        self.running = false;
    }

    pub fn komi(&self) -> f32 {
        self.game.komi()
    }

    pub fn ruleset(&self) -> Ruleset {
        self.config.ruleset
    }

    pub fn boardsize(&self) -> u8 {
        self.game.size()
    }

    pub fn read(&mut self, input: &str) -> Result<String, String> {
        let preprocessed = self.preprocess(input);
        if preprocessed.len() == 0 {
            return Err("empty command".to_string())
        };
        let command: Vec<&str> = preprocessed.split(' ').collect();
        let command_name = match <KnownCommands>::enumify(command[0]) {
            Some(n) => n,
            None => return Err("unknown command".to_string())
    	};
        let arguments = &command[1..];
        match command_name {
            KnownCommands::boardsize => self.execute_boardsize(arguments),
            KnownCommands::clear_board => self.execute_clear_board(arguments),
            KnownCommands::final_score => self.execute_final_score(arguments),
            KnownCommands::genmove => self.execute_genmove(arguments),
            KnownCommands::known_command => self.execute_known_command(arguments),
            KnownCommands::komi => self.execute_komi(arguments),
            KnownCommands::list_commands => self.execute_list_commands(arguments),
            KnownCommands::loadsgf => self.execute_loadsgf(arguments),
            KnownCommands::name => self.execute_name(arguments),
            KnownCommands::play => self.execute_play(arguments),
            KnownCommands::protocol_version => self.execute_protocol_version(arguments),
            KnownCommands::quit => self.execute_quit(arguments),
            KnownCommands::showboard => self.execute_showboard(arguments),
            KnownCommands::time_left => self.execute_time_left(arguments),
            KnownCommands::time_settings => self.execute_time_settings(arguments),
            KnownCommands::version => self.execute_version(arguments),
        }
    }

    fn execute_name(&mut self, _: &[&str]) -> Result<String, String> {
        Ok("Iomrascalai".to_string())
    }

    fn execute_version(&mut self, _: &[&str]) -> Result<String, String> {
        Ok(version::version().to_string())
    }

    fn execute_protocol_version(&mut self, _: &[&str]) -> Result<String, String> {
        Ok("2".to_string())
    }

    fn execute_list_commands(&mut self, _: &[&str]) -> Result<String, String> {
        Ok(<KnownCommands>::stringify())
    }

    fn execute_known_command(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(0) {
            Some(comm) => Ok(format!("{}", <KnownCommands>::enumify(&comm).is_some())),
            None => Err("missing argument".to_string())
        }
    }

    fn execute_boardsize(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(0) {
            Some(comm) => match comm.parse::<u8>() {
                Ok(size) => {
                    self.game = Game::new(size, self.komi(), self.ruleset());
                    Ok("".to_string())
                },
                Err(e) => Err(format!("{:?}", e))
            },
            None => Err("missing argument".to_string())
        }
    }

    fn execute_clear_board(&mut self, _: &[&str]) -> Result<String, String> {
        self.game = Game::new(self.boardsize(), self.komi(), self.ruleset());
        self.timer.reset();
        self.send_command_to_controller.send(ControllerCommand::Reset).unwrap();
        Ok("".to_string())
    }

    fn execute_komi(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(0) {
            Some(comm) =>
                match comm.parse::<f32>() {
                    Ok(komi) => {
                        self.game.set_komi(komi);
                        Ok("".to_string())
                    },
                    Err(e) => Err(format!("{:?}", e))
                },
            None => Err("missing argument".to_string())
        }
    }

    fn execute_genmove(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(0) {
            Some(comm) => {
                self.timer.start();
        	let color = Color::from_gtp(comm);
                let command = ControllerCommand::GenMove(self.game.clone(), color, self.timer.clone());
                self.send_command_to_controller.send(command).unwrap();
                let m = self.receive_move_from_controller.recv().unwrap();
                match self.game.play(m) {
                    Ok(g) => {
                        self.game = g;
                        self.timer.stop();
                        Ok(m.to_gtp())
                    },
                    Err(e) => {
                        Err(format!("Illegal move {:?} ({:?})", m, e))
                    }
                }
            },
            None => Err("missing argument".to_string())
    	}
    }

    fn execute_play(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(1) {
            Some(second) => {
                let m = Move::from_gtp(arguments[0], second);
                match self.game.play(m) {
                    Ok(g) => {
                        self.game = g;
                        Ok("".to_string())
                    },
                    Err(e) => Err(format!("Illegal move {:?} ({:?})", m, e))
                }
            },
            None => Err("missing argument".to_string())
        }
    }

    fn execute_showboard(&mut self, _: &[&str]) -> Result<String, String> {
        Ok(format!("\n{}", self.game))
    }

    fn execute_quit(&mut self, _: &[&str]) -> Result<String, String> {
        self.quit();
        Ok("".to_string())
    }

    fn execute_final_score(&mut self, _: &[&str]) -> Result<String, String> {
        Ok(format!("{}", self.game.score()))
    }

    fn execute_time_settings(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(2) {
            Some(third) => {
            	//command[1] and command[2] should be there
                match (arguments[0].parse::<u32>(), arguments[1].parse::<u32>(), third.parse::<i32>()) {
                    (Ok(main), Ok(byo), Ok(stones)) => {
                        self.timer.setup(main, byo, stones);
                        Ok("".to_string())
                    }
                    _ => Err("error parsing time_settings".to_string())
                }
            },
            None => Err("missing argument(s)".to_string())
        }
    }

    fn execute_time_left(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(2) {
            Some(third) => {
                match (arguments[1].parse::<u32>(), third.parse::<i32>()) {
                    (Ok(time), Ok(stones)) => {
                        self.timer.update(time, stones);
                        Ok("".to_string())
                    },
                    _ => Err("error parsing time_left".to_string())
                }
            },
            None => Err("missing argument(s)".to_string())
        }
    }

    fn execute_loadsgf(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(0) {
            Some(comm) => {
                let filename = comm;

                match Parser::from_path(Path::new(filename)) {
                    Ok(parser) => {
                    	let game = parser.game();
                        match game {
                            Ok(g) => {
                                self.game = g;
                                Ok("".to_string())
                            },
                            Err(_) => Err("cannot load file".to_string())
                        }
                    },
                    Err(_) => Err("cannot load file".to_string())
                }
            },
            None => Err("missing argument".to_string())
        }
    }

    fn measure_playout_speed(started_at: u64, playouts: usize, config: &Arc<Config>) {
        let finished_at = precise_time_ns();
        let duration_ns = finished_at - started_at;
        let duration_s = (duration_ns as f64) / 1000000000.0;
        let pps = (playouts as f64) / duration_s;
        let threads = config.threads;
        let ptps = pps / (threads as f64);
        config.log(format!("{}pps ({}pps per thread)", pps.round() as usize, ptps.round() as usize));
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

impl<'a> Drop for GTPInterpreter<'a> {

    fn drop(&mut self) {
        self.quit();
    }
}
