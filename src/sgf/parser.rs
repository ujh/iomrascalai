/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner                                          *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
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
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use board::Black;
use board::Color;
use board::Empty;
use board::IllegalMove;
use board::Pass;
use board::Play;
use board::White;
use game::Game;
use ruleset::Minimal;

pub struct Parser {
    sgf: String
}

#[derive(Debug)]
struct Property<'a> {
    name: &'a str,
    val:  &'a str
}

impl<'a> Property<'a> {

    fn col(&self) -> u8 {
        self.char_to_int(self.val.as_bytes()[0])
    }

    // SGF counts from top to bottom, while we count from bottom to
    // top (and we start at 1).
    fn row(&self, size: u8) -> u8 {
        size - self.char_to_int(self.val.as_bytes()[1]) + 1
    }

    fn char_to_int(&self, c: u8) -> u8 {
        c - ('a' as u8) + 1
    }

    fn is_pass(&self) -> bool {
        self.val == ""
    }

    fn is_move(&self) -> bool {
        match self.name {
            "AB" | "AW" | "B" | "W" => true,
            _                       => false
        }
    }

    fn color(&self) -> Color {
        match self.name {
            "AB" | "B" => Black,
            "AW" | "W" => White,
            _          => Empty
        }
    }

    fn play(&self, game: Game) -> Result<Game, IllegalMove> {
        if self.is_move() {
            if self.is_pass() {
                game.play(Pass(self.color()))
            } else {
                game.play(Play(self.color(), self.col(), self.row(game.size())))
            }
        } else {
            Ok(game)
        }
    }
}

impl Parser {
    pub fn new(sgf: String) -> Parser {
        Parser {sgf: sgf}
    }

    pub fn attempt_from_path(path: &Path) -> Result<Parser, ::std::io::Error> {
    	match File::open(&path) {
    		Ok(mut file) => {
		        let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                	Ok(_) => Ok(Parser::new(contents)),
                	Err(e) => Err(e)
            	}
			},
    		Err(e) => Err(e)
		}
	}

    pub fn from_path(path: &Path) -> Parser {
        let mut f = File::open(&path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        Parser::new(contents)
    }

    pub fn game(&self) -> Result<Game, IllegalMove> {
        let mut game = Game::new(self.size(), self.komi(), Minimal);
        let props = self.tokenize();
        for prop in props.iter() {
            match prop.play(game) {
                Ok(g) => {
                    game = g;
                },
                Err(e) => return Err(e)
            }
        }
        Ok(game)
    }

    fn size(&self) -> u8 {
        let props = self.tokenize();
        let prop = props.iter().find(|p| p.name == "SZ");
        prop.and_then(|p| p.val.parse().ok()).unwrap_or(19)
    }

    fn komi(&self) -> f32 {
        let props = self.tokenize();
        let prop = props.iter().find(|p| p.name == "KM");
        prop.and_then(|p| p.val.parse().ok()).unwrap_or(6.5)
    }

    fn tokenize<'a>(&'a self) -> Vec<Property<'a>> {
        let mut tokens = Vec::new();
        let mut prev_name = "";
        let re = regex!(r"([:upper:]{1,2})?\[([^]]*)\]");
        for caps in re.captures_iter(self.sgf.as_ref()) {
            match caps.at(1) {
                Some(name) => {
                    tokens.push(Property {name: name, val: caps.at(2).unwrap()});
                    prev_name = name;
                }
                None => {
                    tokens.push(Property {name: prev_name, val: caps.at(2).unwrap()});
                }
            }
        }
        tokens
    }

}
