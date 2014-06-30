use engine::Engine;

use board::move::{Move, Pass, Play};
use board::Color;

use game::Game;

use std::rand::random;

pub struct RandomEngine;

impl RandomEngine {
    pub fn new() -> RandomEngine {
        RandomEngine
    }
}

impl Engine for RandomEngine {
    fn gen_move(&self, color: Color, game: &Game) -> Move {
        let mut m = Pass(color);
        let mut try_counter = 0u8;

        while try_counter < 100 {
            let col = random::<u8>() % game.board_size() + 1;
            let row = random::<u8>() % game.board_size() + 1;
            let play = Play(color, col, row);

            if game.play(play).is_ok() {
                m = play;
            }

            match m {
                Play(_, _, _) => break,
                _             => ()
            }

            try_counter += 1;
        }

        m
    }
}
