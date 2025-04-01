//! convert
//!

use std::convert::From;

use crate::prim::{typ::*, mpz::*, mpf::*, mpq::*};

/// mpz_s from mpz_r
impl<'a> From<&'a mpz_s> for mpz_s {
  /// mpz_s from mpz_r
  #[inline]
  fn from(a: &'a mpz_s) -> Self {
    mpz_s::init_set(a)
  }
}

/// mpz_s from ui_t
impl From<ui_t> for mpz_s {
  /// mpz_s from ui_t
  #[inline]
  fn from(u: ui_t) -> Self {
    mpz_s::init_set_ui(u)
  }
}

/// mpz_s from si_t
impl From<si_t> for mpz_s {
  /// mpz_s from si_t
  #[inline]
  fn from(s: si_t) -> Self {
    mpz_s::init_set_si(s)
  }
}

/// mpz_s from double_t
impl From<double_t> for mpz_s {
  /// mpz_s from double_t
  #[inline]
  fn from(d: double_t) -> Self {
    mpz_s::init_set_d(d)
  }
}

/// mpz_s from &amp;str
impl<'a> From<&'a str> for mpz_s {
  /// mpz_s from &amp;str
  #[inline]
  fn from(s: &'a str) -> Self {
    mpz_s::init_set_str(s, 10)
  }
}

/// mpf_s from mpf_r
impl<'a> From<&'a mpf_s> for mpf_s {
  /// mpf_s from mpf_r
  #[inline]
  fn from(f: &'a mpf_s) -> Self {
    mpf_s::init_set(f)
  }
}

/// mpf_s from ui_t
impl From<ui_t> for mpf_s {
  /// mpf_s from ui_t
  #[inline]
  fn from(u: ui_t) -> Self {
    mpf_s::init_set_ui(u)
  }
}

/// mpf_s from si_t
impl From<si_t> for mpf_s {
  /// mpf_s from si_t
  #[inline]
  fn from(s: si_t) -> Self {
    mpf_s::init_set_si(s)
  }
}

/// mpf_s from double_t
impl From<double_t> for mpf_s {
  /// mpf_s from double_t
  #[inline]
  fn from(d: double_t) -> Self {
    mpf_s::init_set_d(d)
  }
}

/// mpf_s from &amp;str
impl<'a> From<&'a str> for mpf_s {
  /// mpf_s from &amp;str
  #[inline]
  fn from(s: &'a str) -> Self {
    mpf_s::init_set_str(s, 10)
  }
}

/// mpf_s from mpz_r
impl<'a> From<&'a mpz_s> for mpf_s {
  /// mpf_s from mpz_r
  #[inline]
  fn from(a: &'a mpz_s) -> Self {
    mpf_s::init_set_z(a)
  }
}

/// mpf_s from mpq_r
impl<'a> From<&'a mpq_s> for mpf_s {
  /// mpf_s from mpq_r
  #[inline]
  fn from(q: &'a mpq_s) -> Self {
    mpf_s::init_set_q(q)
  }
}

/// mpq_s from mpq_r
impl<'a> From<&'a mpq_s> for mpq_s {
  /// mpq_s from mpq_r
  #[inline]
  fn from(q: &'a mpq_s) -> Self {
    let mut t = mpq_s::init();
    t.set(q);
    t
  }
}

/// mpq_s from (ui_t, ui_t)
impl From<(ui_t, ui_t)> for mpq_s {
  /// mpq_s from (ui_t, ui_t)
  #[inline]
  fn from((p, q): (ui_t, ui_t)) -> Self {
    let mut t = mpq_s::init();
    t.set_ui(p, q);
    t
  }
}

/// mpq_s from (si_t, ui_t)
impl From<(si_t, ui_t)> for mpq_s {
  /// mpq_s from (si_t, ui_t)
  #[inline]
  fn from((p, q): (si_t, ui_t)) -> Self {
    let mut t = mpq_s::init();
    t.set_si(p, q);
    t
  }
}

/// mpq_s from double_t
impl From<double_t> for mpq_s {
  /// mpq_s from double_t
  #[inline]
  fn from(d: double_t) -> Self {
    let mut t = mpq_s::init();
    t.set_d(d);
    t
  }
}

/// mpq_s from &amp;str
impl<'a> From<&'a str> for mpq_s {
  /// mpq_s from &amp;str
  #[inline]
  fn from(s: &'a str) -> Self {
    let mut t = mpq_s::init();
    t.set_str(s, 10);
    t
  }
}

/// mpq_s from mpz_r
impl<'a> From<&'a mpz_s> for mpq_s {
  /// mpq_s from mpz_r
  #[inline]
  fn from(a: &'a mpz_s) -> Self {
    let mut t = mpq_s::init();
    t.set_z(a);
    t
  }
}

/// mpq_s from mpf_r
impl<'a> From<&'a mpf_s> for mpq_s {
  /// mpq_s from mpf_r
  #[inline]
  fn from(f: &'a mpf_s) -> Self {
    let mut t = mpq_s::init();
    t.set_f(f);
    t
  }
}

/// mpq_s from (mpz_r, mpz_r)
impl<'a, 'b> From<(&'a mpz_s, &'b mpz_s)> for mpq_s {
  /// mpq_s from (mpz_r, mpz_r)
  #[inline]
  fn from((p, q): (&'a mpz_s, &'b mpz_s)) -> Self {
    mpq_s::frac(p, q)
  }
}
