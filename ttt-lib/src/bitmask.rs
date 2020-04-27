#[derive(Debug)]
pub struct Mask(pub usize);

impl std::cmp::PartialEq<usize> for Mask {
  fn eq(&self, other: &usize) -> bool {
    self.0 == *other
  }
}

impl std::fmt::Display for Mask {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{0:b}", self.0)
  }
}

impl Mask {
  /// set a single bit at index
  ///
  /// ```
  /// # use ttt_lib::bitmask::Mask;
  ///
  /// let mask = Mask(1);
  ///
  /// assert_eq!(mask.set_bit_at(1), 0b000011)
  /// ```
  pub fn set_bit_at(mut self, index: usize) -> Self {
    self.0 = self.0 | 1 << index;
    self
  }

  /// set bits within an exclusive range
  ///
  /// ```
  /// # use ttt_lib::bitmask::Mask;
  ///
  /// let mask = Mask(1);
  ///
  /// assert_eq!(mask.set_bit_range(2, 4), 0b0001101)
  /// ```
  pub fn set_bit_range(mut self, start: usize, finish: usize) -> Self {
    self.0 = self.0 | (1 << finish) - (1 << start);
    self
  }

  /// see if one mask is a subset of another
  ///
  /// ```
  /// # use ttt_lib::bitmask::Mask;
  ///
  /// let mask = Mask(0b1100);
  ///
  /// assert_eq!(mask.has_bits_set(0b100), true)
  /// ```
  pub fn has_bits_set(&self, mask: usize) -> bool {
    self.0 & mask == mask
  }

  /// see invert masked bits
  ///
  /// ```
  /// # use ttt_lib::bitmask::Mask;
  ///
  /// let mask = Mask(0b110010);
  ///
  /// assert_eq!(mask.invert_by_mask(0b101010), 0b011000)
  /// ```
  pub fn invert_by_mask(mut self, mask: usize) -> Self {
    self.0 = self.0 ^ mask;
    self
  }
}
