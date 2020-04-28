use crate::mask::Mask;

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
  /// let board = Board(Mask(0b00011111), Mask(0b11111000));
  /// assert_eq!(board.is_full(), true);
  /// ```
  pub fn is_full(self) -> bool {
    let Self(x, o) = self;
    (x | o).has_bits_set(0b11111111)
  }
}
