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
use board::White;
use board::move::Pass;
use board::move::Play;
use game::Game;
use game::Minimal;

pub struct Parser {
    sgf: String
}

#[deriving(Show)]
struct Property<'a> {
    name: &'a str,
    val:  &'a str
}

impl<'a> Property<'a> {

    fn col(&self, size: u8) -> u8 {
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
}

impl Parser {
    pub fn new(sgf: String) -> Parser {
        Parser {sgf: sgf}
    }

    pub fn game(&self) -> Game {
        let mut game = Game::new(self.size(), self.komi(), Minimal);
        let props = self.tokenize();
        for prop in props.iter() {
            if prop.name == "AB" {
                let move = Play(Black, prop.col(game.size()), prop.row(game.size()));
                game = game.play(move).unwrap();
            } else if prop.name == "AW" {
                fail!("White handicap stones not implemented, yet")
            } else if prop.name == "B" {
                if prop.is_pass() {
                    let move = Pass(Black);
                    game = game.play(move).unwrap();
                } else {
                    let move = Play(Black, prop.col(game.size()), prop.row(game.size()));
                    game = game.play(move).unwrap();
                }
            } else if prop.name == "W" {
                if prop.is_pass() {
                    let move = Pass(White);
                    game = game.play(move).unwrap();
                } else {
                    let move = Play(White, prop.col(game.size()), prop.row(game.size()));
                    game = game.play(move).unwrap();
                }
            }
        }
        game
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
