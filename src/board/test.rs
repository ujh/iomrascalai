/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner, Thomas Poinsot                          *
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

#[cfg(test)]

use board::{Board, Empty, White, Black, Stone, Coord};

#[test]
fn test_getting_a_valid_coord_returns_a_color(){
  let b = Board::new(19, 6.5);

  assert!(b.get(10,10).color == Empty);
}

#[test]
#[should_fail]
fn test_getting_invalid_coordinates_fails() {
  let b = Board::new(19, 6.5);
  b.get(14,21);
  b.get(21,14);
}

#[test]
fn test_19_19_is_a_valid_coordinate(){
  let b = Board::new(19, 6.5);

  assert!(b.get(19,19).color == Empty);
}

#[test]
#[should_fail]
fn test_0_0_is_not_a_valid_coordinate(){
  let b = Board::new(19, 6.5);

  b.get(0,0);
}

#[test]
fn test_get_komi(){
  let b = Board::new(19, 6.5);

  assert!(b.komi() == 6.5f32)
}

#[test]
fn test_play(){
  let mut b = Board::new(19, 6.5);

  b = b.play(White, 14, 14);
  assert!(b.get(14,14).color == White);
}

#[test]
fn test_neighbours_contain_NSEW() {
  let mut b = Board::new(19, 6.5);

  b = b.play(White, 10, 9);
  b = b.play(White, 9, 10);
  b = b.play(Black, 10, 11);

  let n = b.neighbours(b.get(10,10));

  assert!(n.iter().find(|p| p.coord.col == 10 && p.coord.row == 9  && p.color == White).is_some());
  assert!(n.iter().find(|p| p.coord.col == 9  && p.coord.row == 10 && p.color == White).is_some());
  assert!(n.iter().find(|p| p.coord.col == 10 && p.coord.row == 11 && p.color == Black).is_some());
  assert!(n.iter().find(|p| p.coord.col == 11 && p.coord.row == 10 && p.color == Empty).is_some());
}

#[test]
fn test_neighbours_do_not_contain_diagonals() {
  let b = Board::new(19, 6.5);

  let n = b.neighbours(b.get(10,10));

  assert!(n.iter().find(|p| p.coord.col == 11 && p.coord.row == 11).is_none());
  assert!(n.iter().find(|p| p.coord.col == 9  && p.coord.row == 11).is_none());
  assert!(n.iter().find(|p| p.coord.col == 11 && p.coord.row == 9 ).is_none());
  assert!(n.iter().find(|p| p.coord.col == 9  && p.coord.row == 9 ).is_none());
}

#[test]
fn test_neighbours_do_not_contain_itself() {
  let b = Board::new(19, 6.5);

  let n = b.neighbours(b.get(10,10));

  assert!(n.iter().find(|p| p.coord.col == 10 && p.coord.row == 10).is_none());
}

#[test]
fn test_is_inside_valid_coords_pass() {
  let b = Board::new(19, 6.5);
  assert!(b.is_inside(1,1));
  assert!(b.is_inside(19,19));
  assert!(b.is_inside(10,10));
}

#[test]
fn test_is_inside_0_0_fails() {
  let b = Board::new(19, 6.5);
  assert!(!b.is_inside(0,0));
}

#[test]
fn test_is_inside_invalid_coords_fail() {
  let b = Board::new(19, 6.5);
  assert!(!b.is_inside(4,21));
  assert!(!b.is_inside(21,4));

  let c = Board::new(9, 6.5);
  assert!(!c.is_inside(18,18));
}

#[test]
fn test_get_mut_enables_changing_the_stone() {
  let mut b = Board::new(19, 6.5);

  b.get_mut(10,10).color = Black;
  assert!(b.get(10,10).color == Black);
}

#[test]
#[should_fail]
fn test_get_mut_fails_on_invalid_coords(){
  let mut b = Board::new(19, 6.5);

  b.get_mut(21,4);
  b.get_mut(4,21);
  b.get_mut(0,0);
}

#[test]
fn test_add_chain_creates_a_new_chain_and_adds_it_to_the_board() {
  let mut b = Board::new(19, 6.5);
  let s = Stone::with_color(White, 0, 10, 10);

  b.add_chain(&s);
  assert!(b.chains.len() == 1);
}

#[test]
fn test_add_chain_contains_the_initial_stone_coordinates() {
  let mut b = Board::new(19, 6.5);
  let s = Stone::with_color(White, 0, 10, 10);

  b.add_chain(&s);
  assert!(b.chains.get(0).coords.contains(&Coord {col: 10, row: 10}));
}

#[test]
fn test_add_chain_is_the_same_color_as_the_initial_stone(){
  let mut b = Board::new(19, 6.5);
  let s = Stone::with_color(White, 0, 10, 10);

  b.add_chain(&s);
  assert!(b.chains.get(0).color == White);
}

