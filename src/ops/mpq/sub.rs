//! sub
//

use std::ops::{Sub, SubAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{mpq::*};

onforward_ref_binop!{impl Sub, sub for mpq_s, mpq_s, mpq_s}

/// impl Sub for mpq_r
impl<'a, 'b> Sub<&'b mpq_s> for &'a mpq_s {
  type Output = <mpq_s as Sub<mpq_s>>::Output;

  /// sub mpq_r - mpq_r
  #[inline]
  fn sub(self, rhs: &'b mpq_s) -> <mpq_s as Sub<mpq_s>>::Output {
    let mut t = mpq_s::init();
    mpq_sub(&mut t, self, rhs);
    t
  }
}

onforward_ref_op_assign!{impl SubAssign, sub_assign for mpq_s, mpq_s}

/// impl SubAssign for mpq_s
impl<'a> SubAssign<&'a Self> for mpq_s {
  /// sub_assign mpq_s -= mpq_r
  #[inline]
  fn sub_assign(&mut self, rhs: &'a Self) -> () {
    self.sub(rhs);
  }
}
