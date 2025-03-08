//! mpz
//!

use std::fmt;
use std::error::Error;

use crate::prim::{*, typ::*, gmp::*}; // mpf::*, mpq::*

/// __mpz_struct
#[repr(C)]
pub struct __mpz_struct {
  /// _mp_alloc
  pub _mp_alloc: int_t,
  /// _mp_size
  pub _mp_size: int_t,
  /// _mp_d
  pub _mp_d: *mut mp_limb_t
}

/// impl SNew
impl SNew for __mpz_struct {
  /// new
  fn new() -> Self {
    __mpz_struct {
      _mp_alloc: 0,
      _mp_size: 0,
      _mp_d: 0 as *mut mp_limb_t
    }
  }
}

/// impl Debug
impl fmt::Debug for __mpz_struct {
  /// fmt
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut s = String::from("");
unsafe {
    std::slice::from_raw_parts(self._mp_d, self._mp_alloc as usize).iter()
      .for_each(|d| s += format!(" {:016x}", d).as_str())
}
    write!(f, "{}, {}{}", self._mp_alloc, self._mp_size, s)
  }
}

/// impl Display
impl fmt::Display for __mpz_struct {
  /// fmt
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", mpz_get_str(None, 10, self).expect("mpz str"))
  }
}

/// mpz_s
#[allow(non_camel_case_types)]
pub type mpz_s = __mpz_struct; // [__mpz_struct; 1]
/// mpz_t
#[allow(non_camel_case_types)]
pub type mpz_t<'a> = &'a mut mpz_s; // *mut mpz_s

/// mpz_init
pub fn mpz_init(a: mpz_t) -> () {
  unsafe { __gmpz_init(a) }
}

/// mpz_init_set
pub fn mpz_init_set(a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_init_set(a, b) }
}

/// mpz_init_set_ui
pub fn mpz_init_set_ui(a: mpz_t, u: ui_t) -> () {
  unsafe { __gmpz_init_set_ui(a, u) }
}

/// mpz_init_set_si
pub fn mpz_init_set_si(a: mpz_t, s: si_t) -> () {
  unsafe { __gmpz_init_set_si(a, s) }
}

/// mpz_init_set_d
pub fn mpz_init_set_d(a: mpz_t, d: double_t) -> () {
  unsafe { __gmpz_init_set_d(a, d) }
}

/// mpz_init_set_str
pub fn mpz_init_set_str(a: mpz_t, s: &str, b: int_t) -> () {
  mpz_init_set_str_u8z(a, to_u8z!(s), b)
}

/// mpz_init_set_str_u8z
pub fn mpz_init_set_str_u8z(a: mpz_t, s: &[u8], b: int_t) -> () {
  unsafe { __gmpz_init_set_str(a, s as *const [u8] as *const u8, b) }
}

/// mpz_set
pub fn mpz_set(a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_set(a, b) }
}

/// mpz_set_ui
pub fn mpz_set_ui(a: mpz_t, u: ui_t) -> () {
  unsafe { __gmpz_set_ui(a, u) }
}

/// mpz_set_si
pub fn mpz_set_si(a: mpz_t, s: si_t) -> () {
  unsafe { __gmpz_set_si(a, s) }
}

/// mpz_set_d
pub fn mpz_set_d(a: mpz_t, d: double_t) -> () {
  unsafe { __gmpz_set_d(a, d) }
}

/// mpz_set_str
pub fn mpz_set_str(a: mpz_t, s: &str, b: int_t) -> () {
  mpz_set_str_u8z(a, to_u8z!(s), b)
}

/// mpz_set_str_u8z
pub fn mpz_set_str_u8z(a: mpz_t, s: &[u8], b: int_t) -> () {
  unsafe { __gmpz_set_str(a, s as *const [u8] as *const u8, b) }
}

/// mpz_get_u8z
pub fn mpz_get_u8z<'a>(s: Option<&mut [u8]>, b: int_t, a: &'a mpz_s) ->
  Option<Vec<u8>> {
  let ff = s == None;
unsafe {
  let p = __gmpz_get_str(
    match s { None => 0 as *mut u8, Some(s) => s as *mut [u8] as *mut u8 },
    b, a);
  u8zvec(p, ff)
}
}

/// mpz_get_str
/// the length of mut String must larger than mpz_t display length
pub fn mpz_get_str<'a>(s: Option<&mut String>, b: int_t, a: &'a mpz_s) ->
  Result<String, Box<dyn Error>> {
  let r = mpz_get_u8z(
    match s { None => None, Some(s) => Some(unsafe { s.as_bytes_mut() }) },
    b, a);
  match r {
  None => Err("err mpz get str".into()),
  Some(r) => Ok(String::from_utf8(r)?)
  }
}
