//! cmp
//

use std::cmp::{PartialEq, PartialOrd, Ordering};
// use std::cmp::{Eq, Ord}; // fract reduction
use crate::prim::{typ::*, mpz::*, mpq::*};

use impl_ops_cmp::*;

impl_ops_cmp!{impl PartialEq, PartialOrd, cmp for mpq_s, mpq_s, 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_ui for mpq_s, (ui_t, ui_t), 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_si for mpq_s, (si_t, ui_t), 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_u for mpq_s, ui_t, 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_s for mpq_s, si_t, 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_z for mpq_s, mpz_r<'_>, 1}

impl_ops_cmp_p!{impl PartialEq, PartialOrd, cmp_z for mpq_s, mpz_t<'_>, 1}
