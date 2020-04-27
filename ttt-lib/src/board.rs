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
  /// let mut board = Board::new();
  ///
  /// board.play_x(1);
  ///
  /// let Board(x, _) = board;
  /// assert_eq!(x, 0b10);
  /// ```
  pub fn play_x(&mut self, position: usize) {
    self.0 = self.0.set_bit_at(position);
  }

  /// plays the position for the x player
  ///
  /// ```
  /// # use ttt_lib::board::Board;
  /// let mut board = Board::new();
  ///
  /// board.play_y(1);
  ///
  /// let Board(_, y) = board;
  /// assert_eq!(y, 0b10);
  /// ```
  pub fn play_y(&mut self, position: usize) {
    self.1 = self.1.set_bit_at(position);
  }
}
