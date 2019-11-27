//○●◯◖◗⬤⭗⭕⭘🔴🔵
//♛♕♔♚👑
pub enum Piece {
    MAN,
    KING,
}
type _Row = [Option<Piece>; 8];
type _Board = [_Row; 8];
pub struct Board(_Board);
// pub type Board = _Board;

impl Board {
    pub fn new() -> Board {
        use Piece::*;
        Board(
            [
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, Some(MAN), None],
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
