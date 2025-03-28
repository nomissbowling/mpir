//! mul
//!

use std::ops::{Mul, MulAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{typ::*, mpz::*};

onforward_ref_binop!{impl Mul, mul for mpz_s, mpz_s, mpz_s}

/// impl Mul for mpz_r
impl<'a, 'b> Mul<&'b mpz_s> for &'a mpz_s {
  type Output = <mpz_s as Mul<mpz_s>>::Output;

  /// mul mpz_r * mpz_r
  #[inline]
  fn mul(self, rhs: &'b mpz_s) -> <mpz_s as Mul<mpz_s>>::Output {
    let mut t = mpz_s::init();
    mpz_mul(&mut t, self, rhs);
    t
  }
}

onforward_ref_binop!{impl Mul, mul for mpz_s, ui_t, mpz_s}

/// impl Mul for mpz_r
impl<'a, 'b> Mul<&'b ui_t> for &'a mpz_s {
  type Output = <mpz_s as Mul<ui_t>>::Output;

  /// mul mpz_r * &ui_t
  #[inline]
  fn mul(self, rhs: &'b ui_t) -> <mpz_s as Mul<ui_t>>::Output {
    let mut t = mpz_s::init();
    mpz_mul_ui(&mut t, self, *rhs);
    t
  }
}

onforward_ref_binop!{impl Mul, mul for ui_t, mpz_s, mpz_s}

/// impl Mul for ui_t
impl<'a, 'b> Mul<&'b mpz_s> for &'a ui_t {
  type Output = <mpz_s as Mul<mpz_s>>::Output;

  /// mul &ui_t * mpz_r
  #[inline]
  fn mul(self, rhs: &'b mpz_s) -> <mpz_s as Mul<mpz_s>>::Output {
    let mut t = mpz_s::init();
    mpz_mul_ui(&mut t, rhs, *self);
    t
  }
}

onforward_ref_binop!{impl Mul, mul for mpz_s, si_t, mpz_s}

/// impl Mul for mpz_r
impl<'a, 'b> Mul<&'b si_t> for &'a mpz_s {
  type Output = <mpz_s as Mul<si_t>>::Output;

  /// mul mpz_r * &si_t
  #[inline]
  fn mul(self, rhs: &'b si_t) -> <mpz_s as Mul<si_t>>::Output {
    let mut t = mpz_s::init();
    mpz_mul_si(&mut t, self, *rhs);
    t
  }
}

onforward_ref_binop!{impl Mul, mul for si_t, mpz_s, mpz_s}

/// impl Mul for si_t
impl<'a, 'b> Mul<&'b mpz_s> for &'a si_t {
  type Output = <mpz_s as Mul<mpz_s>>::Output;

  /// mul &si_t * mpz_r
  #[inline]
  fn mul(self, rhs: &'b mpz_s) -> <mpz_s as Mul<mpz_s>>::Output {
    let mut t = mpz_s::init();
    mpz_mul_si(&mut t, rhs, *self);
    t
  }
}

onforward_ref_op_assign!{impl MulAssign, mul_assign for mpz_s, mpz_s}

/// impl MulAssign for mpz_s
impl<'a> MulAssign<&'a Self> for mpz_s {
  /// mul_assign mpz_s *= mpz_r
  #[inline]
  fn mul_assign(&mut self, rhs: &'a Self) -> () {
    self.mul(rhs);
  }
}

onforward_ref_op_assign!{impl MulAssign, mul_assign for mpz_s, ui_t}

/// impl MulAssign for mpz_s
impl<'a> MulAssign<&'a ui_t> for mpz_s {
  /// mul_assign mpz_s *= &ui_t
  #[inline]
  fn mul_assign(&mut self, rhs: &'a ui_t) -> () {
    self.mul_ui(*rhs);
  }
}

onforward_ref_op_assign!{impl MulAssign, mul_assign for mpz_s, si_t}

/// impl MulAssign for mpz_s
impl<'a> MulAssign<&'a si_t> for mpz_s {
  /// mul_assign mpz_s *= &si_t
  #[inline]
  fn mul_assign(&mut self, rhs: &'a si_t) -> () {
    self.mul_si(*rhs);
  }
}
