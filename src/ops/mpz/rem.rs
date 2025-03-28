//! rem based on tdiv_r
//!

use std::ops::{Rem, RemAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{typ::*, mpz::*};

onforward_ref_binop!{impl Rem, rem for mpz_s, mpz_s, mpz_s}

/// impl Rem for mpz_r
impl<'a, 'b> Rem<&'b mpz_s> for &'a mpz_s {
  type Output = <mpz_s as Rem<mpz_s>>::Output;

  /// rem mpz_r % mpz_r
  #[inline]
  fn rem(self, rhs: &'b mpz_s) -> <mpz_s as Rem<mpz_s>>::Output {
    let mut t = mpz_s::init();
    mpz_tdiv_r(&mut t, self, rhs);
    t
  }
}

onforward_ref_binop!{impl Rem, rem for mpz_s, ui_t, mpz_s}

/// impl Rem for mpz_r
impl<'a, 'b> Rem<&'b ui_t> for &'a mpz_s {
  type Output = <mpz_s as Rem<ui_t>>::Output;

  /// rem mpz_r % &ui_t
  #[inline]
  fn rem(self, rhs: &'b ui_t) -> <mpz_s as Rem<ui_t>>::Output {
    let mut t = mpz_s::init();
    mpz_tdiv_r_ui(&mut t, self, *rhs);
    t
  }
}

onforward_ref_op_assign!{impl RemAssign, rem_assign for mpz_s, mpz_s}

/// impl RemAssign for mpz_s
impl<'a> RemAssign<&'a Self> for mpz_s {
  /// rem_assign mpz_s %= mpz_r
  #[inline]
  fn rem_assign(&mut self, rhs: &'a Self) -> () {
    self.set(&self.tdiv_r(rhs));
  }
}

onforward_ref_op_assign!{impl RemAssign, rem_assign for mpz_s, ui_t}

/// impl RemAssign for mpz_s
impl<'a> RemAssign<&'a ui_t> for mpz_s {
  /// rem_assign mpz_s %= &ui_t
  #[inline]
  fn rem_assign(&mut self, rhs: &'a ui_t) -> () {
    self.set(&self.tdiv_r_ui(*rhs).0);
  }
}
