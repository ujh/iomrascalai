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

use config::Config;
use game::Info;
use super::Timer;

use std::sync::Arc;
use std::thread::sleep_ms;

fn config() -> Config {
    Config::default()
}

#[test]
fn the_timer_doesnt_start_on_new() {
    let clock = Timer::new(config()).clock;
    assert!(clock.stopped());
}

#[test]
fn the_timer_has_a_default_of_5_min_sudden_death() {
    let timer = Timer::new(config());
    assert_eq!(5*60*1000, timer.main_time);
    assert_eq!(0, timer.byo_time);
    assert_eq!(0, timer.byo_stones);
}

#[test]
fn reset_stops_the_clock_and_resets_everything() {
    let mut timer = Timer::new(config());
    timer.main_time_left  = 1;
    timer.byo_time_left   = 1;
    timer.byo_stones_left = 1;
    timer.reset();
    assert_eq!(timer.main_time, timer.main_time_left);
    assert_eq!(timer.byo_time, timer.byo_time_left);
    assert_eq!(timer.byo_stones, timer.byo_stones_left);
    assert!(timer.clock.stopped());
}

#[test]
fn update_converts_to_ms_and_resets_everything() {
    let mut timer = Timer::new(config());
    timer.main_time_left  = 1;
    timer.byo_time_left   = 1;
    timer.byo_stones_left = 1;
    timer.setup(2, 2, 2);
    assert_eq!(2000, timer.main_time);
    assert_eq!(2000, timer.main_time_left);
    assert_eq!(2000, timer.byo_time);
    assert_eq!(2000, timer.byo_time_left);
    assert_eq!(2, timer.byo_stones);
    assert_eq!(2, timer.byo_stones_left);
    assert!(timer.clock.stopped());
}

#[test]
fn update_sets_the_main_time() {
    let mut timer = Timer::new(config());
    timer.update(1, 0);
    assert_eq!(1000, timer.main_time_left);
}

#[test]
fn update_sets_the_byo_time() {
    let mut timer = Timer::new(config());
    timer.update(1, 1);
    assert_eq!(0, timer.main_time_left);
    assert_eq!(1000, timer.byo_time_left);
    assert_eq!(1, timer.byo_stones_left);
}

#[test]
fn update_starts_the_clock() {
    let mut timer = Timer::new(config());
    timer.update(1, 0);
    assert!(timer.clock.running());
}

#[test]
fn start_starts_the_clock() {
    let mut timer = Timer::new(config());
    timer.start();
    assert!(timer.clock.running());
}

#[test]
fn stop_changes_the_time_left() {
    let mut timer = Timer::new(config());
    timer.start();
    sleep_ms(10);
    timer.stop();
    assert!(timer.main_time_left < timer.main_time);
}

#[test]
fn stop_stops_the_clock() {
    let mut timer = Timer::new(config());
    timer.stop();
    assert!(timer.clock.stopped());
}

#[test]
fn adjust_time_updates_the_main_time() {
    let mut timer = Timer::new(config());
    assert_eq!(5*60*1000, timer.main_time_left);
    timer.clock.start = Some(0);
    timer.clock.end   = Some(1000000);
    timer.adjust_time();
    assert_eq!(5*60*1000-1, timer.main_time_left);
}

#[test]
fn adjust_time_updates_the_byo_time() {
    let mut timer = Timer::new(config());
    timer.setup(0, 1, 2);
    timer.clock.start = Some(0);
    timer.clock.end   = Some(1000000);
    timer.adjust_time();
    assert_eq!(0, timer.main_time_left);
    assert_eq!(999, timer.byo_time_left);
}

#[test]
fn adjust_time_updates_the_byo_stones() {
    let mut timer = Timer::new(config());
    timer.setup(0, 1, 2);
    timer.clock.start = Some(0);
    timer.clock.end   = Some(1000000);
    timer.adjust_time();
    assert_eq!(1, timer.byo_stones_left);
}

#[test]
fn adjust_time_resets_the_byo_time_after_the_last_move() {
    let mut timer = Timer::new(config());
    timer.setup(0, 1, 2);
    timer.byo_time_left   = 500;
    timer.byo_stones_left = 1;
    timer.clock.start = Some(0);
    timer.clock.end   = Some(1000000);
    timer.adjust_time();
    assert_eq!(0, timer.main_time_left);
    assert_eq!(1000, timer.byo_time_left);
    assert_eq!(2, timer.byo_stones_left);
}

#[test]
fn adjust_time_splits_time_between_main_and_byo_time() {
    let mut timer = Timer::new(config());
    timer.setup(1, 2, 2);
    timer.clock.start = Some(0);
    timer.clock.end   = Some(1000000*1000*2);
    timer.adjust_time();
    assert_eq!(0, timer.main_time_left);
    assert_eq!(1000, timer.byo_time_left);
    assert_eq!(1, timer.byo_stones_left);
}

#[test]
fn adjust_time_sets_remaining_time_to_zero_in_absolute_time_if_time_is_over() {
    let mut timer = Timer::new(config());
    timer.setup(1, 0, 0);
    timer.clock.start = Some(0);
    timer.clock.end   = Some(1000000*1000*2);
    timer.adjust_time();
    assert_eq!(0, timer.main_time_left);
    assert_eq!(0, timer.byo_time_left);
    assert_eq!(0, timer.byo_stones_left);
}

#[test]
fn adjust_time_sets_remaining_time_to_zero_in_byo_time_if_time_is_over() {
    let mut timer = Timer::new(config());
    timer.setup(0, 1, 0);
    timer.clock.start = Some(0);
    timer.clock.end   = Some(1000000*1000*2);
    timer.adjust_time();
    assert_eq!(0, timer.main_time_left);
    assert_eq!(0, timer.byo_time_left);
    assert_eq!(0, timer.byo_stones_left);
}

struct TestGameInfo {
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

#[test]
fn budget_returns_zero_if_the_time_is_over() {
    let mut timer = Timer::new(config());
    timer.main_time_left  = 0;
    timer.byo_time_left   = 0;
    timer.byo_stones_left = 0;
    let info = TestGameInfo::new(0);
    assert_eq!(0, timer.budget(&info));
}

#[test]
fn budget_returns_a_fraction_of_the_byo_time_remaining() {
    let mut timer = Timer::new(config());
    timer.main_time_left  = 0;
    timer.byo_time_left   = 2;
    timer.byo_stones_left = 2;
    let info = TestGameInfo::new(0);
    assert_eq!(1, timer.budget(&info));
}

#[test]
fn budget_rounds_down_when_calculating_the_byo_time() {
    let mut timer = Timer::new(config());
    timer.main_time_left  = 0;
    timer.byo_time_left   = 3;
    timer.byo_stones_left = 2;
    let info = TestGameInfo::new(0);
    assert_eq!(1, timer.budget(&info));
}

#[test]
fn budget_during_main_time_uses_the_vacant_points_to_calculate_the_time() {
    let mut timer = Timer::new(config());
    timer.main_time_left = 10;
    let info = TestGameInfo::new(10);
    assert_eq!(2, timer.budget(&info));
}

#[test]
fn budget_during_main_time_rounds_down() {
    let mut timer = Timer::new(config());
    timer.main_time_left = 11;
    let info = TestGameInfo::new(10);
    assert_eq!(2, timer.budget(&info));
}
