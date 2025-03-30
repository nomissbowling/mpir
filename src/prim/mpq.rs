//! mpq
//!

use std::fmt;
use std::error::Error;

use crate::prim::{*, typ::*, mpz::*, mpf::*, gmp::*}; // randstate::*

/// __mpq_struct
// not use #[derive(Clone)]
#[repr(C)]
pub struct __mpq_struct {
  /// _mp_num
  pub _mp_num: __mpz_struct,
  /// _mp_den
  pub _mp_den: __mpz_struct
}

/// impl SNew
impl SNew for __mpq_struct {
  /// new
  fn new() -> Self {
    __mpq_struct {
      _mp_num: mpz_s::new(), // init_set_ui(0),
      _mp_den: mpz_s::new() // init_set_ui(1)
    }
  }
}

/// impl Drop
impl Drop for __mpq_struct {
  fn drop(&mut self) {
    // needless to call self.clear() when auto called clear for mpz_s members
/*
    self._mp_num = mpz_s::new(); // must set _mp_num before call self.clear()
    self._mp_den = mpz_s::new(); // must set _mp_den before call self.clear()
    self.clear() // duplex (already called clear for mpz_s members)
*/
  }
}

/// impl mpq_s
impl __mpq_struct {
  /// clear
  pub fn clear(&mut self) -> () {
    mpq_clear(self)
  }

  /// init create new instance
  pub fn init() -> Self {
    let mut t = mpq_s::new();
    mpq_init(&mut t);
    t
  }

  /// set self = q
  pub fn set(&mut self, q: mpq_r) -> &mut Self {
    mpq_set(self, q);
    self
  }

  /// set_ui self = u
  pub fn set_ui(&mut self, u: ui_t, f: ui_t) -> &mut Self {
    mpq_set_ui(self, u, f);
    self
  }

  /// set_si self = s
  pub fn set_si(&mut self, s: si_t, f: ui_t) -> &mut Self {
    mpq_set_si(self, s, f);
    self
  }

  /// set_d self = d
  pub fn set_d(&mut self, d: double_t) -> &mut Self {
    mpq_set_d(self, d);
    self
  }

  /// set_z self = a
  pub fn set_z(&mut self, a: mpz_r) -> &mut Self {
    mpq_set_z(self, a);
    self
  }

  /// set_f self = f
  pub fn set_f(&mut self, f: mpf_r) -> &mut Self {
    mpq_set_f(self, f);
    self
  }

  /// set_num
  pub fn set_num(&mut self, num: mpz_r) -> &mut Self {
    mpq_set_num(self, num);
    self
  }

  /// set_den
  pub fn set_den(&mut self, den: mpz_r) -> &mut Self {
    mpq_set_den(self, den);
    self
  }

  /// set_str self from str
  pub fn set_str(&mut self, s: &str, b: int_t) -> &mut Self {
    mpq_set_str(self, s, b);
    self
  }

  /// fmtstr
  pub fn fmtstr(&self, b: int_t) -> String {
    mpq_get_str(None, b, self).expect("mpq fmtstr")
  }

  /// get_d (loss of digits)
  pub fn get_d(&self) -> double_t {
    mpq_get_d(self)
  }

  /// get_num create new instance mpz_s (direct use of numref is recommended)
  #[inline]
  pub fn get_num(&self) -> mpz_s {
/*
    let mut t = mpz_s::init();
    mpq_get_num(&mut t, self);
    t
*/
    mpz_s::init_set(self.numref())
  }

  /// get_den create new instance mpz_s (direct use of denref is recommended)
  #[inline]
  pub fn get_den(&self) -> mpz_s {
/*
    let mut t = mpz_s::init();
    mpq_get_den(&mut t, self);
    t
*/
    mpz_s::init_set(self.denref())
  }

  /// numref
  #[inline]
  pub fn numref(&self) -> mpz_r {
    mpq_numref(self)
  }

  /// denref
  #[inline]
  pub fn denref(&self) -> mpz_r {
    mpq_denref(self)
  }

  /// swap
  pub fn swap(&mut self, r: mpq_t) -> &mut Self {
    mpq_swap(self, r);
    self
  }

  /// cmp
  pub fn cmp(&self, r: mpq_r) -> int_t {
    mpq_cmp(self, r)
  }

  /// cmp_z
  pub fn cmp_z(&self, a: mpz_r) -> int_t {
    mpq_cmp_z(self, a)
  }

  /// cmp_ui
  pub fn cmp_ui(&self, u: ui_t) -> int_t {
    mpq_cmp_ui(self, u)
  }

  /// cmp_si
  pub fn cmp_si(&self, s: si_t) -> int_t {
    mpq_cmp_si(self, s)
  }

  /// equal
  pub fn equal(&self, r: mpq_r) -> bool {
    mpq_equal(self, r)
  }

  /// sgn
  pub fn sgn(&self) -> int_t {
    mpq_sgn(self)
  }

  /// inv q**-1 create new instance
  pub fn inv(&self) -> Self {
    let mut t = mpq_s::init(); // new();
    mpq_inv(&mut t, self);
    t
  }

  /// abs create new instance
  pub fn abs(&self) -> Self {
    let mut t = mpq_s::init(); // new();
    mpq_abs(&mut t, self);
    t
  }

  /// neg create new instance
  pub fn neg(&self) -> Self {
    let mut t = mpq_s::init(); // new();
    mpq_neg(&mut t, self);
    t
  }

  /// sub self -= r
  pub fn sub(&mut self, r: mpq_r) -> &mut Self {
    mpq_sub(self, mpq_s::init().set(self), r);
    self
  }

  /// add self += r
  pub fn add(&mut self, r: mpq_r) -> &mut Self {
    mpq_add(self, mpq_s::init().set(self), r);
    self
  }

  /// mul self *= r
  pub fn mul(&mut self, r: mpq_r) -> &mut Self {
    mpq_mul(self, mpq_s::init().set(self), r);
    self
  }

  /// mul_2exp self *= 2**n
  pub fn mul_2exp(&mut self, n: mp_bitcnt_t) -> &mut Self {
    mpq_mul_2exp(self, mpq_s::init().set(self), n);
    self
  }

  /// div self /= r
  pub fn div(&mut self, r: mpq_r) -> &mut Self {
    mpq_div(self, mpq_s::init().set(self), r);
    self
  }

  /// div_2exp self /= 2**n
  pub fn div_2exp(&mut self, n: mp_bitcnt_t) -> &mut Self {
    mpq_div_2exp(self, mpq_s::init().set(self), n);
    self
  }

  /// frac create new instance
  #[inline]
  pub fn frac(a: mpz_r, b: mpz_r) -> Self {
    let mut t = mpq_s::init();
    t.set_num(a).set_den(b);
    t
  }

  /// reduction of fraction self = (num / gcd) / (den / gcd)
  #[inline]
  pub fn reduce(&mut self) -> &mut Self {
    let gcd = &self.numref().gcd(self.denref());
    let mut num = mpz_s::init();
    let mut den = mpz_s::init();
    mpz_divexact(&mut num, self.numref(), gcd);
    mpz_divexact(&mut den, self.denref(), gcd);
    self.set_num(&num);
    self.set_den(&den);
    self
  }
}

/// impl Debug
impl fmt::Debug for __mpq_struct {
  /// fmt
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({:?}) / ({:?})", self._mp_num, self._mp_den)
  }
}

/// impl Display
impl fmt::Display for __mpq_struct {
  /// fmt
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.fmtstr(10))
  }
}

/// mpq_s
#[allow(non_camel_case_types)]
pub type mpq_s = __mpq_struct; // [__mpq_struct; 1]
/// mpq_t
#[allow(non_camel_case_types)]
pub type mpq_t<'a> = &'a mut mpq_s; // *mut mpq_s
/// mpq_r
#[allow(non_camel_case_types)]
pub type mpq_r<'a> = &'a mpq_s; // *const mpq_s

/// mpq_clears
pub fn mpq_clears(vq: &mut Vec<mpq_t>) -> () {
  vq.iter_mut().for_each(|q| {
    unsafe { __gmpq_clear(*q) } // not use __gmpq_clears
  })
}

/// mpq_clear
#[inline]
pub fn mpq_clear(q: mpq_t) -> () {
  unsafe { __gmpq_clear(q) }
}

/// mpq_inits
pub fn mpq_inits(vq: &mut Vec<mpq_t>) -> () {
  vq.iter_mut().for_each(|q| {
    unsafe { __gmpq_init(*q) } // not use __gmpq_inits
  })
}

/// mpq_init
#[inline]
pub fn mpq_init(q: mpq_t) -> () {
  unsafe { __gmpq_init(q) }
}

/// mpq_set
#[inline]
pub fn mpq_set(q: mpq_t, r: mpq_r) -> () {
  unsafe { __gmpq_set(q, r) }
}

/// mpq_set_ui
#[inline]
pub fn mpq_set_ui(q: mpq_t, u: ui_t, f: ui_t) -> () {
  unsafe { __gmpq_set_ui(q, u, f) }
}

/// mpq_set_si
#[inline]
pub fn mpq_set_si(q: mpq_t, s: si_t, f: ui_t) -> () {
  unsafe { __gmpq_set_si(q, s, f) }
}

/// mpq_set_d
#[inline]
pub fn mpq_set_d(q: mpq_t, d: double_t) -> () {
  unsafe { __gmpq_set_d(q, d) }
}

/// mpq_set_z
#[inline]
pub fn mpq_set_z(q: mpq_t, a: mpz_r) -> () {
  unsafe { __gmpq_set_z(q, a) }
}

/// mpq_set_f
#[inline]
pub fn mpq_set_f(q: mpq_t, f: mpf_r) -> () {
  unsafe { __gmpq_set_f(q, f) }
}

/// mpq_set_num
#[inline]
pub fn mpq_set_num(q: mpq_t, num: mpz_r) -> () {
  unsafe { __gmpq_set_num(q, num) }
}

/// mpq_set_den
#[inline]
pub fn mpq_set_den(q: mpq_t, den: mpz_r) -> () {
  unsafe { __gmpq_set_den(q, den) }
}

/// mpq_set_str
#[inline]
pub fn mpq_set_str(q: mpq_t, s: &str, b: int_t) -> () {
  mpq_set_str_u8z(q, to_u8z!(s), b)
}

/// mpq_set_str_u8z
#[inline]
pub fn mpq_set_str_u8z(q: mpq_t, s: &[u8], b: int_t) -> () {
  unsafe { __gmpq_set_str(q, s as *const [u8] as *const u8, b) }
}

/// mpq_get_u8z
pub fn mpq_get_u8z<'a>(s: Option<&mut [u8]>, b: int_t, q: &'a mpq_s) ->
  Option<Vec<u8>> {
  let ff = s == None;
unsafe {
  let p = __gmpq_get_str(
    match s { None => 0 as *mut u8, Some(s) => s as *mut [u8] as *mut u8 },
    b, q);
  u8zvec(p, ff)
}
}

/// mpq_get_str
/// the length of mut String must larger than mpq_t display length
pub fn mpq_get_str<'a>(s: Option<&mut String>, b: int_t, q: &'a mpq_s) ->
  Result<String, Box<dyn Error>> {
  let r = mpq_get_u8z(
    match s { None => None, Some(s) => Some(unsafe { s.as_bytes_mut() }) },
    b, q);
  match r {
  None => Err("err mpq get str".into()),
  Some(r) => Ok(String::from_utf8(r)?)
  }
}

/// mpq_get_d
#[inline]
pub fn mpq_get_d(q: mpq_r) -> double_t {
  unsafe { __gmpq_get_d(q) }
}

/// mpq_get_num (direct use of numref is recommended) ***may be bug in gmp***
#[inline]
pub fn mpq_get_num(num: mpz_t, q: mpq_r) -> () {
  unsafe { __gmpq_get_num(num, q) }
}

/// mpq_get_den (direct use of denref is recommended) ***may be bug in gmp***
#[inline]
pub fn mpq_get_den(den: mpz_t, q: mpq_r) -> () {
  unsafe { __gmpq_get_den(den, q) }
}

/// mpq_numref
#[inline]
pub fn mpq_numref(q: mpq_r) -> mpz_r {
/*
  unsafe { &mut std::slice::from_raw_parts(__gmpq_numref(q), 1)[0] }
*/
  &q._mp_num
}

/// mpq_denref
#[inline]
pub fn mpq_denref(q: mpq_r) -> mpz_r {
/*
  unsafe { &mut std::slice::from_raw_parts(__gmpq_denref(q), 1)[0] }
*/
  &q._mp_den
}

/// mpq_swap
#[inline]
pub fn mpq_swap(q: mpq_t, r: mpq_t) -> () {
  unsafe { __gmpq_swap(q, r) }
}

/// mpq_cmp
#[inline]
pub fn mpq_cmp(q: mpq_r, r: mpq_r) -> int_t {
  unsafe { __gmpq_cmp(q, r) }
}

/// mpq_cmp_z
#[inline]
pub fn mpq_cmp_z(q: mpq_r, a: mpz_r) -> int_t {
  unsafe { __gmpq_cmp_z(q, a) }
}

/// mpq_cmp_ui
#[inline]
pub fn mpq_cmp_ui(q: mpq_r, u: ui_t) -> int_t {
  unsafe { __gmpq_cmp_ui(q, u) }
}

/// mpq_cmp_si
#[inline]
pub fn mpq_cmp_si(q: mpq_r, s: si_t) -> int_t {
  unsafe { __gmpq_cmp_si(q, s) }
}

/// mpq_equal
#[inline]
pub fn mpq_equal(q: mpq_r, r: mpq_r) -> bool {
  unsafe { __gmpq_equal(q, r) != 0 }
}

/// mpq_sgn
#[inline]
pub fn mpq_sgn(q: mpq_r) -> int_t {
//  unsafe { __gmpq_sgn(q) }
  let t = q._mp_num._mp_size;
  if t < 0 { -1 } else { if t > 0 { 1 } else { 0 } }
}

/// mpq_inv p = q**-1
#[inline]
pub fn mpq_inv(p: mpq_t, q: mpq_r) -> () {
  unsafe { __gmpq_inv(p, q) }
}

/// mpq_abs
#[inline]
pub fn mpq_abs(p: mpq_t, q: mpq_r) -> () {
  unsafe { __gmpq_abs(p, q) }
}

/// mpq_neg
#[inline]
pub fn mpq_neg(p: mpq_t, q: mpq_r) -> () {
  unsafe { __gmpq_neg(p, q) }
}

/// mpq_sub p = q - r
#[inline]
pub fn mpq_sub(p: mpq_t, q: mpq_r, r: mpq_r) -> () {
  unsafe { __gmpq_sub(p, q, r) }
}

/// mpq_add p = q + r
#[inline]
pub fn mpq_add(p: mpq_t, q: mpq_r, r: mpq_r) -> () {
  unsafe { __gmpq_add(p, q, r) }
}

/// mpq_mul p = q * r
#[inline]
pub fn mpq_mul(p: mpq_t, q: mpq_r, r: mpq_r) -> () {
  unsafe { __gmpq_mul(p, q, r) }
}

/// mpq_mul_2exp p = q * 2**n
#[inline]
pub fn mpq_mul_2exp(p: mpq_t, q: mpq_r, n: mp_bitcnt_t) -> () {
  unsafe { __gmpq_mul_2exp(p, q, n) }
}

/// mpq_div p = q / r
#[inline]
pub fn mpq_div(p: mpq_t, q: mpq_r, r: mpq_r) -> () {
  unsafe { __gmpq_div(p, q, r) }
}

/// mpq_div_2exp p = q / 2**n
#[inline]
pub fn mpq_div_2exp(p: mpq_t, q: mpq_r, n: mp_bitcnt_t) -> () {
  unsafe { __gmpq_div_2exp(p, q, n) }
}
