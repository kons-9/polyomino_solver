use rusttype::Point;

pub struct Shape(Vec<Vec<bool>>);

pub struct Piece {
    // Piece includes Shapes which can be regarded as the same.
    shapes: Vec<Shape>, // include Rotational symmetry or Line symmetry of shape.
    index: i32, // index of shapes. if index equal to len(shapes), this piece is not still used.
    id: usize,
    coordinate: Point<i32>, // upper left coordinate of piece.
}

impl Shape {
    // To generate shape

    pub fn from_str(s: &str) -> Shape {
        // str must have same row length, have only 0,1,\n and each row and collom must have 1.
        // for example 110\n011\n, not 01\n001 , 011\n122\n or 011\n011\n;
        // if not, compiler must be going to panic!
        let split_newline: Vec<&str> = s.split("\n").collect();
        Shape::check_input_length(&split_newline);

        let split_all = split_newline
            .iter()
            .map(|x| {
                x.chars()
                    .collect::<Vec<char>>()
                    .iter()
                    .map(|x| x == '1')
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();
        Shape::check_input_row_0(&split_all);
        Shape::check_input_collom_0(&split_all);

        Shape(split_all)
    }
    pub fn check_input_length(v: &Vec<&str>) {
        let leng: Vec<i32> = v
            .iter()
            .map(|&x| x.to_string().len())
            .collect::<Vec<i32>>()
            .sort();
        if leng[0] != leng[leng.len() - 1] {
            panic!("Value Error: Shape must be rectangle. So you should input str having the same length of row : {:?}",v);
        }
    }
    pub fn check_input_row_0(v: &Vec<Vec<bool>>) {
        if true {
            panic!("ValueError: Shape must be optimize. So you should input str each of row must have 1.")
        }
    }
    pub fn check_input_collom_0(v: &Vec<Vec<bool>>) {
        if true {
            panic!("ValueError: Shape must be optimize. So you should input str each of collom must have 1.")
        }
    }

    // To generate Piece
    // ownership is going to move.
    fn rotate(&self) -> Shape {}
    pub fn all_rotate(self, id: usize) -> Piece {
        let mut shapes: Vec<Shape> = Vec::new();
        let mut rotate = self.rotate();
        let mut tmp;
        for i in [0..3] {
            tmp = rotate.rotate();
            shapes.push(rotate);
            rotate = tmp;
        }
        shapes.push(rotate);
        Piece {
            shapes: shapes,
            index: 4,
            id: id,
            coordinate: Point { x: 0, y: 0 },
        }
    }
    fn miller(&self) -> Shape {
        &self.into_iter().map(|x| x.reverse()).collect()
    }
    pub fn all_miller(self, id: usize) -> Piece {
        let mut shapes: Vec<Shape> = Vec::new();
        shapes.push(&self.miller());
        shapes.push(self);

        Piece {
            shapes: shapes,
            index: 2,
            id: id,
            coordinate: Point { x: 0, y: 0 },
        }
    }
    pub fn all_rotate_and_miller(self, id: usize) -> Piece {
        self.all_rotate(id).add_piece(&mut self.all_miller(id))
    }
}

impl Piece {
    // Piece -> Piece
    pub fn translation(&self, p: Point<i32>) -> Piece {
        Piece {
            shapes: self.shapes,
            index: self.index,
            id: self.id,
            coordinate: p + self.coordinate,
        }
    }

    pub fn add_piece(self, other: &mut Piece) -> Piece {
        // index is initialized.
        self.shapes.append(&mut other.shapes);
        self.index = self.shapes.len();
        self
    }
}
