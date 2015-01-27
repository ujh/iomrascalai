/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot, Urban Hafner                          *
 * Copyright 2015 Urban Hafner, Thomas Poinsot                          *
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
use engine::Engine;
use game::Game;
use ruleset::Ruleset;
use timer::Timer;

use time::PreciseTime;

pub mod driver;
mod test;

#[derive(Debug)]
pub enum Command {
    Play,
    PlayError(Move),
    GenMove(String),
    GenMoveError(Move),
    ProtocolVersion,
    Name,
    Version,
    KnownCommand(bool),
    ListCommands(String),
    Quit,
    BoardSize,
    ClearBoard,
    Komi,
    ShowBoard(String),
    Empty,
    Error,
    FinalScore(String),
    TimeSettings
}

pub struct GTPInterpreter<'a> {
    engine: Box<Engine + 'a>,
    game: Game<'a>,
    known_commands: Vec<String>,
    ruleset: Ruleset,
    timer: Timer,
}

impl<'a> GTPInterpreter<'a> {
    pub fn new<'b>(ruleset: Ruleset, engine: Box<Engine + 'b>) -> GTPInterpreter<'b> {
        let komi      = 6.5;
        let boardsize = 19;
        let mut interpreter = GTPInterpreter {
            engine: engine,
            game: Game::new(boardsize, komi, ruleset),
            known_commands: vec!(),
            ruleset: ruleset,
            timer: Timer::new(),
        };
        interpreter.initialize();
        interpreter
    }

    fn initialize(&mut self) {
        self.known_commands = self.generate_known_commands();
    }

    fn generate_known_commands(&self) -> Vec<String> {
        let mut known_commands = Vec::new();
        known_commands.push(String::from_str("play"));
        known_commands.push(String::from_str("genmove"));
        known_commands.push(String::from_str("protocol_version"));
        known_commands.push(String::from_str("name"));
        known_commands.push(String::from_str("version"));
        known_commands.push(String::from_str("known_command"));
        known_commands.push(String::from_str("list_commands"));
        known_commands.push(String::from_str("quit"));
        known_commands.push(String::from_str("boardsize"));
        known_commands.push(String::from_str("clear_board"));
        known_commands.push(String::from_str("komi"));
        known_commands.push(String::from_str("showboard"));
        known_commands.push(String::from_str("final_score"));
        known_commands.push(String::from_str("time_settings"));
        known_commands
    }

    pub fn game<'b>(&'b self) -> &'b Game {
        &self.game
    }

    pub fn komi(&self) -> f32 {
        self.game.komi()
    }

    pub fn ruleset(&self) -> Ruleset {
        self.ruleset
    }

    pub fn main_time(&self) -> u64 {
        self.timer.main_time()
    }

    pub fn byo_time(&self) -> u64 {
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

        if preprocessed.len() == 0 {return Command::Empty};

        let command: Vec<&str> = preprocessed.as_slice().split(' ').collect();

        match command[0] {
            "name"             => return Command::Name,
            "version"          => return Command::Version,
            "protocol_version" => return Command::ProtocolVersion,
            "list_commands"    => return Command::ListCommands(self.list_commands()),
            "known_command"    => return Command::KnownCommand(self.known_commands.contains(&String::from_str(command[1].clone()))),
            "boardsize"        => return match command[1].parse::<u8>() {
                Ok(size) => {
                    self.game = Game::new(size, self.komi(), self.ruleset());
                    Command::BoardSize
                },
                Err(_) => Command::Error
            },
            "clear_board"      => {
                self.game = Game::new(self.boardsize(), self.komi(), self.ruleset());
                self.timer.set_main_time(0);
                self.timer.set_byo_time(30_000);
                self.timer.set_byo_stones(1);
                Command::ClearBoard
            },
            "komi"             => return match command[1].parse::<f32>() {
                Ok(komi) => {
                    self.game.set_komi(komi);
                    Command::Komi
                }
                Err(_) => Command::Error
            },
            "genmove"          => self.gen_move_for(Color::from_gtp(command[1])),
            "play"             => {
                let m = Move::from_gtp(command[1], command[2]);
                match self.game.clone().play(m) {
                    Ok(g) => {
                        self.game = g;
                        Command::Play
                    },
                    Err(_) => {
                        Command::PlayError(m)
                    }
                }
            },
            "showboard"   => Command::ShowBoard(format!("\n{}", self.game)),
            "quit"        => return Command::Quit,
            "final_score" => return Command::FinalScore(format!("{}", self.game.score())),
            "time_settings" => {
                match (command[1].parse::<u64>(), command[2].parse::<u64>(), command[3].parse::<i32>()) {
                    (Some(main), Some(byo), Some(stones)) => {
                        self.timer.set_main_time(main * 1000);
                        self.timer.set_byo_time(byo * 1000);
                        self.timer.set_byo_stones(stones);
                        Command::TimeSettings
                    }
                    _ => Command::Error
                }
            },
            _             => return Command::Error
        }
    }

    fn preprocess(&self, input: &str) -> String {
        // Convert tab to space
        let horizontal_tab = regex!(r"\t");
        let without_tabs = horizontal_tab.replace_all(input, " ");
        // Remove all control characters
        let cntrls = regex!(r"[:cntrl:]");
        let without_ctrls = cntrls.replace_all(without_tabs.as_slice(), "");
        // Then we remove anything after a #
        let comment = regex!(r"#.*");
        let without_comment = comment.replace(without_ctrls.as_slice(), "");
        // We remove the whitespaces before/after the string
        without_comment.as_slice().trim().to_string()
    }

    fn list_commands(&self) -> String {
        let mut result = String::new();

        for c in self.known_commands.iter() {
            result.push_str(c.as_slice());
            result.push_str("\n");
        }
        result.pop();
        result
    }

    fn gen_move_for(&mut self, color: Color) -> Command {
        let start_time = PreciseTime::now();
        let time_budget = self.compute_time_budget();
        println!("| Time budget for this move: {} |", time_budget);
        let m  = self.engine.gen_move(color, &self.game, time_budget);

        match self.game.clone().play(m) {
            Ok(g) => {
                self.game = g;
                let time_left = self.timer.main_time();

                let new_time_left = if time_left > start_time.to(PreciseTime::now()).num_milliseconds() as u64 {
                    time_left - start_time.to(PreciseTime::now()).num_milliseconds() as u64
                } else {
                    0u64
                };

                println!("| New time left: {} |", new_time_left);
                self.timer.set_main_time(new_time_left);
                self.timer.play();
                Command::GenMove(m.to_gtp())
            },
            Err(_) => {
                Command::GenMoveError(m)
            }
        }
    }

    fn compute_time_budget(&self) -> u64 {
        let max_time = match (self.timer.main_time(), self.timer.byo_time()) {
            (main, byo) if main == 0 => { // We're in byo-yomi
                byo / self.timer.byo_stones() as u64
            }
            (main, byo) if byo == 0  => { // We have an absolute clock
                let weighted_board_size = (self.game.board_size() * self.game.board_size()) as f64  * 1.5f64;
                let est_max_nb_move_left = weighted_board_size as u64 - self.game.move_number() as u64;
                main / est_max_nb_move_left
            }
            (main, _)   if main > 0  => {
                // Dumb strategy for the moment, we use the main time to play about the first half of the game;
                let est_half_game = self.game.board_size() as u64 * self.game.board_size()  as u64 / 2 - self.game.move_number() as u64;
                main / est_half_game
            }
            (main, byo) => panic!("The timer run into a strange configuration: main time: {}, byo time: {}", main, byo)
        };

        let lag_time = 1000;
        if max_time < lag_time {
            // If we have less than lag time to think, try to use half of it.
            lag_time / 2
        } else {
            max_time - lag_time
        }

    }
}
