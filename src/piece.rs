struct Piece {
    shape: Vec<Vec<bool>>,
    used: bool,
    id: i32,
}

impl Piece {
    fn rotate(&self) -> Piece {}
    fn miller(&self) -> Piece {}
    fn move(&self,p: &Point) -> Piece {}

}
