mod field;
mod piece;

pub use crate::field::Field;
pub use crate::piece::Piece;

fn solve_poly(field: &str, pieces: [&str]) {
    let field = Field::make_field(field, pieces);
    rec_solve(&field);
}
