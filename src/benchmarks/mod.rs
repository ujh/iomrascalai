extern crate time;

use board::Black;
use engine::{Engine, RandomEngine};
use game::Game;
use ruleset::KgsChinese;
use playout::Playout;
use self::time::get_time;

pub fn pps(size: u8, runtime: uint) {
    let engine = RandomEngine::new();
    let game   = Game::new(size, 6.5, KgsChinese);
    let playout_engine = Playout::new(&engine);
    let mut counter = 0;
    let start = get_time().sec;

    loop {
        playout_engine.run(&game);
        counter += 1;

        if(get_time().sec - start >= runtime as i64) {
            break;
        }
    }

    println!("Playout per second: {}", counter/runtime);
}