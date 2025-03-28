//! cmp
//

use std::cmp::{PartialEq, PartialOrd, Ordering};
// use std::cmp::{Eq, Ord}; // fract reduction
use crate::prim::{mpq::*};

/// impl PartialEq for mpq_s
impl PartialEq<Self> for mpq_s {
  /// eq mpq_r == mpq_r
  #[inline]
  fn eq(&self, rhs: &Self) -> bool {
    self.cmp(rhs) == 0
  }
}

/// impl PartialOrd for mpq_s
impl PartialOrd<Self> for mpq_s {
  /// partial_cmp mpq_r - mpq_r
  #[inline]
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    let c = self.cmp(rhs);
    if c == 0 { Some(Ordering::Equal) }
    else if c < 0 { Some(Ordering::Less) }
    else if c > 0 { Some(Ordering::Greater) }
    else { None }
  }
}
