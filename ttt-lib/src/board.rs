use crate::mask::Mask;

static GOALS: [usize; 8] = [
  0b111_000_000,
  0b000_111_000,
  0b000_000_111,
  0b100_100_100,
  0b010_010_010,
  0b001_001_001,
  0b100_010_001,
  0b001_010_100,
];

#[derive(Debug, Eq, PartialEq)]
pub enum GameResult {
  XWIN,
  YWIN,
  DRAW,
}

#[derive(Default)]
pub struct Board(pub Mask, pub Mask);

impl Board {
  /// create a new board intialized to zero zero
  pub fn new() -> Self {
    Board(Mask(0), Mask(0))
  }

  /// plays the position for the x player
  ///
  /// ```
  /// # use ttt_lib::board::Board;
  /// let board = Board::new().play_x(1);
  ///
  ///
  /// let Board(x, _) = board;
  /// assert_eq!(x, 0b10);
  /// ```
  pub fn play_x(self, position: usize) -> Self {
    let Self(x, y) = self;
    Self(x.set_bit_at(position), y)
  }

  /// plays the position for the y player
  ///
  /// ```
  /// # use ttt_lib::board::Board;
  /// let board = Board::new().play_y(1);
  ///
  ///
  /// let Board(_, y) = board;
  /// assert_eq!(y, 0b10);
  /// ```
  pub fn play_y(self, position: usize) -> Self {
    let Self(x, y) = self;
    Self(x, y.set_bit_at(position))
  }

  /// check if the gameboard is full
  ///
  /// ```
  /// # use ttt_lib::board::Board;
  /// # use ttt_lib::mask::Mask;
  /// let mut board = Board::new();
  ///
  ///
  /// let board = Board(Mask(0b100_110_010), Mask(0b011_001_101));
  /// assert_eq!(board.is_full(), true);
  /// ```
  pub fn is_full(&self) -> bool {
    let Self(x, o) = self;
    (*x | *o).has_bits_set(0b111_111_111)
  }

  /// check if either player has won
  ///
  /// ```
  /// # use ttt_lib::board::{Board, GameResult};
  /// # use ttt_lib::mask::Mask;
  /// let mut board = Board::new();
  ///
  ///
  /// let board = Board(Mask(0b000_111_000), Mask(0b110_000_100));
  /// assert_eq!(board.get_result(), Some(GameResult::XWIN));
  /// ```
  pub fn get_result(&self) -> Option<GameResult> {
    let Self(x, o) = self;
    for mask in GOALS.iter() {
      if x.has_bits_set(*mask) {
        return Some(GameResult::XWIN);
      };
      if o.has_bits_set(*mask) {
        return Some(GameResult::YWIN);
      };

      if self.is_full() {
        return Some(GameResult::DRAW);
      }
    }

    None
  }
}
