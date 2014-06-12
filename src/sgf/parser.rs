/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner                                          *
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

use board::Black;
use board::Color;
use board::Empty;
use board::IllegalMove;
use board::White;
use board::move::Pass;
use board::move::Play;
use game::Game;
use game::Minimal;

pub struct Parser {
    sgf: String
}

#[deriving(Show)]
pub enum Error {
    SyntaxError,
    IllegalMove
}

#[deriving(Show)]
struct Property<'a> {
    name: &'a str,
    val:  &'a str
}

impl<'a> Property<'a> {

    fn col(&self) -> u8 {
        self.char_to_int(self.val[0])
    }

    // SGF counts from top to bottom, while we count from bottom to
    // top (and we start at 1).
    fn row(&self, size: u8) -> u8 {
        size - self.char_to_int(self.val[1]) + 1
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

    pub fn game(&self) -> Result<Game, Error> {
        let mut game = Game::new(self.size(), self.komi(), Minimal);
        let props = self.tokenize();
        for prop in props.iter() {
            match prop.play(game) {
                Ok(g) => {
                    game = g;
                },
                Err(e) => return Err(IllegalMove)
            }
        }
        Ok(game)
    }

    fn size(&self) -> u8 {
        let props = self.tokenize();
        let prop = props.iter().find(|p| p.name == "SZ").unwrap();
        from_str(prop.val).unwrap()
    }

    fn komi(&self) -> f32 {
        let props = self.tokenize();
        let prop = props.iter().find(|p| p.name == "KM").unwrap();
        from_str(prop.val).unwrap()
    }

    fn tokenize<'a>(&'a self) -> Vec<Property<'a>> {
        let mut tokens = Vec::new();
        let mut prev_name = "";
        let re = regex!(r"([:upper:]{1,2})?\[([^]]*)\]");
        for caps in re.captures_iter(self.sgf.as_slice()) {
            if caps.at(1) == "" {
                tokens.push(Property {name: prev_name, val: caps.at(2)});
            } else {
                tokens.push(Property {name: caps.at(1), val: caps.at(2)});
                prev_name = caps.at(1);
            }
        }
        tokens
    }

}
