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

use super::Pattern;

fn pattern() -> Pattern {
    Pattern::new(vec!(
        vec!('X', 'O', '.'),
        vec!('x', 'o', '?'),
        vec!(' ', ' ', ' ')))
}

fn expanded() -> Vec<Pattern> {
    pattern().expand()
}

// expand

#[test]
fn expand_includes_all_variations() {
    assert_eq!(expanded().len(), 12);
}

#[test]
fn expand_includes_the_original_pattern() {
    assert!(expanded().iter().any(|pat| *pat == pattern()));
}

#[test]
fn expand_includes_the_90_deg_rotated_pattern() {
    let pattern = Pattern::new(vec!(
        vec!(' ', 'x', 'X'),
        vec!(' ', 'o', 'O'),
        vec!(' ', '?', '.')));
    assert!(expanded().iter().any(|pat| *pat == pattern));
}

#[test]
fn expand_includes_the_180_deg_rotated_pattern() {
    let pattern = Pattern::new(vec!(
        vec!(' ', ' ', ' '),
        vec!('?', 'o', 'x'),
        vec!('.', 'O', 'X')));
    assert!(expanded().iter().any(|pat| *pat == pattern));
}

#[test]
fn expand_includes_the_270_deg_rotated_pattern() {
    let pattern = Pattern::new(vec!(
        vec!('.', '?', ' '),
        vec!('O', 'o', ' '),
        vec!('X', 'x', ' ')));
    assert!(expanded().iter().any(|pat| *pat == pattern));
}

#[test]
fn expand_includes_the_horizontally_flipped_pattern() {
    let pattern = Pattern::new(vec!(
        vec!(' ', ' ', ' '),
        vec!('x', 'o', '?'),
        vec!('X', 'O', '.')));
    assert!(expanded().iter().any(|pat| *pat == pattern));
}

#[test]
fn expand_includes_the_vertically_flipped_pattern() {
    let pattern = Pattern::new(vec!(
        vec!('.', 'O', 'X'),
        vec!('?', 'o', 'x'),
        vec!(' ', ' ', ' ')));
    assert!(expanded().iter().any(|pat| *pat == pattern));
}
