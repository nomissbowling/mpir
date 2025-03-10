//! mpq
//!

use std::fmt;
use std::error::Error;

use crate::prim::{*, typ::*, mpz::*, mpf::*, gmp::*};

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
  pub fn set(&mut self, q: mpq_t) -> &mut Self {
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
  pub fn set_z(&mut self, a: mpz_t) -> &mut Self {
    mpq_set_z(self, a);
    self
  }

  /// set_f self = f
  pub fn set_f(&mut self, f: mpf_t) -> &mut Self {
    mpq_set_f(self, f);
    self
  }

  /// set_num
  pub fn set_num(&mut self, num: mpz_t) -> &mut Self {
    mpq_set_num(self, num);
    self
  }

  /// set_den
  pub fn set_den(&mut self, den: mpz_t) -> &mut Self {
    mpq_set_den(self, den);
    self
  }

  /// set_str self from str
  pub fn set_str(&mut self, s: &str, b: int_t) -> &mut Self {
    mpq_set_str(self, s, b);
    self
  }

  /// cmp
  pub fn cmp(&mut self, r: mpq_t) -> int_t {
    mpq_cmp(self, r)
  }

  /// cmp_z
  pub fn cmp_z(&mut self, a: mpz_t) -> int_t {
    mpq_cmp_z(self, a)
  }

  /// cmp_ui
  pub fn cmp_ui(&mut self, u: ui_t) -> int_t {
    mpq_cmp_ui(self, u)
  }

  /// cmp_si
  pub fn cmp_si(&mut self, s: si_t) -> int_t {
    mpq_cmp_si(self, s)
  }

  /// equal
  pub fn equal(&mut self, r: mpq_t) -> bool {
    mpq_equal(self, r)
  }

  /// sgn
  pub fn sgn(&mut self) -> int_t {
    mpq_sgn(self)
  }

  /// inv q**-1 create new instance
  pub fn inv(&mut self) -> Self {
    let mut t = mpq_s::init(); // new();
    mpq_inv(&mut t, self);
    t
  }

  /// abs create new instance
  pub fn abs(&mut self) -> Self {
    let mut t = mpq_s::init(); // new();
    mpq_abs(&mut t, self);
    t
  }

  /// neg create new instance
  pub fn neg(&mut self) -> Self {
    let mut t = mpq_s::init(); // new();
    mpq_neg(&mut t, self);
    t
  }

  /// sub self -= r
  pub fn sub(&mut self, r: mpq_t) -> &mut Self {
    let t = &mut mpq_s::init();
    t.set(self);
    mpq_sub(self, t, r);
    self
  }

  /// add self += r
  pub fn add(&mut self, r: mpq_t) -> &mut Self {
    let t = &mut mpq_s::init();
    t.set(self);
    mpq_add(self, t, r);
    self
  }

  /// mul self *= r
  pub fn mul(&mut self, r: mpq_t) -> &mut Self {
    let t = &mut mpq_s::init();
    t.set(self);
    mpq_mul(self, t, r);
    self
  }

  /// mul_2exp self *= 2**n
  pub fn mul_2exp(&mut self, n: mp_bitcnt_t) -> &mut Self {
    let t = &mut mpq_s::init();
    t.set(self);
    mpq_mul_2exp(self, t, n);
    self
  }

  /// div self /= r
  pub fn div(&mut self, r: mpq_t) -> &mut Self {
    let t = &mut mpq_s::init();
    t.set(self);
    mpq_div(self, t, r);
    self
  }

  /// div_2exp self /= 2**n
  pub fn div_2exp(&mut self, n: mp_bitcnt_t) -> &mut Self {
    let t = &mut mpq_s::init();
    t.set(self);
    mpq_div_2exp(self, t, n);
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
    write!(f, "{}", mpq_get_str(None, 10, self).expect("mpq str"))
  }
}

/// mpq_s
#[allow(non_camel_case_types)]
pub type mpq_s = __mpq_struct; // [__mpq_struct; 1]
/// mpq_t
#[allow(non_camel_case_types)]
pub type mpq_t<'a> = &'a mut mpq_s; // *mut mpq_s

/// mpq_clears
pub fn mpq_clears(vq: &mut Vec<mpq_t>) -> () {
  vq.iter_mut().for_each(|q| {
    unsafe { __gmpq_clear(*q) } // not use __gmpq_clears
  })
}

/// mpq_clear
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
pub fn mpq_init(q: mpq_t) -> () {
  unsafe { __gmpq_init(q) }
}

/// mpq_set
pub fn mpq_set(q: mpq_t, r: mpq_t) -> () {
  unsafe { __gmpq_set(q, r) }
}

/// mpq_set_ui
pub fn mpq_set_ui(q: mpq_t, u: ui_t, f: ui_t) -> () {
  unsafe { __gmpq_set_ui(q, u, f) }
}

/// mpq_set_si
pub fn mpq_set_si(q: mpq_t, s: si_t, f: ui_t) -> () {
  unsafe { __gmpq_set_si(q, s, f) }
}

/// mpq_set_d
pub fn mpq_set_d(q: mpq_t, d: double_t) -> () {
  unsafe { __gmpq_set_d(q, d) }
}

/// mpq_set_z
pub fn mpq_set_z(q: mpq_t, a: mpz_t) -> () {
  unsafe { __gmpq_set_z(q, a) }
}

/// mpq_set_f
pub fn mpq_set_f(q: mpq_t, f: mpf_t) -> () {
  unsafe { __gmpq_set_f(q, f) }
}

/// mpq_set_num
pub fn mpq_set_num(q: mpq_t, num: mpz_t) -> () {
  unsafe { __gmpq_set_num(q, num) }
}

/// mpq_set_den
pub fn mpq_set_den(q: mpq_t, den: mpz_t) -> () {
  unsafe { __gmpq_set_den(q, den) }
}

/// mpq_set_str
pub fn mpq_set_str(q: mpq_t, s: &str, b: int_t) -> () {
  mpq_set_str_u8z(q, to_u8z!(s), b)
}

/// mpq_set_str_u8z
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

/// mpq_cmp
pub fn mpq_cmp(q: mpq_t, r: mpq_t) -> int_t {
  unsafe { __gmpq_cmp(q, r) }
}

/// mpq_cmp_z
pub fn mpq_cmp_z(q: mpq_t, a: mpz_t) -> int_t {
  unsafe { __gmpq_cmp_z(q, a) }
}

/// mpq_cmp_ui
pub fn mpq_cmp_ui(q: mpq_t, u: ui_t) -> int_t {
  unsafe { __gmpq_cmp_ui(q, u) }
}

/// mpq_cmp_si
pub fn mpq_cmp_si(q: mpq_t, s: si_t) -> int_t {
  unsafe { __gmpq_cmp_si(q, s) }
}

/// mpq_equal
pub fn mpq_equal(q: mpq_t, r: mpq_t) -> bool {
  unsafe { __gmpq_equal(q, r) != 0 }
}

/// mpq_sgn
pub fn mpq_sgn(q: mpq_t) -> int_t {
//  unsafe { __gmpq_sgn(q) }
  let t = q._mp_num._mp_size;
  if t < 0 { -1 } else { if t > 0 { 1 } else { 0 } }
}

/// mpq_inv p = q**-1
pub fn mpq_inv(p: mpq_t, q: mpq_t) -> () {
  unsafe { __gmpq_inv(p, q) }
}

/// mpq_abs
pub fn mpq_abs(p: mpq_t, q: mpq_t) -> () {
  unsafe { __gmpq_abs(p, q) }
}

/// mpq_neg
pub fn mpq_neg(p: mpq_t, q: mpq_t) -> () {
  unsafe { __gmpq_neg(p, q) }
}

/// mpq_sub p = q - r
pub fn mpq_sub(p: mpq_t, q: mpq_t, r: mpq_t) -> () {
  unsafe { __gmpq_sub(p, q, r) }
}

/// mpq_add p = q + r
pub fn mpq_add(p: mpq_t, q: mpq_t, r: mpq_t) -> () {
  unsafe { __gmpq_add(p, q, r) }
}

/// mpq_mul p = q * r
pub fn mpq_mul(p: mpq_t, q: mpq_t, r: mpq_t) -> () {
  unsafe { __gmpq_mul(p, q, r) }
}

/// mpq_mul_2exp p = q * 2**n
pub fn mpq_mul_2exp(p: mpq_t, q: mpq_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpq_mul_2exp(p, q, n) }
}

/// mpq_div p = q / r
pub fn mpq_div(p: mpq_t, q: mpq_t, r: mpq_t) -> () {
  unsafe { __gmpq_div(p, q, r) }
}

/// mpq_div_2exp p = q / 2**n
pub fn mpq_div_2exp(p: mpq_t, q: mpq_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpq_div_2exp(p, q, n) }
}
