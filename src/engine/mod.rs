use board::move::Move;
use board::Color;

use game::Game;

pub mod random_engine;

pub trait Engine {
    fn gen_move(&self, Color, &Game) -> Move;
}