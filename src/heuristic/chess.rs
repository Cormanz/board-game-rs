use std::cmp::{max, Ordering};

use chess::{Piece, ALL_PIECES};

use crate::ai::minimax::Heuristic;
use crate::ai::solver::SolverHeuristic;
use crate::board::Board;
use crate::games::chess::ChessBoard;

#[derive(Debug)]
pub struct ChessPieceValueHeuristic;

impl Heuristic<ChessBoard> for ChessPieceValueHeuristic {
    type V = i32;

    fn value(&self, board: &ChessBoard, length: u32) -> Self::V {
        if board.is_done() {
            return SolverHeuristic.value(board, length).to_i32();
        }

        let mut total = 0;

        for piece in ALL_PIECES {
            let value = match piece {
                Piece::Pawn => 1,
                Piece::Knight | Piece::Bishop => 3,
                Piece::Rook => 5,
                Piece::Queen => 9,
                Piece::King => 0,
            };

            for square in *board.inner().pieces(piece) {
                // we can unwrap here since we're iterating over the squares that contain the current piece,
                //   so we know the square must also have a color
                if board.inner().color_on(square).unwrap() == board.inner().side_to_move() {
                    total += value;
                } else {
                    total -= value;
                }
            }
        }

        total
    }

    fn merge(old: Self::V, new: Self::V) -> (Self::V, Ordering) {
        (max(old, new), new.cmp(&old))
    }
}
