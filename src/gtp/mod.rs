/************************************************************************
 *                                                                      *
 * Copyright 2014 Thomas Poinsot, Urban Hafner                          *
 * Copyright 2015 Urban Hafner                                          *
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

pub mod driver;
mod test;

#[derive(Show)]
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
    FinalScore(String)
}

pub struct GTPInterpreter<'a> {
    engine: Box<Engine + 'a>,
    game: Game<'a>,
    known_commands: Vec<String>,
    ruleset: Ruleset,
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
                Some(size) => {
                    self.game = Game::new(size, self.komi(), self.ruleset());
                    Command::BoardSize
                },
                None       => Command::Error
            },
            "clear_board"      => {
                self.game = Game::new(self.boardsize(), self.komi(), self.ruleset());
                Command::ClearBoard
            },
            "komi"             => return match command[1].parse::<f32>() {
                Some(komi) => {
                    self.game.set_komi(komi);
                    Command::Komi
                }
                None       => Command::Error
            },
            "genmove"          => {
                let color = Color::from_gtp(command[1]);
                let m  = self.engine.gen_move(color, &self.game);
                match self.game.clone().play(m) {
                    Ok(g) => {
                        self.game = g;
                        Command::GenMove(m.to_gtp())
                    },
                    Err(_) => {
                        Command::GenMoveError(m)
                    }
                }
            },
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
}
