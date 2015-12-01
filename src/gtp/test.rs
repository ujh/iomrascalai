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
pub use super::Command;
pub use super::GTPInterpreter;

pub use std::sync::Arc;

describe! interpreter {

    before_each {
        let config = Arc::new(Config::default());
        let matcher = Arc::new(Matcher::new());
        let engine = Box::new(EngineImpl::new(config.clone(), matcher));
        let mut interpreter = GTPInterpreter::new(config.clone(), engine);
    }

    after_each {
        interpreter.quit();
    }

    it "empty string" {
        let command = interpreter.read("");
        match command {
    	    Command::Empty => (),
    	    _ => panic!("Expected Command::Empty")
        }
    }

    describe! loadsgf {

        it "wrong file" {
            interpreter.read("loadsgf wrongfileactually\n");
        }

        it "one argument" {
            interpreter.read("loadsgf\n");
        }

    }

    describe! time_left {

        it "one argument" {
            interpreter.read("time_left\n");
        }

    }

    describe! time_settings {

        it "one argument" {
            interpreter.read("time_settings\n");
        }

        it "sets the time" {
            interpreter.read("time_settings 30 20 10\n");
            assert_eq!(30_000, interpreter.timer.main_time);
            assert_eq!(20_000, interpreter.timer.byo_time);
            assert_eq!(10, interpreter.timer.byo_stones);
        }

    }

    describe! play {

        it "one argument" {
            interpreter.read("play\n");
        }

        it "plays a move" {
            interpreter.read("play b a1\n");
            assert_eq!(360, interpreter.game.board().vacant_point_count());
        }

    }

    describe! genmove {

        it "one argument" {
            interpreter.read("genmove\n");
        }

    }

    describe! komi {

        it "one argument" {
            interpreter.read("komi\n");
        }

        it "sets the komi" {
            interpreter.read("komi 10\n");
            assert_eq!(10.0, interpreter.komi());
        }

    }

    describe! boardsize {

        it "one argument" {
            interpreter.read("boardsize\n");
        }

        it "sets the correct size" {
            assert_eq!(19, interpreter.game.size());
            interpreter.read("boardsize 9\n");
            assert_eq!(9, interpreter.game.size());
        }

        it "boardsize resets the board" {
            interpreter.read("play b a1\n");
            interpreter.read("boardsize 9\n");
            assert_eq!(81, interpreter.game.board().vacant_point_count());
        }

    }

    describe! known_command {

        it "one argument" {
            interpreter.read("known_command\n");
        }

    }

    describe! list_commands {

        it "no newline at end" {
            let commands = interpreter.read("list_commands\n");
            let expected = "boardsize\nclear_board\nfinal_score\ngenmove\nknown_command\nkomi\nlist_commands\nloadsgf\nname\nplay\nprotocol_version\nquit\nshowboard\ntime_left\ntime_settings\nversion";
            match commands {
                Command::ListCommands(cs) => assert_eq!(expected, cs),
                _                         => panic!("wrong match")
            }
        }

    }

    describe! clear_board {

        it "resets the board" {
            interpreter.read("play b a1\n");
            interpreter.read("clear_board\n");
            assert_eq!(361, interpreter.game.board().vacant_point_count());
        }

    }

    describe! final_score {

        it "no move" {
            match interpreter.read("final_score\n") {
                Command::FinalScore(score) => assert_eq!("W+6.5", score),
                _                          => panic!("FinalScore expected!")
            }
        }

        it "one move" {
            interpreter.read("boardsize 4\n");
            interpreter.read("play b c2\n");
            match interpreter.read("final_score\n") {
                Command::FinalScore(score) => assert_eq!("B+9.5", score),
                _                          => panic!("FinalScore expected!")
            }
        }

    }

    // Gogui extensions
    describe! gogui {

        describe! analyze_commands {

            it "returns the supported analyze commands" {
                let response = interpreter.read("gogui-analyze_commands\n");
                let expected = "dboard/Safe Territory/imrscl-safe_territory";
                match response {
                    Command::GoGuiAnalyzeCommands(r) => assert_eq!(expected, r),
                    _ => panic!("wrong command returned: {:?}", response)
                }
            }

        }

    }
}
