//! cmp
//!

use std::cmp::{PartialEq, PartialOrd, Ordering};
use crate::prim::{mpz::*};

/// impl PartialEq for mpz_s
impl PartialEq<Self> for mpz_s {
  /// eq mpz_r == mpz_r
  #[inline]
  fn eq(&self, rhs: &Self) -> bool {
    self.cmp(rhs) == 0
  }
}

/// impl PartialOrd for mpz_s
impl PartialOrd<Self> for mpz_s {
  /// partial_cmp mpz_r - mpz_r
  #[inline]
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    let c = self.cmp(rhs);
    if c == 0 { Some(Ordering::Equal) }
    else if c < 0 { Some(Ordering::Less) }
    else if c > 0 { Some(Ordering::Greater) }
    else { None }
  }
}
