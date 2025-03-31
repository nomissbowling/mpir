//! randstate
//!

use std::fmt;

use crate::prim::{*, typ::*, mpz::*, gmp::*}; // mpf::*, mpq::*

/// gmp_randalg_t
#[allow(non_camel_case_types)]
pub type gmp_randalg_t = int_t;
/// GMP_RAND_ALG_DEFAULT
pub const GMP_RAND_ALG_DEFAULT: gmp_randalg_t = 0;
/// GMP_RAND_ALG_LC
pub const GMP_RAND_ALG_LC: gmp_randalg_t = 0;
/*
/// gmp_randalg_t (available random number generation algorithms)
#[allow(non_camel_case_types)]
pub enum gmp_randalg_t {
  /// GMP_RAND_ALG_DEFAULT
  GMP_RAND_ALG_DEFAULT = 0,
  /// GMP_RAND_ALG_LC (linear congruential)
  GMP_RAND_ALG_LC = GMP_RAND_ALG_DEFAULT // (currently unused)
}
*/

/// gmp_randstate_struct
// not use #[derive(Clone)]
#[repr(C)]
pub struct gmp_randstate_struct {
  /// _mp_seed (_mp_d member points to state of the generator)
  pub _mp_seed: __mpz_struct,
  /// _mp_alg (currently unused)
  pub _mp_alg: gmp_randalg_t,
  /// _mp_algdata_union_mp_lc (c union: pointer to function pointers structure)
  pub _mp_algdata_union_mp_lc: mp_t
}

/// impl SNew
impl SNew for gmp_randstate_struct {
  /// new
  #[inline]
  fn new() -> Self {
    gmp_randstate_struct {
      _mp_seed: mpz_s::new(), // init_set_ui(0),
      _mp_alg: GMP_RAND_ALG_DEFAULT,
      _mp_algdata_union_mp_lc: 0 as mp_t
    }
  }
}

/// impl Drop
impl Drop for gmp_randstate_struct {
  fn drop(&mut self) {
/*
    self._mp_seed = mpz_s::new(); // must set _mp_seed before call self.clear()
    self.clear() // duplex (already called clear for mpz_s members)
*/
  }
}

/// impl randstate_s
impl gmp_randstate_struct {
  /// clear
  #[inline]
  pub fn clear(&mut self) -> () {
    gmp_randclear(self)
  }

  /// init create new instance ***(obsoleted)***
  #[inline]
  pub fn init(sz: mp_bitcnt_t) -> Self {
    let mut t = randstate_s::new();
    gmp_randinit(&mut t, GMP_RAND_ALG_LC, sz);
    t
  }

  /// init_set create new instance copy
  #[inline]
  pub fn init_set(s: randstate_r) -> Self {
    let mut t = randstate_s::new();
    gmp_randinit_set(&mut t, s);
    t
  }

  /// init_default create new instance
  #[inline]
  pub fn init_default() -> Self {
    let mut t = randstate_s::new();
    gmp_randinit_default(&mut t);
    t
  }

  /// init_mt create new instance
  #[inline]
  pub fn init_mt() -> Self {
    let mut t = randstate_s::new();
    gmp_randinit_mt(&mut t);
    t
  }

  /// init_lc_2exp x = (a*x + c) mod 2**m2e create new instance
  #[inline]
  pub fn init_lc_2exp(a: mpz_r, c: ui_t, m2e: mp_bitcnt_t) -> Self {
    let mut t = randstate_s::new();
    gmp_randinit_lc_2exp(&mut t, a, c, m2e);
    t
  }

  /// init_lc_2exp_size create new instance
  #[inline]
  pub fn init_lc_2exp_size(sz: mp_bitcnt_t) -> (Self, int_t) {
    let mut t = randstate_s::new();
    let i = gmp_randinit_lc_2exp_size(&mut t, sz);
    (t, i)
  }

  /// seed
  #[inline]
  pub fn seed(&mut self, seed: mpz_t) -> &mut Self {
    gmp_randseed(self, seed);
    self
  }

  /// seed_ui
  #[inline]
  pub fn seed_ui(&mut self, seed: ui_t) -> &mut Self {
    gmp_randseed_ui(self, seed);
    self
  }

  /// urandomb_ui
  #[inline]
  pub fn urandomb_ui(r: randstate_t, nbits: ui_t) -> ui_t {
    gmp_urandomb_ui(r, nbits)
  }

  /// urandomm_ui
  #[inline]
  pub fn urandomm_ui(r: randstate_t, n: ui_t) -> ui_t {
    gmp_urandomm_ui(r, n)
  }
}

/// impl Debug
impl fmt::Debug for gmp_randstate_struct {
  /// fmt
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "seed({}) alg({}) data({:016x})",
      self._mp_seed, self._mp_alg, self._mp_algdata_union_mp_lc as usize)
  }
}

/// impl Display
impl fmt::Display for gmp_randstate_struct {
  /// fmt
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self._mp_seed)
  }
}

/// randstate_s
#[allow(non_camel_case_types)]
pub type randstate_s = gmp_randstate_struct; // [gmp_randstate_struct; 1]
/// randstate_t
#[allow(non_camel_case_types)]
pub type randstate_t<'a> = &'a mut randstate_s; // *mut randstate_s
/// randstate_r
#[allow(non_camel_case_types)]
pub type randstate_r<'a> = &'a randstate_s; // *const randstate_s

/// gmp_randclear
#[inline]
pub fn gmp_randclear(r: randstate_t) -> () {
  unsafe { __gmp_randclear(r) }
}

/// gmp_randinit ***(obsoleted)***
#[inline]
pub fn gmp_randinit(r: randstate_t, _a: gmp_randalg_t, sz: mp_bitcnt_t) -> () {
/*
  unsafe { __gmp_randinit(r, a, sz) }
*/
  unsafe { __gmp_randinit_lc_2exp_size(r, sz); } // ()
}

/// gmp_randinit_set copy
#[inline]
pub fn gmp_randinit_set(r: randstate_t, s: randstate_r) -> () {
  unsafe { __gmp_randinit_set(r, s) }
}

/// gmp_randinit_default
#[inline]
pub fn gmp_randinit_default(r: randstate_t) -> () {
  unsafe { __gmp_randinit_default(r) }
}

/// gmp_randinit_mt
#[inline]
pub fn gmp_randinit_mt(r: randstate_t) -> () {
  unsafe { __gmp_randinit_mt(r) }
}

/// gmp_randinit_lc_2exp x = (a*x + c) mod 2**m2e
#[inline]
pub fn gmp_randinit_lc_2exp(r: randstate_t,
  a: mpz_r, c: ui_t, m2e: mp_bitcnt_t) -> () {
  unsafe { __gmp_randinit_lc_2exp(r, a, c, m2e) }
}

/// gmp_randinit_lc_2exp_size
#[inline]
pub fn gmp_randinit_lc_2exp_size(r: randstate_t, sz: mp_bitcnt_t) -> int_t {
  unsafe { __gmp_randinit_lc_2exp_size(r, sz) }
}

/// gmp_randseed
#[inline]
pub fn gmp_randseed(r: randstate_t, seed: mpz_t) -> () {
  unsafe { __gmp_randseed(r, seed) }
}

/// gmp_randseed_ui
#[inline]
pub fn gmp_randseed_ui(r: randstate_t, seed: ui_t) -> () {
  unsafe { __gmp_randseed_ui(r, seed) }
}

/// gmp_urandomb_ui
#[inline]
pub fn gmp_urandomb_ui(r: randstate_t, nbits: ui_t) -> ui_t {
  unsafe { __gmp_urandomb_ui(r, nbits) }
}

/// gmp_urandomm_ui
#[inline]
pub fn gmp_urandomm_ui(r: randstate_t, n: ui_t) -> ui_t {
  unsafe { __gmp_urandomm_ui(r, n) }
}
