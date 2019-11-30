use std::collections::HashMap;
use std::fmt;
use std::ops;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Team {
    Light,
    Dark,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PieceType {
    Man,
    King,
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Piece {
    pub team: Team,
    pub piece_type: PieceType,
}
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct Square {
    pub x: i8,
    pub y: i8,
}
impl ops::Add<(i8, i8)> for &Square {
    type Output = Square;
    fn add(self, rhs: (i8, i8)) -> Self::Output {
        Square{ x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} to {}", self.from, self.to)
    }
}
impl Move {
    pub fn is_jump(&self) -> bool {
        (self.from.x - self.to.x).abs() > 1
    }
}

type _Row = [Option<Piece>; 8];
type _Board = [_Row; 8];
#[derive(Clone)]
pub struct Board {
    pieces: HashMap<Square, Piece>,
}
// pub type Board = _Board;

impl Board {
    pub const SIZE: i8 = 8;
    pub fn new() -> Board {
        use Team::*;
        let mut pieces = HashMap::new();
        let mut add_piece = |x, y, team| {
            pieces.insert(Square{x, y}, Piece{ team: team, piece_type: PieceType::Man });
        };

        add_piece(1, 0, Dark);
        add_piece(3, 0, Dark);
        add_piece(5, 0, Dark);
        add_piece(7, 0, Dark);
        add_piece(0, 1, Dark);
        add_piece(2, 1, Dark);
        add_piece(4, 1, Dark);
        add_piece(6, 1, Dark);
        add_piece(1, 2, Dark);
        add_piece(3, 2, Dark);
        add_piece(5, 2, Dark);
        add_piece(7, 2, Dark);

        add_piece(0, 5, Light);
        add_piece(2, 5, Light);
        add_piece(4, 5, Light);
        add_piece(6, 5, Light);
        add_piece(1, 6, Light);
        add_piece(3, 6, Light);
        add_piece(5, 6, Light);
        add_piece(7, 6, Light);
        add_piece(0, 7, Light);
        add_piece(2, 7, Light);
        add_piece(4, 7, Light);
        add_piece(6, 7, Light);

        Board { pieces: pieces }
    }

    fn find_piece_square(&self, piece: &Piece) -> &Square {
        for (square, other_piece) in &self.pieces {
            if piece == other_piece {
                return &square;
            }
        }
        panic!("Could not find square for piece {:?}", piece);
    }

    fn square_occupied(&self, s: &Square) -> bool {
        self.pieces.contains_key(s)
    }

    fn in_bounds(s: &Square) -> bool {
        0 <= s.x && s.x < Self::SIZE && 0 <= s.y && s.y < Self::SIZE
    }

    fn _can_step(&self, from: &Square, to: &Square, distance: i8) -> bool {
        let piece = match self.pieces.get(from) {
            Some(x) => x,
            None => return false,
        };
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let correct_direction = piece.piece_type == PieceType::King
            || (piece.team == Team::Light && dy <= -1)
            || (piece.team == Team::Dark && dy >= 1);
        let correct_distance = dx.abs() == distance && dy.abs() == distance;

        // println!("Piece: {:?}, dx: {} dy: {}, correct_dir: {}, correct_distance: {}, occupied: {}, in bounds: {}",
        //          piece, dx, dy, correct_direction, correct_distance, self.square_occupied(to), Self::in_bounds(to));
        !self.square_occupied(to) && correct_direction && correct_distance && Self::in_bounds(to)
    }

    fn can_step(&self, from: &Square, to: &Square) -> bool {
        self._can_step(from, to, 1)
    }

    fn can_jump(&self, from: &Square, to: &Square) -> bool {
        let piece = match self.pieces.get(from) { // TODO would `if let` be cleaner?
            Some(x) => x,
            None => return false,
        };
        let between = Square{ x: (from.x + to.x) / 2, y: (from.y + to.y) / 2 };
        let between_piece = match self.pieces.get(&between) {
            Some(x) => x,
            None => return false,
        };

        // println!("JUMP? Piece: {:?}, between: {}, between piece: {:?}", piece, between, between_piece);
        self._can_step(from, to, 2) && piece.team != between_piece.team
    }

    fn can_move(&self, from: &Square, to: &Square) -> bool {
        self.can_step(from, to) || self.can_jump(from, to)
    }

    pub fn get_valid_moves_for_piece(&self, piece: &Piece) -> Vec<Move> {
        let square = self.find_piece_square(&piece);

        self.get_valid_moves_for_piece_at(square).unwrap()
    }

    pub fn get_valid_moves_for_piece_at(&self, square: &Square) -> Result<Vec<Move>, String> {
        let mut moves = Vec::new();

        for dx in &[-1, 1] {
            for dy in &[-1, 1] {
                let to = square + (*dx, *dy);
                if self.can_move(square, &to) {
                    moves.push(Move{ from: *square, to: to });
                }
            }
        }
        for dx in &[-2, 2] {
            for dy in &[-2, 2] {
                let to = square + (*dx, *dy);
                if self.can_jump(square, &to) {
                    moves.push(Move{ from: *square, to: to });
                }
            }
        }

        Ok(moves)
    }

    pub fn get_all_valid_moves(&self, team: Team) -> Vec<Move> {
        let mut moves = Vec::new();
        for square in self.pieces.keys() {
            match self.pieces.get(square) {
                Some(p) if p.team == team => {
                    moves.append(&mut self.get_valid_moves_for_piece_at(square).unwrap());
                },
                _ => (),
            }
        }

        moves
    }

    pub fn get_pieces(&self) -> &HashMap<Square, Piece> {
        &self.pieces
    }
    // TODO not sure of the best way to expose the array iterator
    // pub fn value(&self) -> &_Board {
    //     &self.0
    // }

    pub fn apply_move(&mut self, m: &Move) {
        let mut piece = self.pieces.remove(&m.from).unwrap();

        // Jump
        if m.is_jump() {
            let between = Square{ x: (m.from.x + m.to.x) / 2, y: (m.from.y + m.to.y) / 2 };
            self.pieces.remove(&between);
        }

        // Promotion
        if (piece.team == Team::Light && m.to.y == 0)
        || (piece.team == Team::Dark && m.to.y == Self::SIZE - 1) {
            piece.piece_type = PieceType::King;
        }

        self.pieces.insert(m.to, piece);
    }
}

// impl IntoIterator for Board {
//     type Item = _Row;
//     type IntoIter = ::IntoIter<Self::Item>;

//     fn into_iter(self) {
//         self.0.into_iter()
//     }
// }
// struct BoardIter {
//     iter: std::iter::Iter<_Board>,
// }
// impl std::iter::Iterator for BoardIter {
//     type Item = _Row;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.iter()
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_bounds() {
        assert!( Board::in_bounds(&Square{ x:  0, y:  0 }));
        assert!( Board::in_bounds(&Square{ x:  0, y:  7 }));
        assert!( Board::in_bounds(&Square{ x:  7, y:  0 }));
        assert!( Board::in_bounds(&Square{ x:  7, y:  7 }));

        assert!(!Board::in_bounds(&Square{ x: -1, y:  0 }));
        assert!(!Board::in_bounds(&Square{ x:  0, y: -1 }));
        assert!(!Board::in_bounds(&Square{ x: -1, y: -1 }));
        assert!(!Board::in_bounds(&Square{ x:  0, y:  8 }));
        assert!(!Board::in_bounds(&Square{ x:  8, y:  0 }));
        assert!(!Board::in_bounds(&Square{ x:  8, y:  8 }));
    }

    #[test]
    fn test_square_occupied() {
        let board = Board::new();
        assert!( board.square_occupied(&Square{ x:  0, y:  1 }));
        assert!( board.square_occupied(&Square{ x:  1, y:  0 }));
        assert!( board.square_occupied(&Square{ x:  6, y:  7 }));
        assert!( board.square_occupied(&Square{ x:  7, y:  6 }));

        assert!(!board.square_occupied(&Square{ x:  0, y:  0 }));
        assert!(!board.square_occupied(&Square{ x:  4, y:  4 }));
        assert!(!board.square_occupied(&Square{ x:  5, y:  5 }));
        assert!(!board.square_occupied(&Square{ x:  7, y:  7 }));

        assert!(!board.square_occupied(&Square{ x: -1, y: -1 }));
        assert!(!board.square_occupied(&Square{ x:  8, y:  8 }));
    }

    #[test]
    fn test_can_step() {
        let mut board = Board::new();

        assert!( board.can_step(&Square{ x:  1, y:  2 }, &Square{ x:  2, y:  3 }));
        assert!( board.can_step(&Square{ x:  1, y:  2 }, &Square{ x:  0, y:  3 }));
        assert!( board.can_step(&Square{ x:  3, y:  2 }, &Square{ x:  2, y:  3 }));
        assert!( board.can_step(&Square{ x:  3, y:  2 }, &Square{ x:  4, y:  3 }));
        assert!( board.can_step(&Square{ x:  0, y:  5 }, &Square{ x:  1, y:  4 }));
        assert!( board.can_step(&Square{ x:  2, y:  5 }, &Square{ x:  1, y:  4 }));
        assert!( board.can_step(&Square{ x:  2, y:  5 }, &Square{ x:  3, y:  4 }));

        // No piece at `from`
        assert!(!board.can_step(&Square{ x:  0, y:  0 }, &Square{ x:  1, y:  1 }));
        assert!(!board.can_step(&Square{ x:  0, y:  2 }, &Square{ x:  1, y:  3 }));
        assert!(!board.can_step(&Square{ x:  2, y:  0 }, &Square{ x:  3, y:  1 }));
        assert!(!board.can_step(&Square{ x:  7, y:  7 }, &Square{ x:  6, y:  6 }));
        assert!(!board.can_step(&Square{ x: -1, y: -1 }, &Square{ x:  0, y:  0 }));
        assert!(!board.can_step(&Square{ x:  8, y:  8 }, &Square{ x:  7, y:  7 }));

        // `to` is occupied
        assert!(!board.can_step(&Square{ x:  1, y:  0 }, &Square{ x:  2, y:  1 }));
        assert!(!board.can_step(&Square{ x:  0, y:  1 }, &Square{ x:  1, y:  2 }));
        assert!(!board.can_step(&Square{ x:  0, y:  7 }, &Square{ x:  1, y:  6 }));
        assert!(!board.can_step(&Square{ x:  1, y:  6 }, &Square{ x:  0, y:  5 }));

        // Wrong distance
        assert!(!board.can_step(&Square{ x:  3, y:  2 }, &Square{ x:  0, y:  3 }));
        assert!(!board.can_step(&Square{ x:  3, y:  2 }, &Square{ x:  1, y:  3 }));
        assert!(!board.can_step(&Square{ x:  3, y:  2 }, &Square{ x:  3, y:  3 }));
        assert!(!board.can_step(&Square{ x:  3, y:  2 }, &Square{ x:  0, y:  4 }));
        assert!(!board.can_step(&Square{ x:  3, y:  2 }, &Square{ x:  1, y:  4 }));
        assert!(!board.can_step(&Square{ x:  3, y:  2 }, &Square{ x:  2, y:  4 }));
        assert!(!board.can_step(&Square{ x:  3, y:  2 }, &Square{ x:  3, y:  4 }));

        // Out of bounds
        assert!(!board.can_step(&Square{ x:  7, y:  0 }, &Square{ x:  8, y:  1 }));
        assert!(!board.can_step(&Square{ x:  0, y:  1 }, &Square{ x:  8, y:  2 }));
        assert!(!board.can_step(&Square{ x:  7, y:  2 }, &Square{ x:  8, y:  3 }));

        // Wrong direction
        board.apply_move(&Move{ from: Square{ x: 1, y: 2 }, to: Square{ x: 2, y: 3 }});
        assert!(!board.can_step(&Square{ x: 2, y: 3 }, &Square{ x: 1, y: 2 }));
        // TODO
    }

    #[test]
    fn test_can_jump() {
        let mut pieces = HashMap::new();

        pieces.insert(Square{x: 4, y: 4}, Piece{ team: Team::Dark, piece_type: PieceType::King });
        let mut add_piece = |x, y, team| {
            pieces.insert(Square{x, y}, Piece{ team, piece_type: PieceType::Man });
        };
                                      //   2 3 4 5
        add_piece(3, 5, Team::Dark);  // 2 W
        add_piece(3, 3, Team::Light); // 3   W   W
        add_piece(2, 2, Team::Light); // 4     B
        add_piece(5, 3, Team::Light); // 5   B

        let board = Board {pieces: pieces};

        assert!(!board.can_jump(&Square{ x: 4, y: 4 }, &Square{ x: 2, y: 2 })); // Space occupied
        assert!(!board.can_jump(&Square{ x: 4, y: 4 }, &Square{ x: 2, y: 6 })); // Can't jump over own piece
        assert!( board.can_jump(&Square{ x: 4, y: 4 }, &Square{ x: 6, y: 2 }));
        assert!(!board.can_jump(&Square{ x: 4, y: 4 }, &Square{ x: 6, y: 6 })); // No piece to jump over
    }
}
