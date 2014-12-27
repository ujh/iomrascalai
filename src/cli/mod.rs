/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
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
use board::IllegalMove;
use board::Pass;
use board::Play;
use game::Game;
use ruleset::KgsChinese;

use std::io::stdio::stdin;

pub struct Driver;

impl Driver {
    pub fn new() {
        print!("Please enter the size of the new game: ");
        let mut reader = stdin();

        let size: u8 = match reader.read_line() {
            Ok(l)  => match l.as_slice().trim_chars('\n').parse() {
                Some(size) => size,
                None       => panic!("Couldn't convert to a number")
            },
            Err(_) => panic!("Couldn't read the line")
        };

        let mut g = Game::new(size, 6.5, KgsChinese);
        let mut current_player = Black;

        loop {
            if g.is_over() {
                println!("Thanks for playing, score: {}", g.score());
                return;
            }

            print!("{} to play (Enter coordinates separated by space) or p to pass: ", current_player);

            let line = reader.read_line().unwrap();

            let m = if line.as_slice() == "p\n" {
                Pass(current_player)
            } else {
                let coords: Vec<u8> = line.as_slice().trim_chars('\n').split(' ').map(|s| s.parse().unwrap()).collect();
                Play(current_player, coords[0], coords[1])
            };

            g = match g.play(m) {
                Ok(g) => g,
                Err(IllegalMove::PlayOutOfBoard) =>
                    panic!("You can't play on invalid coordinates ({} {})", m.coords().col, m.coords().row),
                Err(IllegalMove::IntersectionNotEmpty)  =>
                    panic!("You can't play on a non-empty intersection !"),
                Err(IllegalMove::SuicidePlay)           =>
                    panic!("You can't play a suicide move with a ruleset forbidding them! ({})", g.ruleset()),
                Err(IllegalMove::SamePlayerPlayedTwice) =>
                    panic!("You can't play twice"),
                Err(IllegalMove::GameAlreadyOver)      =>
                    panic!("You can't play after 2 consecutive passes in TrompTaylor rules"),
                Err(IllegalMove::SuperKoRuleBroken)    =>
                    panic!("You can't repeat a board position! (Superko rule)")
            };

            current_player = current_player.opposite();

            println!("");
            println!("{}", g);
            g.show_chains();
        }

    }
}
