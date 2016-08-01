/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot, Urban Hafner                          *
 * Copyright 2015 Urban Hafner, Thomas Poinsot, Igor Polyakov           *
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

use board::Color;
use board::Move;
use config::Config;
use engine::Engine;
use engine::EngineController;
use game::Game;
use ruleset::Ruleset;
use sgf::parser::Parser;
use timer::Timer;
use version;

use regex::Regex;
use std::path::Path;
use std::sync::Arc;
use time::precise_time_ns;

pub mod driver;
mod test;

pub struct GTPInterpreter<'a> {
    byo_stones: i32,
    byo_time: i64,
    commands: Vec<&'a str>,
    config: Arc<Config>,
    controller: EngineController,
    game: Game,
    main_time: i64,
    running: bool,
    timer: Timer,
}

impl<'a> GTPInterpreter<'a> {
    pub fn new(config: Arc<Config>, engine: Engine) -> GTPInterpreter<'a> {
        let controller = EngineController::new(config.clone(), engine);
        let komi = 6.5;
        let boardsize = 19;
        let commands = vec![
            "boardsize",
            "clear_board",
            "final_score",
            "final_status_list",
            "genmove",
            "gogui-analyze_commands",
            "imrscl-donplayouts",
            "imrscl-ownership",
            "imrscl-uct_gfx",
            "kgs-genmove_cleanup",
            "known_command",
            "komi",
            "list_commands",
            "loadsgf",
            "name",
            "play",
            "protocol_version",
            "quit",
            "reg_genmove",
            "showboard",
            "time_left",
            "time_settings",
            "version",
            ];
        GTPInterpreter {
            byo_stones: 0,
            byo_time: 0,
            commands: commands,
            config: config.clone(),
            controller: controller,
            game: Game::new(boardsize, komi, config.ruleset),
            main_time: 5,
            running: true,
            timer: Timer::new(config),
        }
    }

    pub fn quit(&mut self) {
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
            "imrscl-donplayouts" => self.execute_imrscl_donplayouts(arguments),
            "imrscl-ownership" => self.execute_imrscl_ownership(arguments),
            "imrscl-uct_gfx" => self.execute_uct_gfx(arguments),
            "kgs-genmove_cleanup" => self.execute_kgs_genmove_cleanup(arguments),
            "known_command" => self.execute_known_command(arguments),
            "komi" => self.execute_komi(arguments),
            "list_commands" => self.execute_list_commands(arguments),
            "loadsgf" => self.execute_loadsgf(arguments),
            "name" => self.execute_name(arguments),
            "play" => self.execute_play(arguments),
            "protocol_version" => self.execute_protocol_version(arguments),
            "quit" => self.execute_quit(arguments),
            "reg_genmove" => self.execute_reg_genmove(arguments),
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
        let size = self.boardsize();
        let komi = self.komi();
        self.game = Game::new(size, komi, self.ruleset());
        self.timer.setup(self.main_time, self.byo_time, self.byo_stones);
        self.controller.reset(size, komi);
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

    fn execute_reg_genmove(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(0) {
            Some(c) => {
                let started_at = precise_time_ns();
                self.timer.start(&self.game);
                let color = Color::from_gtp(c);
                let (m, playouts) = self.controller.genmove(color, &self.game, &self.timer);
                let response = match self.game.play(m) {
                    Ok(_) => {
                        self.timer.reset();
                        Ok(m.to_gtp())
                    },
                    Err(e) => {
                        Err(format!("Illegal move {:?} ({:?})", m, e))
                    }
                };
                Self::measure_playout_speed(started_at, playouts, &self.config);
                response
            },
            None => Err("missing argument".to_string())
        }
    }

    fn execute_genmove(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(0) {
            Some(comm) => {
                let started_at = precise_time_ns();
                self.timer.start(&self.game);
                let color = Color::from_gtp(comm);
                let (m, playouts) = self.controller.genmove(color, &self.game, &self.timer);
                let response = match self.game.play(m) {
                    Ok(g) => {
                        self.game = g;
                        self.timer.stop();
                        Ok(m.to_gtp())
                    },
                    Err(e) => {
                        Err(format!("Illegal move {:?} ({:?})", m, e))
                    }
                };
                Self::measure_playout_speed(started_at, playouts, &self.config);
                response
            },
            None => Err("missing argument".to_string())
        }
    }

    fn execute_kgs_genmove_cleanup(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(0) {
            Some(comm) => {
                self.game.reset_game_over();
                let started_at = precise_time_ns();
                self.timer.start(&self.game);
                let color = Color::from_gtp(comm);
                let (m, playouts) = self.controller.genmove_cleanup(color, &self.game, &self.timer);
                let response = match self.game.play(m) {
                    Ok(g) => {
                        self.game = g;
                        self.timer.stop();
                        Ok(m.to_gtp())
                    },
                    Err(e) => {
                        Err(format!("Illegal move {:?} ({:?})", m, e))
                    }
                };
                Self::measure_playout_speed(started_at, playouts, &self.config);
                response
            },
            None => Err("missing argument".to_string())
    	}
    }

    fn execute_imrscl_donplayouts(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(0) {
            Some(playouts_str) => {
                match playouts_str.parse() {
                    Ok(playouts) => {
                        self.controller.donplayouts(&self.game, playouts);
                        Ok("".to_string())
                    },
                    Err(e) => Err(format!("{:?}", e))
                }
            }
            None => Err("missing argument".to_string()),
        }
    }

    fn execute_imrscl_ownership(&mut self, _: &[&str]) -> Result<String, String> {
        let stats = self.controller.ownership_statistics();
        Ok(stats)
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
        self.game.reset_game_over();
        Ok(self.controller.final_score(&self.game))
    }

    fn execute_final_status_list(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(0) {
            Some(kind) => {
                self.game.reset_game_over();
                self.controller.final_status_list(&self.game, kind)
            },
            None => Err("missing argument".to_string())
        }
    }

    fn execute_time_settings(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(2) {
            Some(third) => {
            	//command[1] and command[2] should be there
                match (arguments[0].parse::<i64>(), arguments[1].parse::<i64>(), third.parse::<i32>()) {
                    (Ok(main), Ok(byo), Ok(stones)) => {
                        self.main_time = main;
                        self.byo_time = byo;
                        self.byo_stones = stones;
                        self.timer.setup(main, byo, stones);
                        Ok("".to_string())
                    }
                    _ => Err("error parsing time_settings".to_string())
                }
            },
            None => Err("missing argument(s)".to_string())
        }
    }

    fn execute_uct_gfx(&mut self, _: &[&str]) -> Result<String, String> {
        let stats = self.controller.uct_gfx();
        Ok(stats)
    }

    fn execute_time_left(&mut self, arguments: &[&str]) -> Result<String, String> {
        match arguments.get(2) {
            Some(third) => {
                match (arguments[1].parse::<i64>(), third.parse::<i32>()) {
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
            "dboard/Ownership/imrscl-ownership",
            "plist/Final Status List Dead/final_status_list dead",
            "plist/Final Status List Alive/final_status_list alive",
            "gfx/Uct Gfx/imrscl-uct_gfx"
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
        let horizontal_tab = Regex::new(r"\t").unwrap();
        let without_tabs = horizontal_tab.replace_all(input, " ");
        // Remove all control characters
        let cntrls = Regex::new(r"[:cntrl:]").unwrap();
        let without_ctrls = cntrls.replace_all(without_tabs.as_ref(), "");
        // Then we remove anything after a #
        let comment = Regex::new(r"#.*").unwrap();
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
