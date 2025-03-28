//! mul
//!

use std::ops::{Mul, MulAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{mpq::*};

onforward_ref_binop!{impl Mul, mul for mpq_s, mpq_s, mpq_s}

/// impl Mul for mpq_r
impl<'a, 'b> Mul<&'b mpq_s> for &'a mpq_s {
  type Output = <mpq_s as Mul<mpq_s>>::Output;

  /// mul mpq_r * mpq_r
  #[inline]
  fn mul(self, rhs: &'b mpq_s) -> <mpq_s as Mul<mpq_s>>::Output {
    let mut t = mpq_s::init();
    mpq_mul(&mut t, self, rhs);
    t
  }
}

onforward_ref_op_assign!{impl MulAssign, mul_assign for mpq_s, mpq_s}

/// impl MulAssign for mpq_s
impl<'a> MulAssign<&'a Self> for mpq_s {
  /// mul_assign mpq_s *= mpq_r
  #[inline]
  fn mul_assign(&mut self, rhs: &'a Self) -> () {
    self.mul(rhs);
  }
}
