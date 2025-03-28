//! cmp
//

use std::cmp::{PartialEq, PartialOrd, Ordering};
// use std::cmp::{Eq, Ord}; // float comparision with precision
use crate::prim::{mpf::*};

/// impl PartialEq for mpf_s
impl PartialEq<Self> for mpf_s {
  /// eq mpf_r == mpf_r
  #[inline]
  fn eq(&self, rhs: &Self) -> bool {
    self.cmp(rhs) == 0
  }
}

/// impl PartialOrd for mpf_s
impl PartialOrd<Self> for mpf_s {
  /// partial_cmp mpf_r - mpf_r
  #[inline]
  fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
    let c = self.cmp(rhs);
    if c == 0 { Some(Ordering::Equal) }
    else if c < 0 { Some(Ordering::Less) }
    else if c > 0 { Some(Ordering::Greater) }
    else { None }
  }
}


