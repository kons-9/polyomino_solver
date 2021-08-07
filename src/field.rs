use crate::piece::Piece;
use crate::piece::Shape;
use rusttype::Point;
use std::fmt;

pub struct Field {
    field: Vec<Vec<i32>>,
    pieces: Vec<Piece>,
}

pub enum FieldType {
    ROTATE,
    MILLER,
    ALL,
}

impl Field {
    pub fn new(field: &str) -> Field {
        // not immplemented
        let field: Vec<Vec<i32>> = Vec::new();
        let tmp: Vec<i32> = Vec::new();
        field.push(tmp);
        let pieces: Vec<Piece> = Vec::new();
        Field {
            field: field,
            pieces: pieces,
        }
    }
    pub fn make_field(field: &str, pieces: Vec<&str>, filed_type: FieldType) -> Field {
        let mut field = Field::new(field);
        match filed_type {
            ROTATE => {
                for (i, piece) in pieces.iter().enumerate() {
                    let rotate_pieces = Shape::from_str(piece).all_rotate(i);
                    field.append_piece(rotate_pieces);
                }
            }
            MILLER => {
                for (i, piece) in pieces.iter().enumerate() {
                    let rotate_pieces = Shape::from_str(piece).all_miller(i);
                    field.append_piece(rotate_pieces);
                }
            }
            ALL => {
                for (i, piece) in pieces.iter().enumerate() {
                    let rotate_pieces = Shape::from_str(piece).all_rotate_and_miller(i);
                    field.append_piece(rotate_pieces);
                }
            }
        }
        field
    }
    pub fn append_piece(&mut self, piece: Piece) {
        self.pieces.push(piece);
    }

    pub fn find_empty(&self) -> Point<i32> {
        //not immplemented
        // find empty place in field.field
        // return value is coordinate
        let field = &self.field;
        Point { x: 0, y: 0 }
    }

    pub fn can_place_piece_at_point(&self, piece: &Piece, p: &Point<i32>) -> bool {
        // whether it can place the piece at the point in the field.
        true
    }
    pub fn place_piece(&mut self, piece: &Piece, p: &Point<i32>) {
        // it should be done after can_place_piece.
    }

    pub fn get_piece(&mut self) -> Piece {
        let piece = self.pieces.shapes.pop().unwrap();
        piece
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //
        let s: String = String::new();
        for line in self.field {
            for id in line {
                s = s + &id.to_string();
            }
            s = s + &"\n".to_string();
        }
        write!(f, "{}", s)
    }
}
