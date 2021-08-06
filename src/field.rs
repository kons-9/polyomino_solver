use piece::Piece;
use std::fmt;

pub struct Field {
    field: Vec<Vec<i32>>,
    pieces: Vec<Piece>,
}

pub impl Field {
    pub fn make_field(field: &str, pieces: [&str]) {
        let mut field = Field::new(field);

        for piece in pieces {
            let rotate_pieces = Piece::From(piece).all_rotate();
            field.append_pieces(rotate_pieces);
        }
        field
    }
    pub fn append_pieces(&mut self, pieces: Piece) {
        self.pieces.append(pieces);
    }

    pub fn find_empty(&self) -> Point(i32, i32) {
        // field.field内部のemptyを探す
        // 返り値は座標
        let field = &self.field;
    }
    pub fn can_place_piece(&self, piece: &Piece, p: &Point) -> bool {}

    pub fn piece_can_add(&mut self, piece: &Piece, p: &Point) -> bool {
        // 指定された座標にpieceが入るのかどうか
    }
    pub fn place_piece(&mut self, piece: &Piece, p: &Point) {}

    pub fn get_piece(&mut self) -> Piece {
        let piece = self.piece.shape.pop().unwrap();
        piece
    }
}

pub impl fmt::Display for Field {
    pub fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //
        let s::String = String::new();
        for line in self.field {
            for id in line {
                s = s + &id.to_string();
            }
            s = s + "\n".to_string();
        }
        write!(f, "{}", s)
    }
}
