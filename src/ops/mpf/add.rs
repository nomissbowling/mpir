//! add
//!

use std::ops::{Add, AddAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{typ::*, mpf::*};

onforward_ref_binop!{impl Add, add for mpf_s, mpf_s, mpf_s}

/// impl Add for mpf_r
impl<'a, 'b> Add<&'b mpf_s> for &'a mpf_s {
  type Output = <mpf_s as Add<mpf_s>>::Output;

  /// add mpf_r + mpf_r
  #[inline]
  fn add(self, rhs: &'b mpf_s) -> <mpf_s as Add<mpf_s>>::Output {
    let mut t = mpf_s::init();
    mpf_add(&mut t, self, rhs);
    t
  }
}

onforward_ref_binop!{impl Add, add for mpf_s, ui_t, mpf_s}

/// impl Add for mpf_r
impl<'a, 'b> Add<&'b ui_t> for &'a mpf_s {
  type Output = <mpf_s as Add<ui_t>>::Output;

  /// add mpf_r + &ui_t
  #[inline]
  fn add(self, rhs: &'b ui_t) -> <mpf_s as Add<ui_t>>::Output {
    let mut t = mpf_s::init();
    mpf_add_ui(&mut t, self, *rhs);
    t
  }
}

onforward_ref_binop!{impl Add, add for ui_t, mpf_s, mpf_s}

/// impl Add for ui_t
impl<'a, 'b> Add<&'b mpf_s> for &'a ui_t {
  type Output = <mpf_s as Add<mpf_s>>::Output;

  /// add &ui_t + mpf_r
  #[inline]
  fn add(self, rhs: &'b mpf_s) -> <mpf_s as Add<mpf_s>>::Output {
    let mut t = mpf_s::init();
    mpf_add_ui(&mut t, rhs, *self);
    t
  }
}

onforward_ref_op_assign!{impl AddAssign, add_assign for mpf_s, mpf_s}

/// impl AddAssign for mpf_s
impl<'a> AddAssign<&'a Self> for mpf_s {
  /// add_assign mpf_s += mpf_r
  #[inline]
  fn add_assign(&mut self, rhs: &'a Self) -> () {
    self.add(rhs);
  }
}

onforward_ref_op_assign!{impl AddAssign, add_assign for mpf_s, ui_t}

/// impl AddAssign for mpf_s
impl<'a> AddAssign<&'a ui_t> for mpf_s {
  /// addassign mpf_s += &ui_t
  #[inline]
  fn add_assign(&mut self, rhs: &'a ui_t) -> () {
    self.add_ui(*rhs);
  }
}
