use rusttype::Point;
pub struct Piece {
    shape: Vec<Vec<bool>>,
    used: bool,
    id: i32,
}

pub impl Piece {
    pub fn From(s: &str) -> Piece {}
    pub fn rotate(&self) -> Piece {}
    pub fn all_rotate(&self) -> Vec<Piece> {}
    pub fn miller(&self) -> Piece {}
    pub fn translation(&self, p: &Point) -> Piece {}
}
