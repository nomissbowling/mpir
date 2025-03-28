//! mul
//!

use std::ops::{Mul, MulAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{typ::*, mpf::*};

onforward_ref_binop!{impl Mul, mul for mpf_s, mpf_s, mpf_s}

/// impl Mul for mpf_r
impl<'a, 'b> Mul<&'b mpf_s> for &'a mpf_s {
  type Output = <mpf_s as Mul<mpf_s>>::Output;

  /// mul mpf_r * mpf_r
  #[inline]
  fn mul(self, rhs: &'b mpf_s) -> <mpf_s as Mul<mpf_s>>::Output {
    let mut t = mpf_s::init();
    mpf_mul(&mut t, self, rhs);
    t
  }
}

onforward_ref_binop!{impl Mul, mul for mpf_s, ui_t, mpf_s}

/// impl Mul for mpf_r
impl<'a, 'b> Mul<&'b ui_t> for &'a mpf_s {
  type Output = <mpf_s as Mul<ui_t>>::Output;

  /// mul mpf_r * &ui_t
  #[inline]
  fn mul(self, rhs: &'b ui_t) -> <mpf_s as Mul<ui_t>>::Output {
    let mut t = mpf_s::init();
    mpf_mul_ui(&mut t, self, *rhs);
    t
  }
}

onforward_ref_binop!{impl Mul, mul for ui_t, mpf_s, mpf_s}

/// impl Mul for ui_t
impl<'a, 'b> Mul<&'b mpf_s> for &'a ui_t {
  type Output = <mpf_s as Mul<mpf_s>>::Output;

  /// mul &ui_t * mpf_r
  #[inline]
  fn mul(self, rhs: &'b mpf_s) -> <mpf_s as Mul<mpf_s>>::Output {
    let mut t = mpf_s::init();
    mpf_mul_ui(&mut t, rhs, *self);
    t
  }
}

onforward_ref_op_assign!{impl MulAssign, mul_assign for mpf_s, mpf_s}

/// impl MulAssign for mpf_s
impl<'a> MulAssign<&'a Self> for mpf_s {
  /// mul_assign mpf_s *= mpf_r
  #[inline]
  fn mul_assign(&mut self, rhs: &'a Self) -> () {
    self.mul(rhs);
  }
}

onforward_ref_op_assign!{impl MulAssign, mul_assign for mpf_s, ui_t}

/// impl MulAssign for mpf_s
impl<'a> MulAssign<&'a ui_t> for mpf_s {
  /// mul_assign mpf_s *= &ui_t
  #[inline]
  fn mul_assign(&mut self, rhs: &'a ui_t) -> () {
    self.mul_ui(*rhs);
  }
}
