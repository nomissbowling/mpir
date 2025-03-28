//! add
//!

use std::ops::{Add, AddAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{typ::*, mpz::*};

onforward_ref_binop!{impl Add, add for mpz_s, mpz_s, mpz_s}

/// impl Add for mpz_r
impl<'a, 'b> Add<&'b mpz_s> for &'a mpz_s {
  type Output = <mpz_s as Add<mpz_s>>::Output;

  /// add mpz_r + mpz_r
  #[inline]
  fn add(self, rhs: &'b mpz_s) -> <mpz_s as Add<mpz_s>>::Output {
    let mut t = mpz_s::init();
    mpz_add(&mut t, self, rhs);
    t
  }
}

onforward_ref_binop!{impl Add, add for mpz_s, ui_t, mpz_s}

/// impl Add for mpz_r
impl<'a, 'b> Add<&'b ui_t> for &'a mpz_s {
  type Output = <mpz_s as Add<ui_t>>::Output;

  /// add mpz_r + &ui_t
  #[inline]
  fn add(self, rhs: &'b ui_t) -> <mpz_s as Add<ui_t>>::Output {
    let mut t = mpz_s::init();
    mpz_add_ui(&mut t, self, *rhs);
    t
  }
}

onforward_ref_binop!{impl Add, add for ui_t, mpz_s, mpz_s}

/// impl Add for ui_t
impl<'a, 'b> Add<&'b mpz_s> for &'a ui_t {
  type Output = <mpz_s as Add<mpz_s>>::Output;

  /// add &ui_t + mpz_r
  #[inline]
  fn add(self, rhs: &'b mpz_s) -> <mpz_s as Add<mpz_s>>::Output {
    let mut t = mpz_s::init();
    mpz_add_ui(&mut t, rhs, *self);
    t
  }
}

onforward_ref_op_assign!{impl AddAssign, add_assign for mpz_s, mpz_s}

/// impl AddAssign for mpz_s
impl<'a> AddAssign<&'a Self> for mpz_s {
  /// add_assign mpz_s += mpz_r
  #[inline]
  fn add_assign(&mut self, rhs: &'a Self) -> () {
    self.add(rhs);
  }
}

onforward_ref_op_assign!{impl AddAssign, add_assign for mpz_s, ui_t}

/// impl AddAssign for mpz_s
impl<'a> AddAssign<&'a ui_t> for mpz_s {
  /// addassign mpz_s += &ui_t
  #[inline]
  fn add_assign(&mut self, rhs: &'a ui_t) -> () {
    self.add_ui(*rhs);
  }
}
