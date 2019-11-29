use std::collections::HashMap;
use std::fmt;
use std::ops;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Team {
    White,
    Black,
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
#[derive(Copy, Clone)]
pub struct Move {
    pub from: Square,
    pub to: Square,
}
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} to {}", self.from, self.to)
    }
}

type _Row = [Option<Piece>; 8];
type _Board = [_Row; 8];
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

        add_piece(1, 0, Black);
        add_piece(3, 0, Black);
        add_piece(5, 0, Black);
        add_piece(7, 0, Black);
        add_piece(0, 1, Black);
        add_piece(2, 1, Black);
        add_piece(4, 1, Black);
        add_piece(6, 1, Black);
        add_piece(1, 2, Black);
        add_piece(3, 2, Black);
        add_piece(5, 2, Black);
        add_piece(7, 2, Black);

        add_piece(0, 5, White);
        add_piece(2, 5, White);
        add_piece(4, 5, White);
        add_piece(6, 5, White);
        add_piece(1, 6, White);
        add_piece(3, 6, White);
        add_piece(5, 6, White);
        add_piece(7, 6, White);
        add_piece(0, 7, White);
        add_piece(2, 7, White);
        add_piece(4, 7, White);
        add_piece(6, 7, White);

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

    fn in_bounds(&self, s: &Square) -> bool {
        0 <= s.x && s.x <= Self::SIZE && 0 <= s.y && s.y <= Self::SIZE
    }

    fn _can_step(&self, from: &Square, to: &Square, distance: i8) -> bool {
        let piece = match self.pieces.get(from) {
            Some(x) => x,
            None => return false,
        };
        let dx = from.x - to.x;
        let dy = from.y - to.y;
        let correct_direction = piece.piece_type == PieceType::King
            || (piece.team == Team::White && dy <= -1)
            || (piece.team == Team::Black && dy >= 1);
        let correct_distance = dx.abs() == distance && dy.abs() == distance;

        !self.square_occupied(to) && correct_direction && correct_distance && self.in_bounds(to)
    }

    fn can_step(&self, from: &Square, to: &Square) -> bool {
        self._can_step(from, to, 1)
    }

    fn can_jump(&self, from: &Square, to: &Square) -> bool {
        let piece = match self.pieces.get(from) { // TODO would `if let` be cleaner?
            Some(x) => x,
            None => return false,
        };
        let between = Square{ x: from.x + to.x / 2, y: from.y + to.y / 2 };
        let between_piece = match self.pieces.get(&between) {
            Some(x) => x,
            None => return false,
        };

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

    pub fn get_all_valid_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for square in self.pieces.keys() {
            moves.append(&mut self.get_valid_moves_for_piece_at(square).unwrap());
        }

        moves
    }

    // TODO not sure of the best way to expose the array iterator
    // pub fn value(&self) -> &_Board {
    //     &self.0
    // }
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
