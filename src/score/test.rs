/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner                                          *
 * Copyright 2015 Urban Hafner, Igor Polyakov                           *
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

pub use hamcrest::assert_that;
pub use hamcrest::contains;
pub use hamcrest::equal_to;
pub use hamcrest::is;

pub use board::Black;
pub use board::Board;
pub use board::Coord;
pub use board::Empty;
pub use board::Pass;
pub use board::Play;
pub use board::White;
pub use fixtures::load_board;

describe! score {

    describe! simple {

        before_each {
            let board = load_board("score/simple");
            let score = board.score();
        }

        it "counting" {
            assert_that(score.black_stones, is(equal_to(8)));
            assert_that(score.white_stones, is(equal_to(8)));
        }

        it "score" {
            assert_that(score.color(), is(equal_to(White)));
            assert_that(score.score(), is(equal_to(6.5)));
            assert_that(format!("{}", score), is(equal_to("W+6.5".to_string())));
        }

        describe! ownership {

            it "black" {
                let expected_black = vec!["A1", "A2", "A3", "A4", "B1", "B2", "B3", "B4"].iter()
                    .map(|s| s.to_string()).collect();
                let mut black: Vec<String> = score.owner().iter()
                    .enumerate()
                    .filter(|&(_,c)| *c == Black)
                    .map(|(i, _)| Coord::from_index(i, board.size()).to_gtp())
                    .collect();
                black.sort();
                assert_that(&black, contains(expected_black).exactly());
            }

            it "white" {
                let expected_white = vec!["D1", "D2", "D3", "D4", "D1", "D2", "D3", "D4"].iter()
                    .map(|s| s.to_string()).collect();
                let mut white: Vec<String> = score.owner().iter()
                    .enumerate()
                    .filter(|&(_,c)| *c == White)
                    .map(|(i, _)| Coord::from_index(i, board.size()).to_gtp())
                    .collect();
                white.sort();
                assert_that(&white, contains(expected_white).exactly());
            }

            it "dame" {
                let expected_dame = vec![];
                let mut dame: Vec<String> = score.owner().iter()
                    .enumerate()
                    .filter(|&(_,c)| *c == Empty)
                    .map(|(i, _)| Coord::from_index(i, board.size()).to_gtp())
                    .collect();
                dame.sort();
                assert_that(&dame, contains(expected_dame).exactly());
            }

        }

    }

    describe! disjoint_territory {

        before_each {
            let score = load_board("score/disjoint").score();
        }

        it "counting" {
            assert_that(score.black_stones, is(equal_to(9)));
            assert_that(score.white_stones, is(equal_to(16)));
        }

        it "score" {
            assert_that(score.color(), is(equal_to(White)));
            assert_that(score.score(), is(equal_to(13.5)));
            assert_that(format!("{}", score), is(equal_to("W+13.5".to_string())));
        }

        it "ownership" {
            // TODO
        }

    }

    describe! dame {

        before_each {
            let score = load_board("score/dame").score();
        }

        it "counting" {
            assert_that(score.black_stones, is(equal_to(4)));
            assert_that(score.white_stones, is(equal_to(20)));
        }

        it "score" {
            assert_that(score.color(), is(equal_to(White)));
            assert_that(score.score(), is(equal_to(22.5)));
            assert_that(format!("{}", score), is(equal_to("W+22.5".to_string())));
        }

        it "ownership" {
            // TODO
        }

    }

}
