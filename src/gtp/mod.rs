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

use board::Color;
use board::Coord;
use board::Empty;
use board::Move;
use config::Config;
use engine::Engine;
use engine::EngineController;
use game::Game;
use ruleset::Ruleset;
use sgf::parser::Parser;
use timer::Timer;
use version;

use std::path::Path;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use thread_scoped::JoinGuard;
use thread_scoped::scoped;
use time::precise_time_ns;

pub mod driver;
mod test;

pub enum ControllerCommand {
    GenMove(Game, Color, Timer),
    Ownership,
    Reset(u8),
    ShutDown,
}

pub enum ControllerResponse {
    GenMove(Move),
    Ownership(String)
}

pub struct GTPInterpreter<'a> {
    _guard: JoinGuard<'a, ()>,
    commands: Vec<&'a str>,
    config: Arc<Config>,
    game: Game,
    receive_response_from_controller: Receiver<ControllerResponse>,
    running: bool,
    send_command_to_controller: Sender<ControllerCommand>,
    timer: Timer,
}

impl<'a> GTPInterpreter<'a> {
    pub fn new(config: Arc<Config>, engine: Box<Engine>) -> GTPInterpreter<'a> {
        let komi      = 6.5;
        let boardsize = 19;
        let (send_command_to_controller, receive_command_from_interpreter) = channel::<ControllerCommand>();
        let (send_response_to_interpreter, receive_response_from_controller) = channel::<ControllerResponse>();
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
                                    let (m, playouts) = controller.run_and_return_move(color, &game, &timer);
                                    send_response_to_interpreter.send(ControllerResponse::GenMove(m)).expect("Failed to send response to genmove to interpreter");
                                    Self::measure_playout_speed(started_at, playouts, &genmove_config);
                                },
                                ControllerCommand::Ownership => {
                                    let stats = controller.ownership_statistics();
                                    let response = ControllerResponse::Ownership(stats);
                                    send_response_to_interpreter.send(response).expect("Failed to send respnse to imrscl-ownership to interpreter");
                                },
                                ControllerCommand::Reset(boardsize) => {
                                    controller.reset(boardsize);
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
                commands: vec![
                    "boardsize",
                    "clear_board",
                    "final_score",
                    "final_status_list",
                    "genmove",
                    "gogui-analyze_commands",
                    "imrscl-ownership",
                    "known_command",
                    "komi",
                    "list_commands",
                    "loadsgf",
                    "name",
                    "play",
                    "protocol_version",
                    "quit",
                    "showboard",
                    "time_left",
                    "time_settings",
                    "version",
                    ],
                config: config.clone(),
                game: Game::new(boardsize, komi, config.ruleset),
                receive_response_from_controller: receive_response_from_controller,
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
        if !self.commands.contains(&command[0]) {
            return Err("unknown command".to_string());
        }
        let arguments = &command[1..];
        match command[0] {
            "boardsize" => self.execute_boardsize(arguments),
            "clear_board" => self.execute_clear_board(arguments),
            "final_score" => self.execute_final_score(arguments),
            "final_status_list" => self.execute_final_status_list(arguments),
            "genmove" => self.execute_genmove(arguments),
            "gogui-analyze_commands" => self.execute_gogui_analyze_commands(arguments),
            "imrscl-ownership" => self.execute_imrscl_ownership(arguments),
            "known_command" => self.execute_known_command(arguments),
            "komi" => self.execute_komi(arguments),
            "list_commands" => self.execute_list_commands(arguments),
            "loadsgf" => self.execute_loadsgf(arguments),
            "name" => self.execute_name(arguments),
            "play" => self.execute_play(arguments),
            "protocol_version" => self.execute_protocol_version(arguments),
            "quit" => self.execute_quit(arguments),
            "showboard" => self.execute_showboard(arguments),
            "time_left" => self.execute_time_left(arguments),
            "time_settings" => self.execute_time_settings(arguments),
            "version" => self.execute_version(arguments),
            _ => Err("unknown command".to_string())
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
        Ok(self.commands[1..].iter().fold(self.commands[0].to_string(), |acc, &el| format!("{}\n{}", acc, el)))
    }

    fn execute_known_command(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(0) {
            Some(comm) => Ok(format!("{}", self.commands.contains(comm))),
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
        self.send_command_to_controller.send(ControllerCommand::Reset(self.boardsize())).unwrap();
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
                let response = self.receive_response_from_controller.recv().unwrap();
                match response {
                    ControllerResponse::GenMove(m) => {
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
                    _ => Err("received wrong response from controller for genmove".to_string())
                }
            },
            None => Err("missing argument".to_string())
    	}
    }

    fn execute_imrscl_ownership(&mut self, _: &[&str]) -> Result<String, String> {
        let command = ControllerCommand::Ownership;
        self.send_command_to_controller.send(command).unwrap();
        let response = self.receive_response_from_controller.recv().unwrap();
        match response {
            ControllerResponse::Ownership(s) => Ok(s),
            _ => Err("Received from response from controller for imsrcl-ownership command".to_string())
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

    fn execute_final_status_list(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(0) {
            Some(kind) => {
                match *kind {
                    "alive" => {
                        let board = self.game.board();
                        let coords: Vec<Coord> = Coord::for_board_size(board.size()).iter()
                            .filter(|c| board.color(c) != Empty)
                            .cloned()
                            .collect();
                        let s = coords[1..].iter()
                            .fold(coords[0].to_gtp(), |acc, el| format!("{} {}", acc, el.to_gtp()));
                        Ok(s)
                    },
                    "dead" => Ok("".to_string()),
                    "seki" => Ok("".to_string()),
                    _ => Err("unknown argument".to_string()),
                }
            },
            None => Err("missing argument".to_string())
        }
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

    fn execute_gogui_analyze_commands(&mut self, _: &[&str]) -> Result<String, String> {
        let analyze_commands = vec![
            "dboard/Ownership/imrscl-ownership"
                ];
        Ok(analyze_commands[1..].iter().fold(analyze_commands[0].to_string(), |acc, &el| format!("{}\n{}", acc, el)))
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
