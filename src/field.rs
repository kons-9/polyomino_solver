use piece::Piece;
use std::fmt;

struct Field {
    field: Vec<Vec<i32>>,
    pieces: Vec<piece>,
}

pub impl Field {
    fn find_empty(&self) -> Point(i32, i32) {
        // field.field内部のemptyを探す
        // 返り値は座標
        let field = &self.field;
    }
    fn can_place_piece(&self, piece: &Piece, p: &Point) -> bool {}

    fn piece_can_add(&mut self, piece: &Piece, p: &Point) -> bool {
        // 指定された座標にpieceが入るのかどうか
    }
    fn place_piece(&mut self, piece: &Piece, p: &Point) {}

    fn get_piece(&mut self) -> Piece {
        let piece = self.piece.shape.pop().unwrap();
        piece
    }
}

pub impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
