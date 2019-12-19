#[repr(i16)]
pub enum Color {
    RedOnWhite   = 1,
    WhiteOnRed   = 2,
    RedOnBlack   = 3,
    BlackOnRed   = 4,
    WhiteOnBlack = 5,
    BlackOnWhite = 6,
}

arg_enum! {
    #[derive(Clone, Debug)]
    pub enum ColorScheme {
        WhiteRed,
        RedBlack,
        WhiteBlack,
    }
}

#[derive(Clone, Debug)]
pub struct Args {
    pub ascii: bool,
    pub color_scheme: ColorScheme,
}
