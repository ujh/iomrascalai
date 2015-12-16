/************************************************************************
 *                                                                      *
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

pub use board::Black;
pub use config::Config;
pub use game::Info;
pub use ruleset::KgsChinese;
pub use super::Timer;

pub use hamcrest::assert_that;
pub use hamcrest::close_to;
pub use hamcrest::equal_to;
pub use hamcrest::is;
pub use std::sync::Arc;
pub use time::Duration;
pub use time::PreciseTime;

pub struct TestGameInfo {
    vacant_points: u16
}

impl TestGameInfo {

    fn new(vp: u16) -> TestGameInfo {
        TestGameInfo { vacant_points: vp }
    }

}

impl Info for TestGameInfo {

    fn vacant_point_count(&self) -> u16 {
        self.vacant_points
    }

}

// This sleep function doesn't take a Duration from the time crate but
// one from the std library.
pub fn sleep_ms(ms: u64) {
    ::std::thread::sleep(::std::time::Duration::from_millis(ms));
}

describe! timer {

    before_each {
        let mut c = Config::default();
        c.time_control.c = 0.5;
        let config = Arc::new(c);
        let mut timer = Timer::new(config.clone());
    }

    describe! setup {

        it "sets the main time" {
            timer.setup(30, 20, 10);
            assert_that(timer.main_time_left, is(equal_to(30_000)));
        }

        it "sets the byoyomi time" {
            timer.setup(30, 20, 10);
            assert_that(timer.byo_time, is(equal_to(20_000)));
            assert_that(timer.byo_time_left, is(equal_to(20_000)));
        }

        it "sets the byoyomi stones" {
            timer.setup(30, 20, 10);
            assert_that(timer.byo_stones, is(equal_to(10)));
            assert_that(timer.byo_stones_left, is(equal_to(10)));
        }

        it "resets the time stamp" {
            let previous_time_stamp = timer.time_stamp;
            timer.setup(30, 20, 10);
            assert!(previous_time_stamp.to(PreciseTime::now()) > Duration::seconds(0));
        }

    }

    describe! update {

        describe! main_time {

            it "sets the main time" {
                timer.update(30, 0);
                assert_that(timer.main_time_left, is(equal_to(30_000)));
            }

            it "doesn't change the byoyomi time" {
                let byo_time_left_before = timer.byo_time_left;
                timer.update(30, 0);
                assert_that(timer.byo_time_left, is(equal_to(byo_time_left_before)));
            }

            it "doesn't change the byoyomi stones" {
                let byo_stones_left_before = timer.byo_stones_left;
                timer.update(30, 0);
                assert_that(timer.byo_stones_left, is(equal_to(byo_stones_left_before)));
            }

            it "updates the time stamp" {
                let previous_time_stamp = timer.time_stamp;
                timer.update(30, 0);
                assert!(previous_time_stamp.to(PreciseTime::now()) > Duration::seconds(0));
            }

        }

        describe! over_time {

            it "sets the main time to 0" {
                timer.update(30, 5);
                assert_that(timer.main_time_left, is(equal_to(0)));
            }

            it "sets the byoyomi time" {
                timer.update(30, 5);
                assert_that(timer.byo_time_left, is(equal_to(30_000)));
            }

            it "sets the byoyomi stones" {
                timer.update(30, 5);
                assert_that(timer.byo_stones_left, is(equal_to(5)));
            }

            it "updates the time stamp" {
                let previous_time_stamp = timer.time_stamp;
                timer.update(30, 5);
                assert!(previous_time_stamp.to(PreciseTime::now()) > Duration::seconds(0));
            }

        }


    }

    describe! start {

        it "updates the timestamp" {
            let previous_time_stamp = timer.time_stamp;
            timer.start(&TestGameInfo::new(0));
            assert!(previous_time_stamp.to(PreciseTime::now()) > Duration::seconds(0));
        }

        it "sets the current budget" {
            let previous_budget = timer.current_budget;
            timer.setup(30, 0, 0);
            timer.start(&TestGameInfo::new(9));
            assert!(timer.current_budget > previous_budget);
        }
    }

    describe! budget {

        it "returns zero if there's no time left" {
            timer.setup(0, 0, 0);
            let game_info = &TestGameInfo::new(0);
            assert_that(timer.budget(game_info).num_milliseconds(), is(equal_to(0)));
        }

        it "returns a fraction of the byo time" {
            timer.setup(0, 2, 2);
            let game_info = &TestGameInfo::new(0);
            assert_that(timer.budget(game_info).num_milliseconds(), is(equal_to(1_000)));
        }

        it "uses the vacant points to calculate during main time" {
            timer.setup(100, 0, 0);
            let game_info = &TestGameInfo::new(100);
            assert_that(timer.budget(game_info).num_milliseconds(), is(equal_to(2_000)));
        }

        it "uses a minimum of 30 points to calculate during main time" {
            timer.setup(300, 0, 0);
            let game_info = &TestGameInfo::new(10);
            assert_that(timer.budget(game_info).num_milliseconds(), is(equal_to(20_000)));
        }
    }

    describe! stop {

        it "changes the time settings" {
            timer.setup(300, 0, 0);
            sleep_ms(10);
            let previous_time_left = timer.main_time_left;
            timer.stop();
            assert!(timer.main_time_left < previous_time_left);
        }
    }

    describe! adjust_time {

        describe! main_time {

            before_each {
                timer.setup(300, 100, 10);
            }

            it "updates the main time left" {
                let previous_main_time_left = timer.main_time_left;
                sleep_ms(10);
                let elapsed = timer.time_stamp.to(PreciseTime::now()).num_milliseconds();
                timer.adjust_time();
                let expected_new_time_left = (previous_main_time_left - elapsed) as f32;
                assert_that(timer.main_time_left as f32, is(close_to(expected_new_time_left, 5.0)));
            }

            it "doesn't change the byo time" {
                let previous_byo_time = timer.byo_time_left;
                sleep_ms(10);
                timer.adjust_time();
                assert_that(timer.byo_time_left, is(equal_to(previous_byo_time)));
            }

            it "doesn't change the byo stones" {
                let previous_byo_stones = timer.byo_stones_left;
                sleep_ms(10);
                timer.adjust_time();
                assert_that(timer.byo_stones_left, is(equal_to(previous_byo_stones)));
            }

        }

        describe! over_time {

            before_each {
                timer.setup(1, 1, 10);
                timer.main_time_left = 1;
            }

            it "sets the main time to zero" {
                sleep_ms(10);
                timer.adjust_time();
                assert_that(timer.main_time_left, is(equal_to(0)));
            }

            it "reduces the byo time by the remaining amount" {
                let previous_byo_time = timer.byo_time_left;
                sleep_ms(10);
                let elapsed = timer.time_stamp.to(PreciseTime::now()).num_milliseconds();
                timer.adjust_time();
                let expected_byo_time_left = (previous_byo_time - 1 - elapsed) as f32;
                assert_that(timer.byo_time_left as f32, is(close_to(expected_byo_time_left, 5.0)));
            }

            it "reduces the number of byo stones" {
                sleep_ms(10);
                timer.adjust_time();
                assert_that(timer.byo_stones_left, is(equal_to(9)));
            }

        }

        describe! last_stone_in_overtime {

            before_each {
                timer.setup(0, 1, 10);
                timer.byo_stones_left = 1;
                timer.byo_time_left = 100;
                sleep_ms(10);
                timer.adjust_time();
            }

            it "resets the byo time" {
                assert_that(timer.byo_time_left, is(equal_to(1_000)));
            }

            it "resets the byo stones" {
                assert_that(timer.byo_stones_left, is(equal_to(10)));
            }
        }

    }

    describe! ran_out_of_time {

        it "returns false if there's still time left" {
            timer.current_budget = Duration::seconds(1);
            assert!(!timer.ran_out_of_time(0.5));
        }

        it "returns true if the time is up" {
            timer.current_budget = Duration::milliseconds(1);
            sleep_ms(10);
            assert!(timer.ran_out_of_time(0.5));
        }

        describe! over_5_percent_threshold {

            before_each {
                let win_ratio = config.time_control.fastplay5_thres + 0.01;
            }

            it "returns false if less than 5% of the time is up" {
                timer.current_budget = Duration::seconds(1);
                assert!(!timer.ran_out_of_time(win_ratio));
            }

            it "returns true if more than 5% of the time is up" {
                timer.current_budget = Duration::milliseconds(100);
                sleep_ms(10);
                assert!(timer.ran_out_of_time(win_ratio));
            }

        }

        describe! over_20_percent_threshold {

            before_each {
                let win_ratio = config.time_control.fastplay20_thres + 0.01;
            }

            it "returns false if less than 20% of the time is up" {
                timer.current_budget = Duration::seconds(1);
                assert!(!timer.ran_out_of_time(win_ratio));
            }

            it "returns false if more than 5% but less than 20% of the time is up" {
                timer.current_budget = Duration::milliseconds(1000);
                sleep_ms(55);
                assert!(!timer.ran_out_of_time(win_ratio));
            }

            it "returns true if more than 20% of the time is up" {
                timer.current_budget = Duration::milliseconds(50);
                sleep_ms(12);
                assert!(timer.ran_out_of_time(win_ratio));
            }

        }

    }
}
