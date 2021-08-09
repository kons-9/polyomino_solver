use crate::piece::Piece;
use crate::piece::Shape;
use rusttype::Point;
use std::fmt;

pub enum FieldElement {
    ID(usize),
    NONE,
}
pub struct Field {
    field: Vec<Vec<FieldElement>>,
    pieces: Vec<Piece>,
}

pub enum PieceGenSetting {
    ROTATE,
    MILLER,
    ALL, // rotate and miller
}

impl Field {
    pub fn new(field: &str) -> Field {
        // not immplemented
        let mut field: Vec<Vec<FieldElement>> = Vec::new();
        let tmp: Vec<FieldElement> = Vec::new();
        field.push(tmp);
        let pieces: Vec<Piece> = Vec::new();
        Field {
            field: field,
            pieces: pieces,
        }
    }
    pub fn make_field(field: &str, pieces: Vec<&str>, filed_type: PieceGenSetting) -> Field {
        let mut field = Field::new(field);
        match filed_type {
            PieceGenSetting::ROTATE => {
                for (i, piece) in pieces.iter().enumerate() {
                    let rotate_pieces = Shape::from_str(piece).all_rotate(i);
                    field.append_piece(rotate_pieces);
                }
            }
            PieceGenSetting::MILLER => {
                for (i, piece) in pieces.iter().enumerate() {
                    let rotate_pieces = Shape::from_str(piece).all_miller(i);
                    field.append_piece(rotate_pieces);
                }
            }
            PieceGenSetting::ALL => {
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

    pub fn find_empty(&self) -> Option<Point<usize>> {
        // find empty place in field.field
        // return value is coordinate
        let field = &self.field;
        for (i, column) in field.iter().enumerate() {
            for (j, elem) in column.iter().enumerate() {
                if let FieldElement::NONE = elem {
                    let p = Point { x: i, y: j };
                    return Some(p);
                }
            }
        }
        None
    }

    pub fn can_place_piece_at_point(&self, piece: &Piece, p: &Point<i32>) -> bool {
        // not immplemented
        // whether it can place the piece at the point in the field.
        true
    }
    pub fn place_piece(&mut self, piece: &Piece, p: &Point<i32>) {
        // not immplemented
        // it should be done after can_place_piece.
    }

    pub fn get_piece(&mut self) -> Piece {
        let piece = self.pieces.pop().unwrap();
        piece
    }
}
impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ID(n) => write!(f, "{}", n),
            Self::NONE => write!(f, "*"),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //
        let mut s: String = String::new();
        for line in &self.field {
            for id in line {
                s = s + &id.to_string();
            }
            s = s + &"\n".to_string();
        }
        write!(f, "{}", s)
    }
}
