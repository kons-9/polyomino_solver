use crate::piece::Piece;
use crate::piece::PieceSet;
use rusttype::Point;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FieldElement {
    ID(usize),
    NONE,
    WALL,
}

pub enum FieldInput<'a> {
    RECT(usize, usize),
    STR(&'a str),
}
#[derive(Debug)]
pub struct Field {
    pub field: Vec<FieldElement>,
    pub row: usize,
    pub column: usize,
    pub piecesets: Vec<PieceSet>,
}

pub enum PieceSetGenSetting {
    ROTATE,
    MILLER,
    ALL, // rotate and miller
}

impl Field {
    fn check_input_length(v: &Vec<&str>) {
        let mut leng: Vec<usize> = v
            .iter()
            .map(|&x| x.to_string().len())
            .collect::<Vec<usize>>();
        leng.sort();
        if leng[0] != leng[leng.len() - 1] {
            panic!("Value Error: field must be rectangle. So you should input str having the same length of row : {:?}",v);
        }
    }
    pub fn new(field: FieldInput) -> Field {
        match field {
            FieldInput::RECT(column, row) => {
                let field: Vec<FieldElement> = [FieldElement::NONE].repeat(row * column);
                let piecesets: Vec<PieceSet> = Vec::new();
                Field {
                    field,
                    piecesets,
                    row,
                    column,
                }
            }
            FieldInput::STR(s) => {
                let split_newline: Vec<&str> = s.split("\n").collect();
                Field::check_input_length(&split_newline);
                let row = split_newline[0].len();
                let column = split_newline.len();
                let field = split_newline
                    .concat()
                    .chars()
                    .collect::<Vec<char>>()
                    .iter()
                    .map(|x| {
                        if x == &'*' {
                            FieldElement::WALL
                        } else {
                            FieldElement::NONE
                        }
                    })
                    .collect::<Vec<FieldElement>>();
                let piecesets: Vec<PieceSet> = Vec::new();
                Field {
                    field,
                    piecesets,
                    row,
                    column,
                }
            }
        }
    }
    pub fn make_field(
        field: FieldInput,
        piecesets: Vec<&str>,
        filed_type: PieceSetGenSetting,
    ) -> Field {
        let mut field = Field::new(field);
        match filed_type {
            PieceSetGenSetting::ROTATE => {
                for (i, piece) in piecesets.iter().enumerate() {
                    if i == 0 {
                        let piece = Piece::from_str(piece);
                        let rpiece = piece.rotate().points_normalize();
                        field.append_piece(PieceSet::new(vec![piece, rpiece], i));
                    } else {
                        let rotate_piecesets = Piece::from_str(piece).all_rotate(i);
                        field.append_piece(rotate_piecesets);
                    }
                }
            }
            PieceSetGenSetting::MILLER => {
                for (i, piece) in piecesets.iter().enumerate() {
                    if i == 0 {
                        let piece = Piece::from_str(piece);
                        field.append_piece(PieceSet::new(vec![piece], i));
                    } else {
                        let rotate_piecesets = Piece::from_str(piece).all_miller(i);
                        field.append_piece(rotate_piecesets);
                    }
                }
            }
            PieceSetGenSetting::ALL => {
                for (i, piece) in piecesets.iter().enumerate() {
                    if i == 0 {
                        let piece = Piece::from_str(piece);
                        let rpiece = piece.rotate();
                        field.append_piece(PieceSet::new(vec![piece, rpiece], i));
                    } else {
                        let rotate_piecesets = Piece::from_str(piece).all_rotate_and_miller(i);
                        field.append_piece(rotate_piecesets);
                    }
                }
            }
        }
        field
    }

    pub fn run(&mut self) -> usize {
        let p = self.find_empty();
        if let None = p {
            println!("{}", self);
            // println!("{:?}", self);
            // println!("{}", self.field.len());
            return 1;
        }
        let p = p.unwrap();
        let mut cnt: usize = 0;

        let piecesets = &self.piecesets.clone();
        for (i, pieceset) in piecesets.iter().enumerate() {
            if pieceset.used {
                continue;
            }
            self.piecesets[i].used = true;
            let inds = self.find_index_can_place_at_point(pieceset, &p);
            match inds {
                None => {
                    self.piecesets[i].used = false;
                    continue;
                }
                Some(indexs) => {
                    for index in indexs {
                        self.place_piece(pieceset, index, &p);
                        cnt += self.run();
                        self.remove_piece(pieceset, index, &p);
                    }
                }
            }
            self.piecesets[i].used = false;
        }
        // println!("runend");
        return cnt;
    }

    fn append_piece(&mut self, piece: PieceSet) {
        self.piecesets.push(piece);
    }

    pub fn find_empty(&self) -> Option<Point<usize>> {
        // find empty place in field.field
        // return value is coordinate
        let field = &self.field;
        for (i, elem) in field.iter().enumerate() {
            if let FieldElement::NONE = elem {
                let x = i % self.row;
                let y = (i - x) / self.row;
                let p = Point { x, y };
                return Some(p);
            }
        }
        None
    }
    fn can_place_at_point(&self, piece: &Piece, p: &Point<usize>) -> bool {
        // check most left and most right block is inside the field
        for point in &piece.0 {
            let x = point.x + p.x as i32;
            let y = point.y + p.y as i32;
            if x < 0 || (x as usize) >= self.row || (y as usize) >= self.column {
                return false;
            }
            let x = x as usize;
            let y = y as usize;
            let ind = self.row * y + x;

            if self.field[ind] != FieldElement::NONE {
                return false;
            }
        }
        true
    }

    pub fn find_index_can_place_at_point(
        &self,
        pieceset: &PieceSet,
        p: &Point<usize>,
    ) -> Option<Vec<usize>> {
        // whether it can place the piece at the point in the field.
        // return value is cannot=>None, can=>Some(vector of index in which list can place)

        let mut ret: Vec<usize> = Vec::new();
        for (i, piece) in pieceset.pieces.iter().enumerate() {
            let flag = self.can_place_at_point(piece, p);
            if flag == true {
                ret.push(i);
            }
        }
        if ret.len() == 0 {
            None
        } else {
            Some(ret)
        }
    }

    // pub fn place_piece(&mut self, pieceset_ind: &mut PieceSet, index: usize, p: &Point<usize>) {
    pub fn place_piece(&mut self, pieceset: &PieceSet, index: usize, p: &Point<usize>) {
        // it should be done after can_place_piece.
        let piece = &pieceset.pieces[index];
        for point in &piece.0 {
            let x = (p.x as i32 + point.x) as usize;
            let y = (p.y as i32 + point.y) as usize;
            // point to index. it is a bit complicated because field is 1D array.
            let ind = self.row * y + x;
            self.field[ind] = FieldElement::ID(pieceset.id);
        }
    }
    pub fn remove_piece(&mut self, pieceset: &PieceSet, index: usize, p: &Point<usize>) {
        // it must be done after place_piece, otherwise compiler is going to panic.
        let piece = &pieceset.pieces[index];
        for point in &piece.0 {
            let x = (p.x as i32 + point.x) as usize;
            let y = (p.y as i32 + point.y) as usize;
            // point to index. it is a bit complicated because field is 1D array.
            let ind = self.row * y + x;
            self.field[ind] = FieldElement::NONE;
        }
    }

    pub fn get_piece(&mut self) -> PieceSet {
        let piece = self.piecesets.pop().unwrap();
        piece
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ID(n) => {
                write!(f, "{}", std::char::from_digit(*n as u32, 36).unwrap())
            }
            Self::NONE => write!(f, "N"),
            Self::WALL => write!(f, "*"),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //self.field is 1-D vec, so it required a bit complicated process to display.
        let mut s: String = String::new();
        // let hash = std::collections::HashMap::new();

        for i in 0..self.column {
            for j in 0..self.row {
                s = s + &(self.field[i * self.row + j]).to_string();
            }
            s = s + &"\n".to_string();
        }
        write!(f, "{}", s)
    }
}
