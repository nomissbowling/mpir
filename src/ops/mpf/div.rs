//! div
//!

use std::ops::{Div, DivAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{typ::*, mpf::*};

onforward_ref_binop!{impl Div, div for mpf_s, mpf_s, mpf_s}

/// impl Div for mpf_r
impl<'a, 'b> Div<&'b mpf_s> for &'a mpf_s {
  type Output = <mpf_s as Div<mpf_s>>::Output;

  /// div mpf_r / mpf_r
  #[inline]
  fn div(self, rhs: &'b mpf_s) -> <mpf_s as Div<mpf_s>>::Output {
    let mut t = mpf_s::init();
    mpf_div(&mut t, self, rhs);
    t
  }
}

onforward_ref_binop!{impl Div, div for mpf_s, ui_t, mpf_s}

/// impl Div for mpf_r
impl<'a, 'b> Div<&'b ui_t> for &'a mpf_s {
  type Output = <mpf_s as Div<ui_t>>::Output;

  /// div mpf_r / &ui_t
  #[inline]
  fn div(self, rhs: &'b ui_t) -> <mpf_s as Div<ui_t>>::Output {
    let mut t = mpf_s::init();
    mpf_div_ui(&mut t, self, *rhs);
    t
  }
}

onforward_ref_binop!{impl Div, div for ui_t, mpf_s, mpf_s}

/// impl Div for ui_t
impl<'a, 'b> Div<&'b mpf_s> for &'a ui_t {
  type Output = <mpf_s as Div<mpf_s>>::Output;

  /// div &ui_t / mpf_r
  #[inline]
  fn div(self, rhs: &'b mpf_s) -> <mpf_s as Div<mpf_s>>::Output {
    let mut t = mpf_s::init();
    mpf_ui_div(&mut t, *self, rhs);
    t
  }
}

onforward_ref_op_assign!{impl DivAssign, div_assign for mpf_s, mpf_s}

/// impl DivAssign for mpf_s
impl<'a> DivAssign<&'a Self> for mpf_s {
  /// div_assign mpf_s /= mpf_r
  #[inline]
  fn div_assign(&mut self, rhs: &'a Self) -> () {
    self.div(rhs);
  }
}

onforward_ref_op_assign!{impl DivAssign, div_assign for mpf_s, ui_t}

/// impl DivAssign for mpf_s
impl<'a> DivAssign<&'a ui_t> for mpf_s {
  /// div_assign mpf_s /= &ui_t
  #[inline]
  fn div_assign(&mut self, rhs: &'a ui_t) -> () {
    self.div_ui(*rhs);
  }
}
