//! mpq
//!

pub mod sub;
pub mod add;
pub mod mul;
pub mod div;
pub mod rem;
pub mod cmp;

use std::ops::{Neg};
use crate::ops::onforward_ref_unop;
// use std::ops::{Index, IndexMut};
use crate::prim::{mpq::*};

onforward_ref_unop!{impl Neg, neg for mpq_s}

/// impl Neg for mpq_r
impl<'a> Neg for &'a mpq_s {
  type Output = <mpq_s as Neg>::Output;

  /// neg mpq_r
  #[inline]
  fn neg(self) -> <mpq_s as Neg>::Output {
    let mut t = mpq_s::init();
    mpq_neg(&mut t, self);
    t
  }
}
