/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
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

pub use super::MoveStat;
pub use super::MoveStats;


mod move_stats {

    use board::Black;
    use board::Pass;
    use board::Play;
    use super::MoveStats;

    #[test]
    fn returns_pass_as_best_move_by_default() {
        let moves = vec!();
        let stats = MoveStats::new(&moves, Black);
        let (m, _) = stats.best();
        assert_eq!(Pass(Black), m);
    }

    #[test]
    fn returns_the_best_move() {
        let moves = vec![Play(Black, 1, 1), Play(Black, 2, 2)];
        let mut stats = MoveStats::new(&moves, Black);
        stats.record_win(Play(Black, 1, 1));
        stats.record_loss(Play(Black, 1, 1));
        stats.record_win(Play(Black, 2, 2));
        stats.record_win(Play(Black, 2, 2));
        let (m, ms) = stats.best();
        assert_eq!(Play(Black, 2, 2), m);
        assert_eq!(ms.plays, 2);
        assert_eq!(ms.wins, 2);
    }

    #[test]
    fn all_wins_returns_true_when_no_losses_were_recorded() {
        let moves = vec![Play(Black, 1, 1), Play(Black, 2, 2)];
        let mut stats = MoveStats::new(&moves, Black);
        stats.record_win(Play(Black, 1, 1));
        stats.record_win(Play(Black, 2, 2));
        assert!(stats.all_wins());
    }

    #[test]
    fn all_wins_returns_false_when_a_loss_was_recorded() {
        let moves = vec![Play(Black, 1, 1), Play(Black, 2, 2)];
        let mut stats = MoveStats::new(&moves, Black);
        stats.record_loss(Play(Black, 1, 1));
        assert!(!stats.all_wins());
    }

    #[test]
    fn all_losses_returns_true_when_no_wins_were_recorded() {
        let moves = vec![Play(Black, 1, 1), Play(Black, 2, 2)];
        let mut stats = MoveStats::new(&moves, Black);
        stats.record_loss(Play(Black, 1, 1));
        stats.record_loss(Play(Black, 2, 2));
        assert!(stats.all_losses());
    }

    #[test]
    fn all_losses_returns_false_when_a_win_was_recorded() {
        let moves = vec![Play(Black, 1, 1), Play(Black, 2, 2)];
        let mut stats = MoveStats::new(&moves, Black);
        stats.record_win(Play(Black, 1, 1));
        assert!(!stats.all_losses());
    }

    #[test]
    fn record_win_does_nothing_for_untracked_moves() {
        let moves = vec!();
        let mut stats = MoveStats::new(&moves, Black);
        stats.record_win(Play(Black, 1, 1));
    }

    #[test]
    fn record_loss_does_nothing_for_untracked_moves() {
        let moves = vec!();
        let mut stats = MoveStats::new(&moves, Black);
        stats.record_loss(Play(Black, 1, 1));
    }

    #[test]
    fn merge_merges_stats_existing_in_both() {
        let m = Play(Black, 1, 1);
        let moves = vec!(m);
        let mut stats = MoveStats::new(&moves, Black);
        stats.record_win(m);
        let mut other = MoveStats::new(&moves, Black);
        other.record_loss(m);
        stats.merge(&other);
        let ms = stats.stats.get(&m).unwrap();
        assert_eq!(ms.wins, 1);
        assert_eq!(ms.plays, 2);
    }

    #[test]
    fn merge_ignores_stats_that_only_exist_in_other() {
        let m = Play(Black, 1, 1);
        let moves = vec!();
        let mut stats = MoveStats::new(&moves, Black);
        let moves2 = vec!(m);
        let mut other = MoveStats::new(&moves2, Black);
        other.record_loss(m);
        stats.merge(&other);
        assert!(!stats.stats.get(&m).is_some());
    }

}

mod move_stat {

    use super::MoveStat;

    #[test]
    fn newly_produced_move_stat_should_have_0pc_win_ratio() {
        let ms = MoveStat::new();
        assert_eq!(ms.win_ratio(), 0f32);
    }

    #[test]
    fn merge_adds_the_plays_and_wins_of_other() {
        let mut ms = MoveStat::new();
        let mut other = MoveStat::new();
        ms.wins = 1;
        ms.plays = 1;
        other.wins = 1;
        other.plays = 2;
        ms.merge(&other);
        assert_eq!(2, ms.wins);
        assert_eq!(3, ms.plays);
    }
}
