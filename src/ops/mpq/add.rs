//! add
//

use std::ops::{Add, AddAssign};
use crate::ops::{onforward_ref_binop, onforward_ref_op_assign};
use crate::prim::{mpq::*};

onforward_ref_binop!{impl Add, add for mpq_s, mpq_s, mpq_s}

/// impl Add for mpq_r
impl<'a, 'b> Add<&'b mpq_s> for &'a mpq_s {
  type Output = <mpq_s as Add<mpq_s>>::Output;

  /// add mpq_r + mpq_r
  #[inline]
  fn add(self, rhs: &'b mpq_s) -> <mpq_s as Add<mpq_s>>::Output {
    let mut t = mpq_s::init();
    mpq_add(&mut t, self, rhs);
    t
  }
}

onforward_ref_op_assign!{impl AddAssign, add_assign for mpq_s, mpq_s}

/// impl AddAssign for mpq_s
impl<'a> AddAssign<&'a Self> for mpq_s {
  /// add_assign mpq_s += mpq_r
  #[inline]
  fn add_assign(&mut self, rhs: &'a Self) -> () {
    self.add(rhs);
  }
}
