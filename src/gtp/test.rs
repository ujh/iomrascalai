/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner                                          *
 * Copyright 2015 Thomas Poinsot, Igor Polyakov, Urban Hafner           *
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
use engine::RandomEngine;
use super::Command;
use super::GTPInterpreter;

#[test]
fn empty_string() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    let command = interpreter.read("");
    match command {
    	Command::Empty => (),
    	_ => panic!("Expected Command::Empty")
    }
    interpreter.quit();
}

#[test]
fn loadsgf_wrong_file() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("loadsgf wrongfileactually\n");
    interpreter.quit();
}

#[test]
fn loadsgf_one_argument() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("loadsgf\n");
    interpreter.quit();
}

#[test]
fn time_left_one_argument() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("time_left\n");
    interpreter.quit();
}

#[test]
fn time_settings_one_argument() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("time_settings\n");
    interpreter.quit();
}

#[test]
fn play_one_argument() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("play\n");
    interpreter.quit();
}

#[test]
fn genmove_one_argument() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("genmove\n");
    interpreter.quit();
}

#[test]
fn komi_one_argument() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("komi\n");
    interpreter.quit();
}

#[test]
fn boardsize_one_argument() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("boardsize\n");
    interpreter.quit();
}

#[test]
fn known_command_one_argument() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("known_command\n");
    interpreter.quit();
}

#[test]
fn no_newline_at_end_of_list_commands() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    let commands    = interpreter.read("list_commands\n");
    let expected    = "boardsize\nclear_board\nfinal_score\ngenmove\nknown_command\nkomi\nlist_commands\nloadsgf\nname\nplay\nprotocol_version\nquit\nshowboard\ntime_left\ntime_settings\nversion";
    match commands {
        Command::ListCommands(cs) => assert_eq!(expected, cs),
        _                         => panic!("wrong match")
    }
    interpreter.quit();
}

#[test]
fn boardsize_sets_the_correct_size() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    assert_eq!(19, interpreter.game.size());
    interpreter.read("boardsize 9\n");
    interpreter.quit();
    assert_eq!(9, interpreter.game.size());
}

#[test]
fn boardsize_resets_the_board() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("play b a1\n");
    interpreter.read("boardsize 9\n");
    interpreter.quit();
    assert_eq!(81, interpreter.game.board().vacant_point_count());
}

#[test]
fn play_plays_a_move() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("play b a1\n");
    interpreter.quit();
    assert_eq!(360, interpreter.game.board().vacant_point_count());
}

#[test]
fn sets_the_komi() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("komi 10\n");
    interpreter.quit();
    assert_eq!(10.0, interpreter.komi());
}

#[test]
fn sets_the_time() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("time_settings 30 20 10\n");
    interpreter.quit();
    assert_eq!(30_000, interpreter.timer.main_time);
    assert_eq!(20_000, interpreter.timer.byo_time);
    assert_eq!(10, interpreter.timer.byo_stones);
}

#[test]
fn clear_board_resets_the_board() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("play b a1\n");
    interpreter.read("clear_board\n");
    interpreter.quit();
    assert_eq!(361, interpreter.game.board().vacant_point_count());
}

#[test]
fn final_score_no_move() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    match interpreter.read("final_score\n") {
        Command::FinalScore(score) => assert_eq!("W+6.5", score),
        _                          => panic!("FinalScore expected!")
    }
    interpreter.quit();
}

#[test]
fn final_score_one_move() {
    let mut interpreter = GTPInterpreter::new(Config::default(), Box::new(RandomEngine::new()));
    interpreter.read("boardsize 4\n");
    interpreter.read("play b c2\n");
    match interpreter.read("final_score\n") {
        Command::FinalScore(score) => assert_eq!("B+9.5", score),
        _                          => panic!("FinalScore expected!")
    }
    interpreter.quit();
}
