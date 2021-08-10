mod field;
mod piece;

pub use crate::field::Field;
pub use crate::field::FieldElement;
pub use crate::field::FieldInput;
pub use crate::field::PieceSetGenSetting;
pub use crate::piece::Piece;

pub fn solve_poly(
    row: usize,
    column: usize,
    pieces: Vec<&str>,
    setting: PieceSetGenSetting,
) -> usize {
    let mut field = Field::make_field(FieldInput::RECT(column, row), pieces, setting);
    field.run()
}

#[cfg(test)]
mod test {
    #[test]
    fn run_test() {
        let pieces = vec!["11\n10\n10", "01\n11\n01", "010\n111\n010", "101\n111"];
        let n = super::solve_poly(3, 6, pieces, super::PieceSetGenSetting::ALL);
        assert_eq!(n, 1);
    }
}
