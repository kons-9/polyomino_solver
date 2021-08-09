use rusttype::Point;

pub struct Shape(Vec<Vec<bool>>);

pub struct Piece {
    // Piece includes Shapes which can be regarded as the same.
    shapes: Vec<Shape>, // include Rotational symmetry or Line symmetry of shape.
    index: usize, // index of shapes. if index equal to len(shapes), this piece is not still used.
    id: usize,
    coordinate: Point<usize>, // upper left coordinate of piece.
}

impl Shape {
    // To generate shape

    pub fn from_str(s: &str) -> Shape {
        // str must have same row length, have only 0,1,\n and each row and column must have 1.
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
                    .map(|x| x == &'1')
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();
        Shape::check_input_row_0(&split_all);
        Shape::check_input_column_0(&split_all);

        Shape(split_all)
    }
    pub fn check_input_length(v: &Vec<&str>) {
        let mut leng: Vec<usize> = v
            .iter()
            .map(|&x| x.to_string().len())
            .collect::<Vec<usize>>();
        leng.sort();
        if leng[0] != leng[leng.len() - 1] {
            panic!("Value Error: Shape must be rectangle. So you should input str having the same length of row : {:?}",v);
        }
    }
    pub fn check_input_row_0(v: &Vec<Vec<bool>>) {
        // not immplemented
        if true {
            panic!("ValueError: Shape must be optimize. So you should input str each of row must have 1.")
        }
    }
    pub fn check_input_column_0(v: &Vec<Vec<bool>>) {
        // not immplemented
        if true {
            panic!("ValueError: Shape must be optimize. So you should input str each of column must have 1.")
        }
    }

    // To generate Piece
    // ownership is going to move.
    fn rotate(&self) -> Shape {
        // not implemented
        let tmp: Vec<Vec<bool>> = Vec::new();
        Shape(tmp)
    }
    fn miller<'a>(&self) -> Shape {
        let mut miller_shape: Vec<Vec<bool>> = Vec::new();
        for column in &self.0 {
            let mut miller = column.clone();
            miller.reverse();
            miller_shape.push(miller);
        }
        Shape(miller_shape)
    }

    pub fn all_rotate(self, id: usize) -> Piece {
        let mut shapes: Vec<Shape> = Vec::new();
        let mut rotate = self.rotate();
        let mut tmp;
        for _ in 0..3 {
            tmp = rotate.rotate();
            shapes.push(rotate);
            rotate = tmp;
        }
        shapes.push(rotate);
        Piece::new(shapes, id)
    }
    pub fn all_miller(self, id: usize) -> Piece {
        let mut shapes: Vec<Shape> = Vec::new();
        shapes.push(self.miller());
        shapes.push(self);

        Piece::new(shapes, id)
    }
    pub fn all_rotate_and_miller(self, id: usize) -> Piece {
        let miller = self.all_miller(id);
        let mut shapes: Vec<Shape> = Vec::new();
        for sps in miller.shapes {
            let mut rotate = sps.all_rotate(id);
            shapes.append(&mut rotate.shapes);
        }
        Piece::new(shapes, id)
    }
}

impl Piece {
    pub fn new(shapes: Vec<Shape>, id: usize) -> Piece {
        let index = shapes.len();
        let coordinate = Point { x: 0, y: 0 };
        Piece {
            shapes,
            index,
            id,
            coordinate,
        }
    }
    // Piece -> Piece
    pub fn translation(self, p: Point<usize>) -> Piece {
        Piece {
            shapes: self.shapes,
            index: self.index,
            id: self.id,
            coordinate: Point {
                x: p.x + self.coordinate.x,
                y: p.y + self.coordinate.y,
            },
        }
    }

    pub fn add_piece(mut self, other: &mut Piece) -> Piece {
        // index is initialized.
        self.shapes.append(&mut other.shapes);
        self.index = self.shapes.len();
        self
    }
}
