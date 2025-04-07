//! ops
//!

pub mod mpz;
pub mod mpf;
pub mod mpq;

pub use crate::ops::mpz::{*, sub::*, add::*, mul::*, div::*, rem::*, cmp::*};
pub use crate::ops::mpf::{*, sub::*, add::*, mul::*, div::*, rem::*, cmp::*};
pub use crate::ops::mpq::{*, sub::*, add::*, mul::*, div::*, rem::*, cmp::*};

pub use onforward_ref::*;

use std::ops::{Mul, MulAssign, Div, DivAssign};
use crate::prim::{mpz::*, mpf::*, mpq::*};

onforward_ref_binop!{impl Mul, mul for mpz_s, mpf_s, mpf_s}

/// impl Mul for mpz_r
impl<'a, 'b> Mul<&'b mpf_s> for &'a mpz_s {
  type Output = <mpf_s as Mul<mpf_s>>::Output;

  /// mul mpz_r * mpf_r
  #[inline]
  fn mul(self, rhs: &'b mpf_s) -> <mpf_s as Mul<mpf_s>>::Output {
    mpf_s::from(self) * rhs
  }
}

onforward_ref_binop!{impl Mul, mul for mpf_s, mpz_s, mpf_s}

/// impl Mul for mpf_r
impl<'a, 'b> Mul<&'b mpz_s> for &'a mpf_s {
  type Output = <mpf_s as Mul<mpz_s>>::Output;

  /// mul mpf_r * mpz_r
  #[inline]
  fn mul(self, rhs: &'b mpz_s) -> <mpf_s as Mul<mpz_s>>::Output {
    self * mpf_s::from(rhs)
  }
}

onforward_ref_op_assign!{impl MulAssign, mul_assign for mpf_s, mpz_s}

/// impl MulAssign for mpf_s
impl<'a> MulAssign<&'a mpz_s> for mpf_s {
  /// mul_assign mpf_s *= mpz_r
  #[inline]
  fn mul_assign(&mut self, rhs: &'a mpz_s) -> () {
    *self *= mpf_s::from(rhs);
  }
}

onforward_ref_binop!{impl Div, div for mpz_s, mpf_s, mpf_s}

/// impl Div for mpz_r
impl<'a, 'b> Div<&'b mpf_s> for &'a mpz_s {
  type Output = <mpf_s as Div<mpf_s>>::Output;

  /// div mpz_r / mpf_r
  #[inline]
  fn div(self, rhs: &'b mpf_s) -> <mpf_s as Div<mpf_s>>::Output {
    mpf_s::from(self) / rhs
  }
}

onforward_ref_binop!{impl Div, div for mpf_s, mpz_s, mpf_s}

/// impl Div for mpf_r
impl<'a, 'b> Div<&'b mpz_s> for &'a mpf_s {
  type Output = <mpf_s as Div<mpz_s>>::Output;

  /// div mpf_r / mpz_r
  #[inline]
  fn div(self, rhs: &'b mpz_s) -> <mpf_s as Div<mpz_s>>::Output {
    self / mpf_s::from(rhs)
  }
}

onforward_ref_op_assign!{impl DivAssign, div_assign for mpf_s, mpz_s}

/// impl DivAssign for mpf_s
impl<'a> DivAssign<&'a mpz_s> for mpf_s {
  /// div_assign mpf_s /= mpz_r
  #[inline]
  fn div_assign(&mut self, rhs: &'a mpz_s) -> () {
    *self /= mpf_s::from(rhs);
  }
}

onforward_ref_binop!{impl Mul, mul for mpz_s, mpq_s, mpq_s}

/// impl Mul for mpz_r
impl<'a, 'b> Mul<&'b mpq_s> for &'a mpz_s {
  type Output = <mpq_s as Mul<mpq_s>>::Output;

  /// mul mpz_r * mpq_r
  #[inline]
  fn mul(self, rhs: &'b mpq_s) -> <mpq_s as Mul<mpq_s>>::Output {
//    mpq_s::frac(&(self * &rhs.get_num()), &rhs.get_den())
    mpq_s::from(self) * rhs
  }
}

onforward_ref_binop!{impl Mul, mul for mpq_s, mpz_s, mpq_s}

/// impl Mul for mpq_r
impl<'a, 'b> Mul<&'b mpz_s> for &'a mpq_s {
  type Output = <mpq_s as Mul<mpz_s>>::Output;

  /// mul mpq_r * mpz_r
  #[inline]
  fn mul(self, rhs: &'b mpz_s) -> <mpq_s as Mul<mpz_s>>::Output {
//    mpq_s::frac(&(&self.get_num() * rhs), &self.get_den())
    self * mpq_s::from(rhs)
  }
}

onforward_ref_op_assign!{impl MulAssign, mul_assign for mpq_s, mpz_s}

/// impl MulAssign for mpq_s
impl<'a> MulAssign<&'a mpz_s> for mpq_s {
  /// mul_assign mpq_s *= mpz_r
  #[inline]
  fn mul_assign(&mut self, rhs: &'a mpz_s) -> () {
//    self.set_num(&(&self.get_num() * rhs));
    *self *= mpq_s::from(rhs);
  }
}

onforward_ref_binop!{impl Div, div for mpz_s, mpq_s, mpq_s}

/// impl Div for mpz_r
impl<'a, 'b> Div<&'b mpq_s> for &'a mpz_s {
  type Output = <mpq_s as Div<mpq_s>>::Output;

  /// div mpz_r / mpq_r
  #[inline]
  fn div(self, rhs: &'b mpq_s) -> <mpq_s as Div<mpq_s>>::Output {
//    mpq_s::frac(&(self * &rhs.get_den()), &rhs.get_num())
    mpq_s::from(self) / rhs
  }
}

onforward_ref_binop!{impl Div, div for mpq_s, mpz_s, mpq_s}

/// impl Div for mpq_r
impl<'a, 'b> Div<&'b mpz_s> for &'a mpq_s {
  type Output = <mpq_s as Div<mpz_s>>::Output;

  /// div mpq_r / mpz_r
  #[inline]
  fn div(self, rhs: &'b mpz_s) -> <mpq_s as Div<mpz_s>>::Output {
//    mpq_s::frac(&self.get_num(), &(&self.get_den() * rhs))
    self / mpq_s::from(rhs)
  }
}

onforward_ref_op_assign!{impl DivAssign, div_assign for mpq_s, mpz_s}

/// impl DivAssign for mpq_s
impl<'a> DivAssign<&'a mpz_s> for mpq_s {
  /// div_assign mpq_s /= mpz_r
  #[inline]
  fn div_assign(&mut self, rhs: &'a mpz_s) -> () {
//    self.set_den(&(&self.get_den() * rhs));
    *self /= mpq_s::from(rhs);
  }
}

onforward_ref_binop!{impl Mul, mul for mpq_s, mpf_s, mpf_s}

/// impl Mul for mpq_r
impl<'a, 'b> Mul<&'b mpf_s> for &'a mpq_s {
  type Output = <mpf_s as Mul<mpf_s>>::Output;

  /// mul mpq_r * mpf_r
  #[inline]
  fn mul(self, rhs: &'b mpf_s) -> <mpf_s as Mul<mpf_s>>::Output {
//    mpf_s::from(self) * rhs
    self.numref() * rhs / self.denref()
  }
}

onforward_ref_binop!{impl Mul, mul for mpf_s, mpq_s, mpf_s}

/// impl Mul for mpf_r
impl<'a, 'b> Mul<&'b mpq_s> for &'a mpf_s {
  type Output = <mpf_s as Mul<mpq_s>>::Output;

  /// mul mpf_r * mpq_r
  #[inline]
  fn mul(self, rhs: &'b mpq_s) -> <mpf_s as Mul<mpq_s>>::Output {
//    self * mpf_s::from(rhs)
    self * rhs.numref() / rhs.denref()
  }
}

onforward_ref_op_assign!{impl MulAssign, mul_assign for mpf_s, mpq_s}

/// impl MulAssign for mpf_s
impl<'a> MulAssign<&'a mpq_s> for mpf_s {
  /// mul_assign mpf_s *= mpq_r
  #[inline]
  fn mul_assign(&mut self, rhs: &'a mpq_s) -> () {
//    *self *= mpf_s::from(rhs);
    self.set(&(&*self * rhs)); // first mul last div
  }
}

onforward_ref_binop!{impl Div, div for mpq_s, mpf_s, mpf_s}

/// impl Div for mpq_r
impl<'a, 'b> Div<&'b mpf_s> for &'a mpq_s {
  type Output = <mpf_s as Div<mpf_s>>::Output;

  /// div mpq_r / mpf_r
  #[inline]
  fn div(self, rhs: &'b mpf_s) -> <mpf_s as Div<mpf_s>>::Output {
//    mpf_s::from(self) / rhs
    self.numref() / (self.denref() * rhs)
  }
}

onforward_ref_binop!{impl Div, div for mpf_s, mpq_s, mpf_s}

/// impl Div for mpf_r
impl<'a, 'b> Div<&'b mpq_s> for &'a mpf_s {
  type Output = <mpf_s as Div<mpq_s>>::Output;

  /// div mpf_r / mpq_r
  #[inline]
  fn div(self, rhs: &'b mpq_s) -> <mpf_s as Div<mpq_s>>::Output {
//    self / mpf_s::from(rhs)
    self * rhs.denref() / rhs.numref()
  }
}

onforward_ref_op_assign!{impl DivAssign, div_assign for mpf_s, mpq_s}

/// impl DivAssign for mpf_s
impl<'a> DivAssign<&'a mpq_s> for mpf_s {
  /// div_assign mpf_s /= mpq_r
  #[inline]
  fn div_assign(&mut self, rhs: &'a mpq_s) -> () {
//    *self /= mpf_s::from(rhs);
    self.set(&(&*self / rhs)); // first mul last div
  }
}
