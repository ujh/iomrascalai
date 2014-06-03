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
#![feature(phase)]
#[phase(syntax)]
extern crate regex_macros;
extern crate regex;
extern crate core;
extern crate rand;

use board::Black;
use board::{PlayOutOfBoard, SuicidePlay, IntersectionNotEmpty, SamePlayerPlayedTwice, GameAlreadyOver, SuperKoRuleBroken};
use board::Minimal;
use board::hash::ZobristHashTable;
use board::move::{Play, Pass};
use std::io::stdio::stdin;

mod board;
mod sgf;

fn main() {
  print!("Please enter the size of the new game: ");
  let mut reader = stdin();

  let size: u8 = match reader.read_line() {
    Ok(l)  => match from_str(l.as_slice().trim_chars('\n')) {
      Some(size) => size,
      None       => fail!("Couldn't convert to a number")
    },
    Err(_) => fail!("Couldn't read the line")
  };

  let z_hash_table = ZobristHashTable::new(size);
  let mut b = board::Board::new(size, 6.5, Minimal, &z_hash_table);
  let mut current_player = Black;

  loop {
    if b.is_game_over() {
      println!("Thanks for playing, score: {}", b.score());
      return;
    }

    print!("{} to play (Enter coordinates separated by space) or p to pass: ", current_player);

    let line = reader.read_line().unwrap();

    let move = if line.as_slice() == "p\n" {
      Pass(current_player)
    } else {
      let coords: Vec<u8> = line.as_slice().trim_chars('\n').split(' ').map(|s| from_str(s).unwrap()).collect();
      Play(current_player, *coords.get(0), *coords.get(1))
    };

    b = match b.play(move) {
      Ok(b)                     => b,
      Err(PlayOutOfBoard)       => fail!("You can't play on invalid coordinates ({} {})", move.coords().col, move.coords().row),
      Err(IntersectionNotEmpty) => fail!("You can't play on a non-empty intersection !"),
      Err(SuicidePlay)          => fail!("You can't play a suicide move with a ruleset forbidding them! ({})", b.ruleset()),
      Err(SamePlayerPlayedTwice)=> fail!("You can't play twice"),
      Err(GameAlreadyOver)      => fail!("You can't play after 2 consecutive passes in TrompTaylor rules"),
      Err(SuperKoRuleBroken)    => fail!("You can't repeat a board position! (Superko rule)")
    };

    current_player = current_player.opposite();

    println!("");
    b.show();
    println!("hash: {}", b.hash());
    b.show_chains();
  }
}
