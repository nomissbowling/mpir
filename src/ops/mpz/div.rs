//! div based on tdiv_q
//!

use std::ops::{Div, DivAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{typ::*, mpz::*};

onforward_ref_binop!{impl Div, div for mpz_s, mpz_s, mpz_s}

/// impl Div for mpz_r
impl<'a, 'b> Div<&'b mpz_s> for &'a mpz_s {
  type Output = <mpz_s as Div<mpz_s>>::Output;

  /// div mpz_r / mpz_r
  #[inline]
  fn div(self, rhs: &'b mpz_s) -> <mpz_s as Div<mpz_s>>::Output {
    let mut t = mpz_s::init();
    mpz_tdiv_q(&mut t, self, rhs);
    t
  }
}

onforward_ref_binop!{impl Div, div for mpz_s, ui_t, mpz_s}

/// impl Div for mpz_r
impl<'a, 'b> Div<&'b ui_t> for &'a mpz_s {
  type Output = <mpz_s as Div<ui_t>>::Output;

  /// div mpz_r / &ui_t
  #[inline]
  fn div(self, rhs: &'b ui_t) -> <mpz_s as Div<ui_t>>::Output {
    let mut t = mpz_s::init();
    mpz_tdiv_q_ui(&mut t, self, *rhs);
    t
  }
}

onforward_ref_op_assign!{impl DivAssign, div_assign for mpz_s, mpz_s}

/// impl DivAssign for mpz_s
impl<'a> DivAssign<&'a Self> for mpz_s {
  /// div_assign mpz_s /= mpz_r
  #[inline]
  fn div_assign(&mut self, rhs: &'a Self) -> () {
    self.set(&self.tdiv_q(rhs));
  }
}

onforward_ref_op_assign!{impl DivAssign, div_assign for mpz_s, ui_t}

/// impl DivAssign for mpz_s
impl<'a> DivAssign<&'a ui_t> for mpz_s {
  /// div_assign mpz_s /= &ui_t
  #[inline]
  fn div_assign(&mut self, rhs: &'a ui_t) -> () {
    self.set(&self.tdiv_q_ui(*rhs).0);
  }
}
