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

#![cfg(test)]

use board::Black;
use config::Config;
use game::Game;
use ruleset::KgsChinese;
use super::Node;

use rand::weak_rng;
use std::sync::Arc;
use test::Bencher;

#[bench]
fn uct_playout_19x19(b: &mut Bencher) {
    let game = Game::new(19, 6.5, KgsChinese);
    let mut rng = weak_rng();
    let mut root = Node::root(&game);
    let config = Arc::new(Config::default());
    b.iter(|| root.run_playout(Black, config.clone(), &mut rng));
}
