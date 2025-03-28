//! mpz
//!

pub mod sub;
pub mod add;
pub mod mul;
pub mod div;
pub mod rem;
pub mod cmp;

use std::ops::{Neg, Not};
use crate::ops::onforward_ref_unop;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
// use std::ops::{Index, IndexMut};
use crate::prim::{typ::*, mpz::*};

onforward_ref_unop!{impl Neg, neg for mpz_s}

/// impl Neg for mpz_r
impl<'a> Neg for &'a mpz_s {
  type Output = <mpz_s as Neg>::Output;

  /// neg mpz_r
  #[inline]
  fn neg(self) -> <mpz_s as Neg>::Output {
    let mut t = mpz_s::init();
    mpz_neg(&mut t, self);
    t
  }
}

onforward_ref_unop!{impl Not, not for mpz_s}

/// impl Not for mpz_r
impl<'a> Not for &'a mpz_s {
  type Output = <mpz_s as Not>::Output;

  /// not mpz_r
  #[inline]
  fn not(self) -> <mpz_s as Not>::Output {
    let mut t = mpz_s::init();
    mpz_com(&mut t, self);
    t
  }
}

onforward_ref_binop!{impl BitAnd, bitand for mpz_s, mpz_s, mpz_s}

/// impl BitAnd for mpz_r
impl<'a, 'b> BitAnd<&'b mpz_s> for &'a mpz_s {
  type Output = <mpz_s as BitAnd<mpz_s>>::Output;

  /// bitand mpz_r &amp; mpz_r
  #[inline]
  fn bitand(self, rhs: &'b mpz_s) -> <mpz_s as BitAnd<mpz_s>>::Output {
    let mut t = mpz_s::init();
    mpz_and(&mut t, self, rhs);
    t
  }
}

onforward_ref_op_assign!{impl BitAndAssign, bitand_assign for mpz_s, mpz_s}

/// impl BitAndAssign for mpz_s
impl<'a> BitAndAssign<&'a Self> for mpz_s {
  /// bitand_assign mpz_s &amp;= mpz_r
  #[inline]
  fn bitand_assign(&mut self, rhs: &'a Self) -> () {
    self.set(&self.and(rhs));
  }
}

onforward_ref_binop!{impl BitOr, bitor for mpz_s, mpz_s, mpz_s}

/// impl BitOr for mpz_r
impl<'a, 'b> BitOr<&'b mpz_s> for &'a mpz_s {
  type Output = <mpz_s as BitOr<mpz_s>>::Output;

  /// bitor mpz_r | mpz_r
  #[inline]
  fn bitor(self, rhs: &'b mpz_s) -> <mpz_s as BitOr<mpz_s>>::Output {
    let mut t = mpz_s::init();
    mpz_ior(&mut t, self, rhs);
    t
  }
}

onforward_ref_op_assign!{impl BitOrAssign, bitor_assign for mpz_s, mpz_s}

/// impl BitOrAssign for mpz_s
impl<'a> BitOrAssign<&'a Self> for mpz_s {
  /// bitor_assign mpz_s |= mpz_r
  #[inline]
  fn bitor_assign(&mut self, rhs: &'a Self) -> () {
    self.set(&self.ior(rhs));
  }
}

onforward_ref_binop!{impl BitXor, bitxor for mpz_s, mpz_s, mpz_s}

/// impl BitXor for mpz_r
impl<'a, 'b> BitXor<&'b mpz_s> for &'a mpz_s {
  type Output = <mpz_s as BitXor<mpz_s>>::Output;

  /// bitxor mpz_r &amp; mpz_r
  #[inline]
  fn bitxor(self, rhs: &'b mpz_s) -> <mpz_s as BitXor<mpz_s>>::Output {
    let mut t = mpz_s::init();
    mpz_xor(&mut t, self, rhs);
    t
  }
}

onforward_ref_op_assign!{impl BitXorAssign, bitxor_assign for mpz_s, mpz_s}

/// impl BitXorAssign for mpz_s
impl<'a> BitXorAssign<&'a Self> for mpz_s {
  /// bitxor_assign mpz_s &amp;= mpz_r
  #[inline]
  fn bitxor_assign(&mut self, rhs: &'a Self) -> () {
    self.set(&self.xor(rhs));
  }
}

onforward_ref_binop!{impl Shl, shl for mpz_s, mp_bitcnt_t, mpz_s}

/// impl Shl for mpz_r
impl<'a, 'b> Shl<&'b mp_bitcnt_t> for &'a mpz_s {
  type Output = <mpz_s as Shl<mp_bitcnt_t>>::Output;

  /// shl mpz_r << mp_bitcnt_t
  #[inline]
  fn shl(self, rhs: &'b mp_bitcnt_t) -> <mpz_s as Shl<mp_bitcnt_t>>::Output {
    let mut t = mpz_s::init();
    mpz_mul_2exp(&mut t, self, *rhs);
    t
  }
}

onforward_ref_op_assign!{impl ShlAssign, shl_assign for mpz_s, mp_bitcnt_t}

/// impl ShlAssign for mpz_s
impl<'a> ShlAssign<&'a mp_bitcnt_t> for mpz_s {
  /// shl_assign mpz_s <<= mp_bitcnt_t
  #[inline]
  fn shl_assign(&mut self, rhs: &'a mp_bitcnt_t) -> () {
    self.mul_2exp(*rhs);
  }
}

onforward_ref_binop!{impl Shr, shr for mpz_s, mp_bitcnt_t, mpz_s}

/// impl Shr for mpz_r
impl<'a, 'b> Shr<&'b mp_bitcnt_t> for &'a mpz_s {
  type Output = <mpz_s as Shr<mp_bitcnt_t>>::Output;

  /// shr mpz_r >> mp_bitcnt_t
  #[inline]
  fn shr(self, rhs: &'b mp_bitcnt_t) -> <mpz_s as Shr<mp_bitcnt_t>>::Output {
    let mut t = mpz_s::init();
    mpz_tdiv_q_2exp(&mut t, self, *rhs);
    t
  }
}

onforward_ref_op_assign!{impl ShrAssign, shr_assign for mpz_s, mp_bitcnt_t}

/// impl ShrAssign for mpz_s
impl<'a> ShrAssign<&'a mp_bitcnt_t> for mpz_s {
  /// shr_assign mpz_s >>= mp_bitcnt_t
  #[inline]
  fn shr_assign(&mut self, rhs: &'a mp_bitcnt_t) -> () {
    self.set(&self.tdiv_q_2exp(*rhs));
  }
}
