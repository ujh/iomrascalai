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

pub use board::Black;
pub use board::Empty;
pub use board::White;
pub use super::Point;

describe! matches {

    describe! black {

        before_each {
            let point = Point::Black;
        }

        it "matches black" {
            assert!(point.matches(Some(Black)));
        }

        it "doesn't match white" {
            assert!(!point.matches(Some(White)));
        }

        it "doesn't match empty" {
            assert!(!point.matches(Some(Empty)));
        }

        it "doesn't match off board" {
            assert!(!point.matches(None));
        }

    }

    describe! white {

        before_each {
            let point = Point::White;
        }

        it "doesn't match black" {
            assert!(!point.matches(Some(Black)));
        }

        it "matches white" {
            assert!(point.matches(Some(White)));
        }

        it "doesn't match empty" {
            assert!(!point.matches(Some(Empty)));
        }

        it "doesn't match off board" {
            assert!(!point.matches(None));
        }

    }

    describe! all {

        before_each {
            let point = Point::All;
        }

        it "matches black" {
            assert!(point.matches(Some(Black)));
        }

        it "matches white" {
            assert!(point.matches(Some(White)));
        }

        it "matches empty" {
            assert!(point.matches(Some(Empty)));
        }

        it "matches off board" {
            assert!(point.matches(None));
        }

    }

    describe! not_black {

        before_each {
            let point = Point::NotBlack;
        }

        it "doesn't match black" {
            assert!(!point.matches(Some(Black)));
        }

        it "matches white" {
            assert!(point.matches(Some(White)));
        }

        it "matches empty" {
            assert!(point.matches(Some(Empty)));
        }

        it "matches off board" {
            assert!(point.matches(None));
        }

    }

    describe! not_white {

        before_each {
            let point = Point::NotWhite;
        }

        it "matches black" {
            assert!(point.matches(Some(Black)));
        }

        it "doesn't match white" {
            assert!(!point.matches(Some(White)));
        }

        it "matches empty" {
            assert!(point.matches(Some(Empty)));
        }

        it "matches off board" {
            assert!(point.matches(None));
        }

    }

    describe! empty {

        before_each {
            let point = Point::Empty;
        }

        it "doesn't match black" {
            assert!(!point.matches(Some(Black)));
        }

        it "doesn't match white" {
            assert!(!point.matches(Some(White)));
        }

        it "matches empty" {
            assert!(point.matches(Some(Empty)));
        }

        it "doesn't match off board" {
            assert!(!point.matches(None));
        }

    }

    describe! off_board {

        before_each {
            let point = Point::OffBoard;
        }

        it "doesn't match black" {
            assert!(!point.matches(Some(Black)));
        }

        it "doesn't match white" {
            assert!(!point.matches(Some(White)));
        }

        it "doesn't match empty" {
            assert!(!point.matches(Some(Empty)));
        }

        it "matches off board" {
            assert!(point.matches(None));
        }


    }

}
