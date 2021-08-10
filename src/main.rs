extern crate lib;

use lib::solve_poly;
use lib::PieceSetGenSetting;

fn main() {
    let pieces = vec![
        "1110\n0011\n0010",
        "1111\n1010",
        "11110\n00011",
        "11111\n00010",
        "1110\n1011",
        "11111\n00001",
        "111\n001\n011",
        "0100\n1111\n0100",
        "1110\n0111",
        "011\n110\n011",
        "111\n110\n100",
    ];
    let n = solve_poly(11, 6, pieces, PieceSetGenSetting::ROTATE);
    println!("count -> {}", n);
}
