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

pub use board::Board;
pub use config::Config;
pub use ruleset::KgsChinese;
pub use super::OwnershipStatistics;

pub use hamcrest::assert_that;
pub use hamcrest::equal_to;
pub use hamcrest::is;
pub use std::sync::Arc;
pub use test::Bencher;


describe! ownership {

    describe! statistics {

        // Tests for merge
        // Tests for formatting

        describe! formatting {

            before_each {
                let config = Arc::new(Config::test_config());
                let stats = OwnershipStatistics::new(config, 3, 6.5);
            }

            it "returns 0 by default" {
                let formatted = format!("{}", stats);

                assert_that(formatted, is(equal_to("0 0 0 \n0 0 0 \n0 0 0 \n".to_string())));
            }
        }
    }

}

#[bench]
fn new(b: &mut Bencher) {
    let config = Arc::new(Config::test_config());
    b.iter(|| OwnershipStatistics::new(config.clone(), 19, 6.5))

}

#[bench]
fn merge(b: &mut Bencher) {
    let size = 19;
    let komi = 6.5;
    let config = Arc::new(Config::test_config());
    let mut ownership = OwnershipStatistics::new(config.clone(), size, komi);
    let score = Board::new(size, komi, KgsChinese).score();
    b.iter(|| ownership.merge(&score))
}
