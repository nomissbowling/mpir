//! div
//!

use std::ops::{Div, DivAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{mpq::*};

onforward_ref_binop!{impl Div, div for mpq_s, mpq_s, mpq_s}

/// impl Div for mpq_r
impl<'a, 'b> Div<&'b mpq_s> for &'a mpq_s {
  type Output = <mpq_s as Div<mpq_s>>::Output;

  /// div mpq_r / mpq_r
  #[inline]
  fn div(self, rhs: &'b mpq_s) -> <mpq_s as Div<mpq_s>>::Output {
    let mut t = mpq_s::init();
    mpq_div(&mut t, self, rhs);
    t
  }
}

onforward_ref_op_assign!{impl DivAssign, div_assign for mpq_s, mpq_s}

/// impl DivAssign for mpq_s
impl<'a> DivAssign<&'a Self> for mpq_s {
  /// div_assign mpq_s /= mpq_r
  #[inline]
  fn div_assign(&mut self, rhs: &'a Self) -> () {
    self.div(rhs);
  }
}
