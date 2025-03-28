//! sub
//!

use std::ops::{Sub, SubAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{typ::*, mpz::*};

onforward_ref_binop!{impl Sub, sub for mpz_s, mpz_s, mpz_s}

/// impl Sub for mpz_r
impl<'a, 'b> Sub<&'b mpz_s> for &'a mpz_s {
  type Output = <mpz_s as Sub<mpz_s>>::Output;

  /// sub mpz_r - mpz_r
  #[inline]
  fn sub(self, rhs: &'b mpz_s) -> <mpz_s as Sub<mpz_s>>::Output {
    let mut t = mpz_s::init();
    mpz_sub(&mut t, self, rhs);
    t
  }
}

onforward_ref_binop!{impl Sub, sub for mpz_s, ui_t, mpz_s}

/// impl Sub for mpz_r
impl<'a, 'b> Sub<&'b ui_t> for &'a mpz_s {
  type Output = <mpz_s as Sub<ui_t>>::Output;

  /// sub mpz_r - &ui_t
  #[inline]
  fn sub(self, rhs: &'b ui_t) -> <mpz_s as Sub<ui_t>>::Output {
    let mut t = mpz_s::init();
    mpz_sub_ui(&mut t, self, *rhs);
    t
  }
}

onforward_ref_binop!{impl Sub, sub for ui_t, mpz_s, mpz_s}

/// impl Sub for ui_t
impl<'a, 'b> Sub<&'b mpz_s> for &'a ui_t {
  type Output = <mpz_s as Sub<mpz_s>>::Output;

  /// sub &ui_t - mpz_r
  #[inline]
  fn sub(self, rhs: &'b mpz_s) -> <mpz_s as Sub<mpz_s>>::Output {
    let mut t = mpz_s::init();
    mpz_ui_sub(&mut t, *self, rhs);
    t
  }
}

onforward_ref_op_assign!{impl SubAssign, sub_assign for mpz_s, mpz_s}

/// impl SubAssign for mpz_s
impl<'a> SubAssign<&'a Self> for mpz_s {
  /// sub_assign mpz_s -= mpz_r
  #[inline]
  fn sub_assign(&mut self, rhs: &'a Self) -> () {
    self.sub(rhs);
  }
}

onforward_ref_op_assign!{impl SubAssign, sub_assign for mpz_s, ui_t}

/// impl SubAssign for mpz_s
impl<'a> SubAssign<&'a ui_t> for mpz_s {
  /// sub_assign mpz_s -= &ui_t
  #[inline]
  fn sub_assign(&mut self, rhs: &'a ui_t) -> () {
    self.sub_ui(*rhs);
  }
}
