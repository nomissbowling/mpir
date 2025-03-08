//! mpq
//!

use std::fmt;
use std::error::Error;

use crate::prim::{*, typ::*, mpz::*, mpf::*, gmp::*};

/// __mpq_struct
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
      _mp_num: __mpz_struct::new(),
      _mp_den: __mpz_struct::new()
    }
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
