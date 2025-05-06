//! cmp
//

use std::cmp::{PartialEq, PartialOrd, Ordering};
// use std::cmp::{Eq, Ord}; // float comparision with precision
use crate::prim::{typ::*, mpz::*, mpf::*};

use impl_ops_cmp::*;

impl_ops_cmp!{impl PartialEq, PartialOrd, cmp for mpf_s, mpf_s, 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_d for mpf_s, double_t, 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_ui for mpf_s, ui_t, 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_si for mpf_s, si_t, 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_z for mpf_s, mpz_r<'_>, 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_z for mpf_s, mpz_t<'_>, 1}
