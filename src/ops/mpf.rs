//! mpf
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
use crate::prim::{mpf::*};

onforward_ref_unop!{impl Neg, neg for mpf_s}

/// impl Neg for mpf_r
impl<'a> Neg for &'a mpf_s {
  type Output = <mpf_s as Neg>::Output;

  /// neg mpf_r
  #[inline]
  fn neg(self) -> <mpf_s as Neg>::Output {
    let mut t = mpf_s::init();
    mpf_neg(&mut t, self);
    t
  }
}
