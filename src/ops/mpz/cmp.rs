//! cmp
//!

use std::cmp::{PartialEq, PartialOrd, Ordering};
// use std::cmp::{Eq, Ord};
use crate::prim::{typ::*, mpz::*};

use impl_ops_cmp::*;

impl_ops_cmp!{impl PartialEq, PartialOrd, cmp for mpz_s, mpz_s, 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_d for mpz_s, double_t, 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_ui for mpz_s, ui_t, 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_si for mpz_s, si_t, 1}
