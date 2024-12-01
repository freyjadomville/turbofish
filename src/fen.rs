use anyhow::Result;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FENParseError {
    #[error("Invalid piece {invalid_piece:?}")]
    InvalidPiece { invalid_piece: char },

    #[error("Invalid number of sections in FEN statement, expected 6, found {count:?}")]
    InvalidSectionCount { count: usize },

    #[error("Incomplete board state - duplicate square inserted during FEN parse")]
    DuplicateSquare
}

#[derive(Clone, Copy, Debug)]
pub enum PieceColour {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    // bool is to indicate if this is a target for en passant
    Pawn(PieceColour, bool),
    Knight(PieceColour),
    Bishop(PieceColour),
    Rook(PieceColour),
    Queen(PieceColour),
    King(PieceColour),
    Empty,
}

fn location(rank: usize, file: usize) -> String {
    let file = ('a'..='h').into_iter().collect::<Vec<_>>()[file];
    format!("{}{}", file, rank)
}

fn piece_colour(piece: char) -> PieceColour {
    if piece.is_uppercase() {
        PieceColour::White
    } else {
        PieceColour::Black
    }
}

fn parse_piece(piece: char, pawn_is_target: bool) -> Result<Piece> {
    let colour = piece_colour(piece);
    match piece {
        'P' | 'p' => Ok(Piece::Pawn(colour, pawn_is_target)),
        'N' | 'n' => Ok(Piece::Knight(colour)),
        'B' | 'b' => Ok(Piece::Bishop(colour)),
        'R' | 'r' => Ok(Piece::Rook(colour)),
        'Q' | 'q' => Ok(Piece::Queen(colour)),
        'K' | 'k' => Ok(Piece::King(colour)),
        _ => Err(FENParseError::InvalidPiece {
            invalid_piece: piece,
        }
        .into()),
    }
}

pub fn parse_fen(input: &str) -> Result<HashMap<String, Piece>> {
    let pieces = input.split(' ').collect::<Vec<_>>();
    let ranks = pieces[0].split('/');
    let ranks_with_index = ranks.enumerate();
    let [_, side, castling, en_passant_target, halfmove, fullmove] = pieces[..] else {
        return Err(FENParseError::InvalidSectionCount {
            count: pieces.len(),
        }
        .into());
    };

    let mut expanded_ranks = HashMap::new();
    for (rank_index, rank) in ranks_with_index {
        let mut file_index: usize = 0;
        for piece in rank.chars() {
            match piece {
                '1'..'8' => {
                    file_index = file_index
                        + piece
                            .to_digit(10)
                            .expect("Digit case didn't produce a digit")
                            as usize
                }
                _ => {
                    let result = expanded_ranks.insert(
                        location(rank_index, file_index),
                        parse_piece(piece, en_passant_target == location(rank_index, file_index))?,
                    );
                    if let Some(_) = result {
                        return Err(FENParseError::DuplicateSquare.into())
                    }
                }
            };
        }
    }

    Ok(expanded_ranks)
}
