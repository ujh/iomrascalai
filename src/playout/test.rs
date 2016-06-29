/************************************************************************
 *                                                                      *
 * Copyright 2015 Thomas Poinsot, Igor Polyakov, Urban Hafner           *
 * Copyright 2016 Urban Hafner                                          *
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
use board::Board;
use board::Play;
use config::Config;
use patterns::SmallPatternMatcher;
use ruleset::KgsChinese;
use super::Playout;

use rand::weak_rng;
use std::sync::Arc;
use test::Bencher;

fn config() -> Arc<Config> {
    Arc::new(Config::test_config())
}

fn playout(matcher: Arc<SmallPatternMatcher>) -> Playout {
    Playout::new(config(), matcher)
}

#[test]
fn max_moves() {
    assert_eq!(1083, playout(Arc::new(SmallPatternMatcher::new())).max_moves(19));
}

#[bench]
fn playout_09x09(b: &mut Bencher) {
    let board = Board::new(9, 6.5, KgsChinese);
    let matcher = Arc::new(SmallPatternMatcher::new());
    let playout = playout(matcher);
    let mut rng = weak_rng();
    b.iter(|| {
        let mut b = board.clone();
        playout.run(&mut b, Some(&Play(Black, 1, 1)), &mut rng)
    });
}

#[bench]
fn playout_13x13(b: &mut Bencher) {
    let board = Board::new(13, 6.5, KgsChinese);
    let matcher = Arc::new(SmallPatternMatcher::new());
    let playout = playout(matcher);
    let mut rng = weak_rng();
    b.iter(|| {
        let mut b = board.clone();
        playout.run(&mut b, Some(&Play(Black, 1, 1)), &mut rng)
    });
}

#[bench]
fn playout_19x19(b: &mut Bencher) {
    let board = Board::new(19, 6.5, KgsChinese);
    let matcher = Arc::new(SmallPatternMatcher::new());
    let playout = playout(matcher);
    let mut rng = weak_rng();
    b.iter(|| {
        let mut b = board.clone();
        playout.run(&mut b, Some(&Play(Black, 1, 1)), &mut rng)
    });
}
