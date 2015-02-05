/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner                                          *
 * Copyright 2015 Thomas Poinsot                                        *
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

use engine::RandomEngine;
use game::Info;
use ruleset::Minimal;
use super::Command;
use super::GTPInterpreter;

#[test]
fn no_newline_at_end_of_list_commands() {
    let mut interpreter = GTPInterpreter::new(Minimal, Box::new(RandomEngine::new()));
    let commands    = interpreter.read("list_commands\n");
    let expected    = "play\ngenmove\nprotocol_version\nname\nversion\nknown_command\nlist_commands\nquit\nboardsize\nclear_board\nkomi\nshowboard\nfinal_score\ntime_settings\ntime_left";
    match commands {
        Command::ListCommands(cs) => assert_eq!(expected, cs.as_slice()),
        _                         => panic!("wrong match")
    }
}

#[test]
fn boardsize_sets_the_correct_size() {
    let mut interpreter = GTPInterpreter::new(Minimal, Box::new(RandomEngine::new()));
    assert_eq!(19, interpreter.game.size());
    interpreter.read("boardsize 9\n");
    assert_eq!(9, interpreter.game.size());
}

#[test]
fn boardsize_resets_the_board() {
    let mut interpreter = GTPInterpreter::new(Minimal, Box::new(RandomEngine::new()));
    interpreter.read("play b a1\n");
    interpreter.read("boardsize 9\n");
    assert_eq!(0, interpreter.game.move_number());
}

#[test]
fn play_plays_a_move() {
    let mut interpreter = GTPInterpreter::new(Minimal, Box::new(RandomEngine::new()));
    interpreter.read("play b a1\n");
    assert_eq!(1, interpreter.game.move_number());
}

#[test]
fn sets_the_komi() {
    let mut interpreter = GTPInterpreter::new(Minimal, Box::new(RandomEngine::new()));
    interpreter.read("komi 10\n");
    assert_eq!(10.0, interpreter.komi());
}

#[test]
fn sets_the_time() {
    let mut interpreter = GTPInterpreter::new(Minimal, Box::new(RandomEngine::new()));
    interpreter.read("time_settings 30 20 10\n");
    assert_eq!(30_000, interpreter.main_time());
    assert_eq!(20_000, interpreter.byo_time());
    assert_eq!(10, interpreter.byo_stones());
}

#[test]
fn clear_board_resets_the_board() {
    let mut interpreter = GTPInterpreter::new(Minimal, Box::new(RandomEngine::new()));
    interpreter.read("play b a1\n");
    interpreter.read("clear_board\n");
    assert_eq!(0, interpreter.game.move_number());
}

#[test]
fn final_score_no_move() {
    let mut interpreter = GTPInterpreter::new(Minimal, Box::new(RandomEngine::new()));
    match interpreter.read("final_score\n") {
        Command::FinalScore(score) => assert_eq!("W+6.5", score.as_slice()),
        _                          => panic!("FinalScore expected!")
    }
}

#[test]
fn final_score_one_move() {
    let mut interpreter = GTPInterpreter::new(Minimal, Box::new(RandomEngine::new()));
    interpreter.read("boardsize 4\n");
    interpreter.read("play b c2\n");
    match interpreter.read("final_score\n") {
        Command::FinalScore(score) => assert_eq!("B+9.5", score.as_slice()),
        _                          => panic!("FinalScore expected!")
    }
}
