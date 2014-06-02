use board::Board;
use board::Ruleset;
use board::IllegalMove;
use board::move::Move;
use board::Color;
use board::coord::Coord;
use board::TrompTaylor;

use std::rc::Rc;

use board::hash::ZobristHashTable;

#[deriving(Clone)]
pub struct Game<'a> {
    board: Board<'a>,
    base_zobrist_table: Rc<ZobristHashTable>
}

impl<'a> Game<'a> {
    pub fn new(size: u8, komi: f32, ruleset: Ruleset) -> Game {
        let base_zobrist_table = Rc::new(ZobristHashTable::new(size));
        let new_board = Board::new(size, komi, ruleset, base_zobrist_table.clone());

        Game {
            board: new_board,
            base_zobrist_table: base_zobrist_table
        }
    }

    pub fn with_Tromp_Taylor_rules(size: u8, komi: f32) -> Game {
        Game::new(size, komi, TrompTaylor)
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

    pub fn score(&self) ->  (uint, f32) {
        self.board.score()
    }

    pub fn show(&self) {
        self.board.show();
    }

    pub fn show_chains(&self) {
        self.board.show_chains();
    }
}