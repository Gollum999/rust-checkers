#[derive(Copy, Clone)]
pub enum Team {
    White,
    Black,
}
#[derive(Copy, Clone)]
pub enum PieceType {
    Man,
    King,
}
#[derive(Copy, Clone)]
pub struct Piece {
    pub team: Team,
    pub piece_type: PieceType,
}

type _Row = [Option<Piece>; 8];
type _Board = [_Row; 8];
pub struct Board(_Board);
// pub type Board = _Board;

impl Board {
    pub const SIZE: i32 = 8;
    pub fn new() -> Board {
        use Team::*;
        let p = |team| Some(Piece{ team: team, piece_type: PieceType::Man });
        Board(
            [
                [    None, p(Black),     None, p(Black),     None, p(Black),     None, p(Black)],
                [p(Black),     None, p(Black),     None, p(Black),     None, p(Black),     None],
                [    None, p(Black),     None, p(Black),     None, p(Black),     None, p(Black)],
                [    None,     None,     None,     None,     None,     None,     None,     None],
                [    None,     None,     None,     None,     None,     None,     None,     None],
                [p(White),     None, p(White),     None, p(White),     None, p(White),     None],
                [    None, p(White),     None, p(White),     None, p(White),     None, p(White)],
                [p(White),     None, p(White),     None, p(White),     None, p(White),     None],
            ]
        )
    }
    // TODO not sure of the best way to expose the array iterator
    pub fn value(&self) -> &_Board {
        &self.0
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
