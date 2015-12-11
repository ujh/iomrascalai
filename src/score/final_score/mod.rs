/************************************************************************
 *                                                                      *
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

use board::Coord;
use board::Empty;
use config::Config;
use game::Game;

use std::sync::Arc;

mod test;

pub struct FinalScore<'a> {
    config: Arc<Config>,
    game: &'a Game
}

impl<'a> FinalScore<'a> {

    pub fn new(config: Arc<Config>, game: &Game) -> FinalScore {
        FinalScore { config: config, game: game }
    }

    pub fn score(&self) -> String {
        format!("{}", self.game.score())
    }

    pub fn status_list(&self, kind: &str) -> Result<String, String> {
        match kind {
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
    }

}
