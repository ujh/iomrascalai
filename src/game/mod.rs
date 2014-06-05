use board::Board;
use board::IllegalMove;
use board::move::Move;
use board::{Color, Empty, Black, White};
use board::coord::Coord;

use std::rc::Rc;

use board::hash::ZobristHashTable;

mod game_test;

#[deriving(Clone, Show, Eq, PartialEq)]
pub enum Ruleset {
    AnySizeTrompTaylor,
    Minimal
}

#[deriving(Clone)]
pub struct Game<'a> {
    board: Board<'a>,
    base_zobrist_table: Rc<ZobristHashTable>,
    komi: f32
}

impl<'a> Game<'a> {
    pub fn new(size: u8, komi: f32, ruleset: Ruleset) -> Game {
        let base_zobrist_table = Rc::new(ZobristHashTable::new(size));
        let new_board = Board::new(size, ruleset, base_zobrist_table.clone());

        Game {
            board: new_board,
            base_zobrist_table: base_zobrist_table,
            komi: komi
        }
    }

    #[allow(non_snake_case_functions)]
    pub fn with_Tromp_Taylor_rules(size: u8, komi: f32) -> Game {
        Game::new(size, komi, AnySizeTrompTaylor)
    }

    pub fn play(&self, move: Move) -> Result<Game, IllegalMove> {
        let new_board = self.board.play(move);

        match new_board {
            Ok(b) => {
                let mut new_game_state = self.clone();
                new_game_state.board = b;
                Ok(new_game_state)
            },
            Err(m) => Err(m)
        }
    }

    // Note: This method uses 1-1 as the origin point, not 0-0. 19-19 is a valid coordinate in a 19-sized board, while 0-0 is not.
    //       this is done because I think it makes more sense in the context of go. (Least surprise principle, etc...)
    pub fn get(&self, col: u8, row: u8) -> Color {
        self.board.get_coord(Coord::new(col, row))
    }

    pub fn ruleset(&self) -> Ruleset {
        self.board.ruleset()
    }

    pub fn is_over(&self) -> bool {
        self.board.is_game_over()
    }

    pub fn komi(&self) -> f32 {
        self.komi
    }

    pub fn size(&self) -> u8 {
        self.board.size()
    }

    pub fn score(&self) ->  (uint, f32) {
        let (b_score, w_score) = self.board.score();
        (b_score, w_score as f32 + self.komi)
    }

    pub fn set_komi(&mut self, komi: f32) {
        self.komi = komi;
    }

    pub fn board_size(&self) -> u8 {
        self.board.size()
    }

    pub fn show(&self) {
        println!("komi: {}", self.komi);

        // First we print the board
        for row in range(1u8, self.board.size()+1).rev() {

            // Prints the row number
            print!("{:2} ", row);

            // Prints the actual row
            for col in range(1u8, self.board.size()+1) {
                let current_coords = Coord::new(col, row);

                if self.board.get_coord(current_coords) == Empty {
                    let hoshis = &[4u8,10,16];
                    if  hoshis.contains(&row) && hoshis.contains(&col) {print!("+ ")}
                    else                                               {print!(". ")}
                } else if self.board.get_coord(current_coords) == White {print!("O ")}
                  else if self.board.get_coord(current_coords) == Black {print!("X ")}
            }
            println!("");
        }

        // Then we print the col numbers under the board
        print!("{:3}", "");
        for col in range(1, self.board.size()+1) {
            print!("{:<2}", col);
        }

        println!("");
    }

    pub fn show_chains(&self) {
        for c in self.board.chains().iter() {
            println!("{}", c.show());
        }
    }
}
