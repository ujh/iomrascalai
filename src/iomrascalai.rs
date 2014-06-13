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
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;
extern crate core;
extern crate rand;

use board::Black;
use board::{PlayOutOfBoard, SuicidePlay, IntersectionNotEmpty, SamePlayerPlayedTwice, GameAlreadyOver, SuperKoRuleBroken};
use board::move::{Play, Pass};

use game::Game;
use game::Minimal;

use gtp::{Quit, Name, Version, ProtocolVersion, ListCommands, KnownCommand, BoardSize, ClearBoard, Komi, GenMove, ShowBoard};

use engine::Engine;
use engine::random_engine::RandomEngine;

use std::io::stdio::stdin;
use std::os::args;

mod board;
mod game;
mod sgf;
mod gtp;
mod engine;

fn main() {
  match args().len() {
    1 => cli_mode(),
    _ => match args().get(1).as_slice() {
      "--mode-gtp" => gtp_mode(),
      _            => cli_mode()
    }
  }
}

fn cli_mode() {
  print!("Please enter the size of the new game: ");
  let mut reader = stdin();

  let size: u8 = match reader.read_line() {
    Ok(l)  => match from_str(l.as_slice().trim_chars('\n')) {
      Some(size) => size,
      None       => fail!("Couldn't convert to a number")
    },
    Err(_) => fail!("Couldn't read the line")
  };

  let mut g = Game::new(size, 6.5, Minimal);
  let mut current_player = Black;

  loop {
    if g.is_over() {
      println!("Thanks for playing, score: {}", g.score());
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

    g = match g.play(move) {
      Ok(g)                     => g,
      Err(PlayOutOfBoard)       => fail!("You can't play on invalid coordinates ({} {})", move.coords().col, move.coords().row),
      Err(IntersectionNotEmpty) => fail!("You can't play on a non-empty intersection !"),
      Err(SuicidePlay)          => fail!("You can't play a suicide move with a ruleset forbidding them! ({})", g.ruleset()),
      Err(SamePlayerPlayedTwice)=> fail!("You can't play twice"),
      Err(GameAlreadyOver)      => fail!("You can't play after 2 consecutive passes in TrompTaylor rules"),
      Err(SuperKoRuleBroken)    => fail!("You can't repeat a board position! (Superko rule)")
    };

    current_player = current_player.opposite();

    println!("");
    println!("{}", g);
    g.show_chains();
  }
}

fn gtp_mode() {
  let engine = RandomEngine::new();
  let engine_name = "Iomrascálaí";
  let engine_version = "0.1";
  let protocol_version = "2";

  let interpreter = gtp::GTPInterpreter::new();
  let mut reader = stdin();

  let mut komi = 6.5;
  let mut board_size = 19;
  let mut game = Game::with_Tromp_Taylor_rules(board_size, komi);

  loop {
    let command = interpreter.read(reader.read_line().unwrap().as_slice());

    match command {
      Name            => print!("= {}\n\n", engine_name),
      Version         => print!("= {}\n\n", engine_version),
      ProtocolVersion => print!("= {}\n\n", protocol_version),
      ListCommands    => print!("= {}\n", interpreter.gen_list_known_commands()),
      KnownCommand(b) => print!("= {}\n\n", b),
      BoardSize(size) => {
        board_size = size;
        game = Game::with_Tromp_Taylor_rules(board_size, komi);
        print!("= \n\n");
      },
      ClearBoard      => {
        game = Game::with_Tromp_Taylor_rules(board_size, komi);
        print!("= \n\n");
      },
      Komi(k)         => {
        komi = k;
        game.set_komi(k);
        print!("= \n\n");
      },
      gtp::Play(move) => {
        game = match game.play(move) {
          Ok(g)  => {print!("= \n\n"); g},
          Err(e) => {print!("? Illegal Move: {}\n\n", e); game}
        }
      },
      GenMove(c)      => {
        let generated_move = engine.gen_move(c, &game);
        game = match game.play(generated_move) {
          Ok(g)  => {print!("= {}\n\n", generated_move.to_gtp()); g},
          Err(e) => {print!("? Illegal Move: {}\n\n", e); game}
        }
      },
      ShowBoard       => {
        print!("= \n");
        println!("{}", game);
        print!("\n\n");
      }
      Quit            => {print!("= \n\n"); return;},
      _               => ()
    }
  }

}
