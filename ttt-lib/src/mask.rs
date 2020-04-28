#[derive(Debug, Clone, Copy)]
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

impl std::ops::BitOr<usize> for Mask {
  type Output = Self;
  fn bitor(self, rhs: usize) -> Self {
    Self(self.0 | rhs)
  }
}

impl std::ops::BitAnd<usize> for Mask {
  type Output = Self;

  fn bitand(self, rhs: usize) -> Self {
    Self(self.0 & rhs)
  }
}

impl std::ops::BitXor<usize> for Mask {
  type Output = Self;

  fn bitxor(self, rhs: usize) -> Self {
    Self(self.0 ^ rhs)
  }
}

impl std::ops::BitOr for Mask {
  type Output = Self;

  fn bitor(self, rhs: Self) -> Self {
    Self(self.0 | rhs.0)
  }
}

impl std::ops::BitAnd for Mask {
  type Output = Self;

  fn bitand(self, rhs: Self) -> Self {
    Self(self.0 & rhs.0)
  }
}

impl std::ops::BitXor for Mask {
  type Output = Self;

  fn bitxor(self, rhs: Self) -> Self {
    Self(self.0 ^ rhs.0)
  }
}

impl Mask {
  /// set a single bit at index
  ///
  /// ```
  /// # use ttt_lib::mask::Mask;
  ///
  /// let mask = Mask(1);
  ///
  /// assert_eq!(mask.set_bit_at(1), 0b000011)
  /// ```
  pub fn set_bit_at(self, index: usize) -> Self {
    self | 1 << index
  }

  /// set bits within an exclusive range
  ///
  /// ```
  /// # use ttt_lib::mask::Mask;
  ///
  /// let mask = Mask(1);
  ///
  /// assert_eq!(mask.set_bit_range(2, 4), 0b0001101)
  /// ```
  pub fn set_bit_range(self, start: usize, finish: usize) -> Self {
    self | (1 << finish) - (1 << start)
  }

  /// see if one mask is a subset of another
  ///
  /// ```
  /// # use ttt_lib::mask::Mask;
  ///
  /// let mask = Mask(0b1100);
  ///
  /// assert_eq!(mask.has_bits_set(0b100), true)
  /// ```
  pub fn has_bits_set(self, mask: usize) -> bool {
    self.0 & mask == mask
  }

  /// see invert masked bits
  ///
  /// ```
  /// # use ttt_lib::mask::Mask;
  ///
  /// let mask = Mask(0b110010);
  ///
  /// assert_eq!(mask.invert_by_mask(0b101010), 0b011000)
  /// ```
  pub fn invert_by_mask(self, mask: usize) -> Self {
    self ^ mask
  }
}
