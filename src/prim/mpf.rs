//! mpf
//!

use std::fmt;
use std::error::Error;

use crate::prim::{*, typ::*, mpz::*, gmp::*}; // mpq::*

/// __mpf_struct
#[repr(C)]
pub struct __mpf_struct {
  /// _mp_prec
  pub _mp_prec: int_t,
  /// _mp_size
  pub _mp_size: int_t,
  /// _mp_exp
  pub _mp_exp: mp_exp_t,
  /// _mp_d
  pub _mp_d: *mut mp_limb_t
}

/// impl SNew
impl SNew for __mpf_struct {
  /// new
  fn new() -> Self {
    __mpf_struct {
      _mp_prec: 0,
      _mp_size: 0,
      _mp_exp: 0,
      _mp_d: 0 as *mut mp_limb_t
    }
  }
}

/// impl Debug
impl fmt::Debug for __mpf_struct {
  /// fmt
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut s = String::from("");
unsafe {
    let mut sz = self._mp_size;
    sz = if sz < 0 { -sz } else { sz };
    std::slice::from_raw_parts(self._mp_d, sz as usize).iter()
      .for_each(|d| s += format!(" {:016x}", d).as_str())
}
    write!(f, "{}, {}, {}{}", self._mp_prec, self._mp_size, self._mp_exp, s)
  }
}

/// impl Display
impl fmt::Display for __mpf_struct {
  /// fmt
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", mpf_get_fmtstr(self, 10, 20).expect("mpf fmtstr"))
  }
}

/// mpf_s
#[allow(non_camel_case_types)]
pub type mpf_s = __mpf_struct; // [__mpf_struct; 1]
/// mpf_t
#[allow(non_camel_case_types)]
pub type mpf_t<'a> = &'a mut mpf_s; // *mut mpf_s

/// mpf_init
pub fn mpf_init(f: mpf_t) -> () {
  unsafe { __gmpf_init(f) }
}

/// mpf_init_set
pub fn mpf_init_set(f: mpf_t, g: mpf_t) -> () {
  unsafe { __gmpf_init_set(f, g) }
}

/// mpf_init_set_ui
pub fn mpf_init_set_ui(f: mpf_t, u: ui_t) -> () {
  unsafe { __gmpf_init_set_ui(f, u) }
}

/// mpf_init_set_si
pub fn mpf_init_set_si(f: mpf_t, s: si_t) -> () {
  unsafe { __gmpf_init_set_si(f, s) }
}

/// mpf_init_set_d
pub fn mpf_init_set_d(f: mpf_t, d: double_t) -> () {
  unsafe { __gmpf_init_set_d(f, d) }
}

/// mpf_init_set_str
pub fn mpf_init_set_str(f: mpf_t, s: &str, b: int_t) -> () {
  mpf_init_set_str_u8z(f, to_u8z!(s), b)
}

/// mpf_init_set_str_u8z
pub fn mpf_init_set_str_u8z(f: mpf_t, s: &[u8], b: int_t) -> () {
  unsafe { __gmpf_init_set_str(f, s as *const [u8] as *const u8, b) }
}

/// mpf_set
pub fn mpf_set(f: mpf_t, g: mpf_t) -> () {
  unsafe { __gmpf_set(f, g) }
}

/// mpf_set_ui
pub fn mpf_set_ui(f: mpf_t, u: ui_t) -> () {
  unsafe { __gmpf_set_ui(f, u) }
}

/// mpf_set_si
pub fn mpf_set_si(f: mpf_t, s: si_t) -> () {
  unsafe { __gmpf_set_si(f, s) }
}

/// mpf_set_d
pub fn mpf_set_d(f: mpf_t, d: double_t) -> () {
  unsafe { __gmpf_set_d(f, d) }
}

/// mpf_set_z
pub fn mpf_set_z(f: mpf_t, a: mpz_t) -> () {
  unsafe { __gmpf_set_z(f, a) }
}

/// mpf_set_str
pub fn mpf_set_str(f: mpf_t, s: &str, b: int_t) -> () {
  mpf_set_str_u8z(f, to_u8z!(s), b)
}

/// mpf_set_str_u8z
pub fn mpf_set_str_u8z(f: mpf_t, s: &[u8], b: int_t) -> () {
  unsafe { __gmpf_set_str(f, s as *const [u8] as *const u8, b) }
}

/// mpf_get_u8z
pub fn mpf_get_u8z<'a>(s: Option<&mut [u8]>,
  e: &'a mut mp_exp_t, b: int_t, d: mp_size_t, f: &'a mpf_s) ->
  Option<Vec<u8>> {
  let ff = s == None;
unsafe {
  let p = __gmpf_get_str(
    match s { None => 0 as *mut u8, Some(s) => s as *mut [u8] as *mut u8 },
    e, b, d, f);
  u8zvec(p, ff) // return without follow code (trough check exp == 0 && "")
/*
  match u8zvec(p, ff) {
  None => None,
  Some(r) => {
    if *e == 0 && r.len() == 0 { Some(vec![48u8]) }
    else { Some(r) }
  }
  }
*/
}
}

/// mpf_get_str
/// the length of mut String must larger than mpf_t display length
pub fn mpf_get_str<'a>(s: Option<&mut String>,
  e: &'a mut mp_exp_t, b: int_t, d: mp_size_t, f: &'a mpf_s) ->
  Result<String, Box<dyn Error>> {
  let r = mpf_get_u8z(
    match s { None => None, Some(s) => Some(unsafe { s.as_bytes_mut() }) },
    e, b, d, f);
  match r {
  None => Err("err mpf get str".into()),
  Some(r) => Ok(String::from_utf8(r)?)
  }
}

/// mpf_get_fmtstr
pub fn mpf_get_fmtstr<'a>(f: &'a mpf_s, b: int_t, d: mp_size_t) ->
  Result<String, Box<dyn Error>> {
  let e = &mut (0 as mp_exp_t);
  let r = mpf_get_u8z(None, e, b, d, f);
  match r {
  None => Err("err mpf get fmtstr".into()),
  Some(r) => {
    let (l, z) = (r.len(), String::from("0"));
    let (sign, o) = if f._mp_size < 0 { ("-", 1) } else { ("", 0) };
    let (es, en) = if *e < 0 { ("-", -1) } else { ("+", 1) };
    let s = if o == l { z } else { String::from_utf8(r[o..l].to_vec())? };
    Ok(format!("{}0.{}e{}{}", sign, s, es, *e * en))
  }
  }
}

/// mpf_sqrt
pub fn mpf_sqrt(g: mpf_t, f: mpf_t) -> () {
  unsafe { __gmpf_sqrt(g, f) }
}

/// mpf_div g = f / e
pub fn mpf_div(g: mpf_t, f: mpf_t, e: mpf_t) -> () {
  unsafe { __gmpf_div(g, f, e) }
}

/// mpf_ui_div g = u / f
pub fn mpf_ui_div(g: mpf_t, u: ui_t, f: mpf_t) -> () {
  unsafe { __gmpf_ui_div(g, u, f) }
}

/// mpf_div_ui g = f / u
pub fn mpf_div_ui(g: mpf_t, f: mpf_t, u: ui_t) -> () {
  unsafe { __gmpf_div_ui(g, f, u) }
}

/// mpf_div_2exp g = f / 2**n
pub fn mpf_div_2exp(g: mpf_t, f: mpf_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpf_div_2exp(g, f, n) }
}
