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

pub use config::Config;
pub use engine::EngineImpl;
pub use patterns::Matcher;
pub use ruleset::CGOS;
pub use super::GTPInterpreter;

pub use hamcrest::assert_that;
pub use hamcrest::equal_to;
pub use hamcrest::is;
pub use std::sync::Arc;

pub fn err(s: &'static str) -> Result<String, String> {
    Err(s.to_string())
}

pub fn ok(s: &'static str) -> Result<String, String> {
    Ok(s.to_string())
}

describe! interpreter {

    describe! cgos {

        before_each {
            let mut c = Config::default();
            c.ruleset = CGOS;
            let config = Arc::new(c);
            let matcher = Arc::new(Matcher::new());
            let engine = Box::new(EngineImpl::new(config.clone(), matcher));
            let mut interpreter = GTPInterpreter::new(config.clone(), engine);
        }

        it "empty string" {
            let response = interpreter.read("");
            assert_that(response, is(equal_to(err("empty command"))));
        }

        describe! loadsgf {

            it "wrong file" {
                let response = interpreter.read("loadsgf wrongfileactually\n");
                assert_that(response, is(equal_to(err("cannot load file"))));
            }

            it "one argument" {
                let response = interpreter.read("loadsgf\n");
                assert_that(response, is(equal_to(err("missing argument"))));
            }

        }

        describe! time_left {

            it "one argument" {
                let response = interpreter.read("time_left\n");
                assert_that(response, is(equal_to(err("missing argument(s)"))));
            }

            it "sets the main time" {
                let response = interpreter.read("time_left b 30 0");
                assert_that(response, is(equal_to(ok(""))));
                assert_that(interpreter.timer.main_time_left(), is(equal_to(30_000)));
            }

            it "sets the over time" {
                let response = interpreter.read("time_left b 30 1");
                assert_that(response, is(equal_to(ok(""))));
                assert_that(interpreter.timer.main_time_left(), is(equal_to(0)));
                assert_that(interpreter.timer.byo_time_left(), is(equal_to(30_000)));
                assert_that(interpreter.timer.byo_stones_left(), is(equal_to(1)));
            }

        }

        describe! time_settings {

            it "one argument" {
                let response = interpreter.read("time_settings\n");
                assert_that(response, is(equal_to(err("missing argument(s)"))));
            }

            it "sets the time" {
                let response = interpreter.read("time_settings 30 20 10\n");
                assert_that(response, is(equal_to(ok(""))));
                assert_that(interpreter.timer.main_time, is(equal_to(30_000)));
                assert_that(interpreter.timer.byo_time, is(equal_to(20_000)));
                assert_that(interpreter.timer.byo_stones, is(equal_to(10)));
            }

        }

        describe! play {

            it "one argument" {
                let response = interpreter.read("play\n");
                assert_that(response, is(equal_to(err("missing argument"))));
            }

            it "plays a move" {
                let response = interpreter.read("play b a1\n");
                assert_that(response, is(equal_to(ok(""))));
                assert_that(interpreter.game.board().vacant_point_count(), is(equal_to(360)));
            }

        }

        describe! genmove {

            it "one argument" {
                let response = interpreter.read("genmove\n");
                assert_that(response, is(equal_to(err("missing argument"))));
            }

            it "generates a move" {
                let response = interpreter.read("genmove b\n");
                assert!(response.is_ok());
            }
        }

        describe! komi {

            it "one argument" {
                let response = interpreter.read("komi\n");
                assert_that(response, is(equal_to(err("missing argument"))));
            }

            it "sets the komi" {
                let response = interpreter.read("komi 10\n");
                assert_that(response, is(equal_to(ok(""))));
                assert_that(interpreter.komi(), is(equal_to(10.0)));
            }

        }

        describe! boardsize {

            it "one argument" {
                let response = interpreter.read("boardsize\n");
                assert_that(response, is(equal_to(err("missing argument"))));
            }

            it "sets the correct size" {
                assert_that(interpreter.game.size(), is(equal_to(19)));
                let response = interpreter.read("boardsize 9\n");
                assert_that(response, is(equal_to(ok(""))));
                assert_that(interpreter.game.size(), is(equal_to(9)));
            }

            it "boardsize resets the board" {
                interpreter.read("play b a1\n").unwrap();
                interpreter.read("boardsize 9\n").unwrap();
                assert_that(interpreter.game.board().vacant_point_count(), is(equal_to(81)));
            }

        }

        describe! known_command {

            it "one argument" {
                let response = interpreter.read("known_command\n");
                assert_that(response, is(equal_to(err("missing argument"))));
            }

            it "known command" {
                let response = interpreter.read("known_command play\n");
                assert_that(response, is(equal_to(ok("true"))));
            }

            it "unknown command" {
                let response = interpreter.read("known_command XXX\n");
                assert_that(response, is(equal_to(ok("false"))));
            }

        }

        describe! list_commands {

            it "no newline at end" {
                let response = interpreter.read("list_commands\n");
                let expected = "boardsize\nclear_board\nfinal_score\nfinal_status_list\ngenmove\ngogui-analyze_commands\nimrscl-ownership\nknown_command\nkomi\nlist_commands\nloadsgf\nname\nplay\nprotocol_version\nquit\nshowboard\ntime_left\ntime_settings\nversion";
                assert_that(response, is(equal_to(ok(expected))));
            }

        }

        describe! clear_board {

            it "resets the board" {
                interpreter.read("play b a1\n").unwrap();
                let response = interpreter.read("clear_board\n");
                assert_that(response, is(equal_to(ok(""))));
                assert_eq!(361, interpreter.game.board().vacant_point_count());
            }

        }

        describe! final_score {

            it "no move" {
                let response = interpreter.read("final_score\n");
                assert_that(response, is(equal_to(ok("W+6.5"))));
            }

            it "one move" {
                interpreter.read("boardsize 4\n").unwrap();
                interpreter.read("play b c2\n").unwrap();
                let response = interpreter.read("final_score\n");
                assert_that(response, is(equal_to(ok("B+9.5"))));
            }

        }

        describe! name {

            it "returns the engine name" {
                let response = interpreter.read("name\n");
                assert_that(response, is(equal_to(ok("Iomrascalai"))));
            }

        }

        describe! protocol_version {

            it "returns 2" {
                let response = interpreter.read("protocol_version\n");
                assert_that(response, is(equal_to(ok("2"))));
            }

        }

        describe! showboard {

            it "returns a board representation" {
                interpreter.read("boardsize 3\n").unwrap();
                let response = interpreter.read("showboard\n");
                let expected = "\nkomi: 6.5\n 3 . . . \n 2 . . . \n 1 . . . \n   1 2 3 \n";
                assert_that(response, is(equal_to(ok(expected))));
            }

        }

        describe! version {

            it "returns the current version" {
                let response = interpreter.read("version\n");
                assert_that(response, is(equal_to(ok(::version::version()))));
            }
        }

        describe! quit {

            it "shuts down the interpreter" {
                let response = interpreter.read("quit\n");
                assert_that(response, is(equal_to(ok(""))));
                assert!(!interpreter.running);
            }
        }

        describe! final_status_list {

            before_each {
                interpreter.read("boardsize 3\n").unwrap();
                interpreter.read("clear_board\n").unwrap();
                interpreter.read("play b a1\n").unwrap();
                interpreter.read("play w b2\n").unwrap();
            }

            it "reports no dead stones" {
                let response = interpreter.read("final_status_list dead\n");
                assert_that(response, is(equal_to(ok(""))));
            }

            it "reports one alive stone" {
                let response = interpreter.read("final_status_list alive\n");
                assert_that(response, is(equal_to(ok("A1 B2"))));
            }

            it "reports no seki stones" {
                let response = interpreter.read("final_status_list seki\n");
                assert_that(response, is(equal_to(ok(""))));
            }

            it "returns an error on other arguments" {
                let response = interpreter.read("final_status_list other\n");
                assert_that(response, is(equal_to(err("unknown argument"))));
            }

            it "returns an error when no argument is given" {
                let response = interpreter.read("final_status_list\n");
                assert_that(response, is(equal_to(err("missing argument"))));
            }
        }

        // Gogui extensions
        describe! gogui {

            describe! analyze_commands {

                it "returns the supported analyze commands" {
                    let expected = "dboard/Ownership/imrscl-ownership";
                    let response = interpreter.read("gogui-analyze_commands\n");
                    assert_that(response, is(equal_to(ok(expected))));
                }

            }

        }

        // Our extensions
        describe! imsrcl {

            describe! ownership {

                it "returns board of ownership likelihoods" {
                    interpreter.read("boardsize 3\n").unwrap();
                    interpreter.read("clear_board\n").unwrap();
                    let response = interpreter.read("imrscl-ownership\n");
                    assert_that(response, is(equal_to(ok("0 0 0 \n0 0 0 \n0 0 0 \n"))));
                }
            }

        }

    }

}
