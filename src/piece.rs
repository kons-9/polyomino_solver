use rusttype::Point;

// base on left upper block.
// ex) 001\n111\n => [Point(0,0), Point(-2,1), Point(-1,1), Point(0,1)]

#[derive(Clone, Debug, PartialEq)]
pub struct Piece(pub Vec<Point<i32>>);

#[derive(Clone, Debug)]
pub struct PieceSet {
    // PieceSet includes Pieces which can be regarded as the same.
    pub pieces: Vec<Piece>, // include Rotational symmetry or Line symmetry of piece.
    pub index: usize, // index of pieces. if index equal to len(pieces), this piece is not still used.
    pub used: bool,
    pub id: usize,
    pub coordinate: Point<usize>, // upper left coordinate of piece.
}

impl Piece {
    // To generate piece

    pub fn from_str(s: &str) -> Piece {
        // str must have same row length, have only 0,1,\n and each row and column must have 1.
        // for example 110\n011\n, not 01\n001 , 011\n122\n or 011\n011\n;
        // if not, compiler must be going to panic!

        let split_newline: Vec<&str> = s.split("\n").collect();
        Piece::check_input_length(&split_newline);

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
        Piece::check_input_shapes(&split_all);

        let mut base_x: i32 = -1;
        let mut base_y: i32 = -1;

        let mut points = vec![Point { x: 0, y: 0 }];
        for (column_num, column) in split_all.iter().enumerate() {
            for (row_num, flag) in column.iter().enumerate() {
                if base_x < 0 && *flag {
                    base_x = row_num as i32;
                    base_y = column_num as i32;
                } else if *flag {
                    let x = row_num as i32 - base_x;
                    let y = column_num as i32 - base_y;
                    points.push(Point { x, y });
                }
            }
        }

        Piece(points)
    }

    fn check_input_length(v: &Vec<&str>) {
        let mut leng: Vec<usize> = v
            .iter()
            .map(|&x| x.to_string().len())
            .collect::<Vec<usize>>();
        leng.sort();
        if leng[0] != leng[leng.len() - 1] {
            panic!("Value Error: Piece must be rectangle. So you should input str having the same length of row : {:?}",v);
        }
    }

    fn check_input_shapes(v: &Vec<Vec<bool>>) {
        let mut v = v.clone();
        let mut stack: Vec<Point<usize>> = Vec::new();
        for x in 0..v[0].len() {
            for y in 0..v.len() {
                if v[y][x] {
                    stack.push(Point { x, y });
                    break;
                }
            }
        }
        let direction = vec![
            Point { x: 0, y: 1 },
            Point { x: 0, y: -1 },
            Point { x: 1, y: 0 },
            Point { x: -1, y: 0 },
        ];
        while !stack.is_empty() {
            let p = stack.pop().unwrap();
            v[p.y][p.x] = false;

            for q in &direction {
                let nowx = p.x as i32 + q.x;
                let nowy = p.y as i32 + q.y;
                if !nowx.is_negative()
                    && !nowy.is_negative()
                    && (nowx as usize) < v[0].len()
                    && (nowy as usize) < v.len()
                    && v[nowy as usize][nowx as usize]
                {
                    stack.push(Point {
                        x: nowx as usize,
                        y: nowy as usize,
                    })
                }
            }
        }
        for i in &v {
            if i.contains(&true) {
                panic!(
                    "ValueError: there are two or more objects in the same piece. {:?}",
                    v
                )
            }
        }
    }

    // To generate PieceSet
    // ownership is going to move.
    pub fn points_normalize(self) -> Piece {
        let mut points = self.0;
        points.sort_by(|a, b| {
            // first cmp y, second cmp x.
            if a.y == b.y {
                a.x.cmp(&b.x)
            } else {
                a.y.cmp(&b.y)
            }
        });

        let base = Point {
            x: points[0].x,
            y: points[0].y,
        };
        for p in &mut points {
            p.x = &p.x - base.x;
            p.y = &p.y - base.y;
        }
        Piece(points)
    }

    pub fn rotate(&self) -> Piece {
        let mut rotate_piece: Vec<Point<i32>> = Vec::new();
        for p in &self.0 {
            rotate_piece.push(Point { x: -p.y, y: p.x });
        }
        Piece(rotate_piece).points_normalize()
    }

    fn miller(&self) -> Piece {
        let mut miller_piece: Vec<Point<i32>> = Vec::new();
        for p in &self.0 {
            miller_piece.push(Point { x: -p.x, y: p.y });
        }
        Piece(miller_piece).points_normalize()
    }
    fn remove_symmetry(pieces: Vec<Piece>) -> Vec<Piece> {
        let mut removed_pieces: Vec<Piece> = Vec::new();
        for piece in pieces {
            if !removed_pieces.contains(&piece) {
                removed_pieces.push(piece);
            }
        }
        removed_pieces
    }

    pub fn all_rotate(self, id: usize) -> PieceSet {
        let mut pieces: Vec<Piece> = Vec::new();
        let mut rotate = self.rotate();
        let mut tmp;
        for _ in 0..3 {
            tmp = rotate.rotate();
            pieces.push(rotate);
            rotate = tmp;
        }
        pieces.push(rotate);
        let pieces = Piece::remove_symmetry(pieces);
        PieceSet::new(pieces, id)
    }
    pub fn all_miller(self, id: usize) -> PieceSet {
        let mut pieces: Vec<Piece> = Vec::new();
        pieces.push(self.miller());
        pieces.push(self);

        let pieces = Piece::remove_symmetry(pieces);
        PieceSet::new(pieces, id)
    }
    pub fn all_rotate_and_miller(self, id: usize) -> PieceSet {
        let miller = self.all_miller(id);
        let mut pieces: Vec<Piece> = Vec::new();
        for sps in miller.pieces {
            let mut rotate = sps.all_rotate(id);
            pieces.append(&mut rotate.pieces);
        }

        let pieces = Piece::remove_symmetry(pieces);
        PieceSet::new(pieces, id)
    }
}

impl PieceSet {
    pub fn new(pieces: Vec<Piece>, id: usize) -> PieceSet {
        let index = pieces.len();
        let coordinate = Point { x: 0, y: 0 };
        let used = false;
        PieceSet {
            pieces,
            index,
            used,
            id,
            coordinate,
        }
    }
    // PieceSet -> PieceSet
    pub fn translation(self, p: Point<usize>) -> PieceSet {
        PieceSet {
            pieces: self.pieces,
            index: self.index,
            used: false,
            id: self.id,
            coordinate: Point {
                x: p.x + self.coordinate.x,
                y: p.y + self.coordinate.y,
            },
        }
    }

    pub fn add_piece(mut self, other: &mut PieceSet) -> PieceSet {
        // index is initialized.
        self.pieces.append(&mut other.pieces);
        self.index = self.pieces.len();
        self
    }
}
