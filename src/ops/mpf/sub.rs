//! sub
//!

use std::ops::{Sub, SubAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{typ::*, mpf::*};

onforward_ref_binop!{impl Sub, sub for mpf_s, mpf_s, mpf_s}

/// impl Sub for mpf_r
impl<'a, 'b> Sub<&'b mpf_s> for &'a mpf_s {
  type Output = <mpf_s as Sub<mpf_s>>::Output;

  /// sub mpf_r - mpf_r
  #[inline]
  fn sub(self, rhs: &'b mpf_s) -> <mpf_s as Sub<mpf_s>>::Output {
    let mut t = mpf_s::init();
    mpf_sub(&mut t, self, rhs);
    t
  }
}

onforward_ref_binop!{impl Sub, sub for mpf_s, ui_t, mpf_s}

/// impl Sub for mpf_r
impl<'a, 'b> Sub<&'b ui_t> for &'a mpf_s {
  type Output = <mpf_s as Sub<ui_t>>::Output;

  /// sub mpf_r - &ui_t
  #[inline]
  fn sub(self, rhs: &'b ui_t) -> <mpf_s as Sub<ui_t>>::Output {
    let mut t = mpf_s::init();
    mpf_sub_ui(&mut t, self, *rhs);
    t
  }
}

onforward_ref_binop!{impl Sub, sub for ui_t, mpf_s, mpf_s}

/// impl Sub for ui_t
impl<'a, 'b> Sub<&'b mpf_s> for &'a ui_t {
  type Output = <mpf_s as Sub<mpf_s>>::Output;

  /// sub &ui_t - mpf_r
  #[inline]
  fn sub(self, rhs: &'b mpf_s) -> <mpf_s as Sub<mpf_s>>::Output {
    let mut t = mpf_s::init();
    mpf_ui_sub(&mut t, *self, rhs);
    t
  }
}

onforward_ref_op_assign!{impl SubAssign, sub_assign for mpf_s, mpf_s}

/// impl SubAssign for mpf_s
impl<'a> SubAssign<&'a Self> for mpf_s {
  /// sub_assign mpf_s -= mpf_r
  #[inline]
  fn sub_assign(&mut self, rhs: &'a Self) -> () {
    self.sub(rhs);
  }
}

onforward_ref_op_assign!{impl SubAssign, sub_assign for mpf_s, ui_t}

/// impl SubAssign for mpf_s
impl<'a> SubAssign<&'a ui_t> for mpf_s {
  /// sub_assign mpf_s -= &ui_t
  #[inline]
  fn sub_assign(&mut self, rhs: &'a ui_t) -> () {
    self.sub_ui(*rhs);
  }
}
