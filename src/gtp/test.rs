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

    it "empty string" {
        let command = interpreter.read("");
        match command {
    	    Command::Empty => (),
    	    _ => panic!("Expected Command::Empty")
        }
        interpreter.quit();
    }

    it "loadsgf wrong file" {
        interpreter.read("loadsgf wrongfileactually\n");
        interpreter.quit();
    }

    it "loadsgf one argument" {
        interpreter.read("loadsgf\n");
        interpreter.quit();
    }

    it "time_left one argument" {
        interpreter.read("time_left\n");
        interpreter.quit();
    }

    it "time_settings one argument" {
        interpreter.read("time_settings\n");
        interpreter.quit();
    }

    it "play one argument" {
        interpreter.read("play\n");
        interpreter.quit();
    }

    it "genmove one argument" {
        interpreter.read("genmove\n");
        interpreter.quit();
    }

    it "komi one argument" {
        interpreter.read("komi\n");
        interpreter.quit();
    }

    it "boardsize one argument" {
        interpreter.read("boardsize\n");
        interpreter.quit();
    }

    it "known_command one argument" {
        interpreter.read("known_command\n");
        interpreter.quit();
    }

    it "no newline at end of list_commands" {
        let commands = interpreter.read("list_commands\n");
        let expected = "boardsize\nclear_board\nfinal_score\ngenmove\nknown_command\nkomi\nlist_commands\nloadsgf\nname\nplay\nprotocol_version\nquit\nshowboard\ntime_left\ntime_settings\nversion";
        match commands {
            Command::ListCommands(cs) => assert_eq!(expected, cs),
            _                         => panic!("wrong match")
        }
        interpreter.quit();
    }

    it "boardsize sets the correct size" {
        assert_eq!(19, interpreter.game.size());
        interpreter.read("boardsize 9\n");
        interpreter.quit();
        assert_eq!(9, interpreter.game.size());
    }

    it "boardsize resets the board" {
        interpreter.read("play b a1\n");
        interpreter.read("boardsize 9\n");
        interpreter.quit();
        assert_eq!(81, interpreter.game.board().vacant_point_count());
    }

    it "play plays a move" {
        interpreter.read("play b a1\n");
        interpreter.quit();
        assert_eq!(360, interpreter.game.board().vacant_point_count());
    }

    it "sets the komi" {
        interpreter.read("komi 10\n");
        interpreter.quit();
        assert_eq!(10.0, interpreter.komi());
    }

    it "sets the time" {
        interpreter.read("time_settings 30 20 10\n");
        interpreter.quit();
        assert_eq!(30_000, interpreter.timer.main_time);
        assert_eq!(20_000, interpreter.timer.byo_time);
        assert_eq!(10, interpreter.timer.byo_stones);
    }

    it "clear_board resets the board" {
        interpreter.read("play b a1\n");
        interpreter.read("clear_board\n");
        interpreter.quit();
        assert_eq!(361, interpreter.game.board().vacant_point_count());
    }

    it "final_score no move" {
        match interpreter.read("final_score\n") {
            Command::FinalScore(score) => assert_eq!("W+6.5", score),
            _                          => panic!("FinalScore expected!")
        }
        interpreter.quit();
    }

    it "final_score one move" {
        interpreter.read("boardsize 4\n");
        interpreter.read("play b c2\n");
        match interpreter.read("final_score\n") {
            Command::FinalScore(score) => assert_eq!("B+9.5", score),
            _                          => panic!("FinalScore expected!")
        }
        interpreter.quit();
    }

}
