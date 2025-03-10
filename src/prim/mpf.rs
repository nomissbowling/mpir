//! mpf
//!

use std::fmt;
use std::error::Error;

use crate::prim::{*, typ::*, mpz::*, gmp::*}; // mpq::*

/// __mpf_struct
// not use #[derive(Clone)]
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

/// impl Drop
impl Drop for __mpf_struct {
  fn drop(&mut self) {
    self.clear()
  }
}

/// impl mpf_s
impl __mpf_struct {
  /// clear
  pub fn clear(&mut self) -> () {
    mpf_clear(self)
  }

  /// init create new instance
  pub fn init() -> Self {
    let mut t = mpf_s::new();
    mpf_init(&mut t);
    t
  }

  /// init_set create new instance
  pub fn init_set(f: mpf_t) -> Self {
    let mut t = mpf_s::new();
    mpf_init_set(&mut t, f);
    t
  }

  /// init_set_ui create new instance
  pub fn init_set_ui(u: ui_t) -> Self {
    let mut t = mpf_s::new();
    mpf_init_set_ui(&mut t, u);
    t
  }

  /// init_set_si create new instance
  pub fn init_set_si(s: si_t) -> Self {
    let mut t = mpf_s::new();
    mpf_init_set_si(&mut t, s);
    t
  }

  /// init_set_d create new instance
  pub fn init_set_d(d: double_t) -> Self {
    let mut t = mpf_s::new();
    mpf_init_set_d(&mut t, d);
    t
  }

  /// init_set_str create new instance
  pub fn init_set_str(s: &str, b: int_t) -> Self {
    let mut t = mpf_s::new();
    mpf_init_set_str(&mut t, s, b);
    t
  }

  /// set self = f
  pub fn set(&mut self, f: mpf_t) -> &mut Self {
    mpf_set(self, f);
    self
  }

  /// set_ui self = u
  pub fn set_ui(&mut self, u: ui_t) -> &mut Self {
    mpf_set_ui(self, u);
    self
  }

  /// set_si self = s
  pub fn set_si(&mut self, s: si_t) -> &mut Self {
    mpf_set_si(self, s);
    self
  }

  /// set_d self = d
  pub fn set_d(&mut self, d: double_t) -> &mut Self {
    mpf_set_d(self, d);
    self
  }

  /// set_z self = a
  pub fn set_z(&mut self, a: mpz_t) -> &mut Self {
    mpf_set_z(self, a);
    self
  }

  /// set_str self from str
  pub fn set_str(&mut self, s: &str, b: int_t) -> &mut Self {
    mpf_set_str(self, s, b);
    self
  }

  /// cmp
  pub fn cmp(&mut self, g: mpf_t) -> int_t {
    mpf_cmp(self, g)
  }

  /// cmp_d
  pub fn cmp_d(&mut self, d: double_t) -> int_t {
    mpf_cmp_d(self, d)
  }

  /// cmp_ui
  pub fn cmp_ui(&mut self, u: ui_t) -> int_t {
    mpf_cmp_ui(self, u)
  }

  /// cmp_si
  pub fn cmp_si(&mut self, s: si_t) -> int_t {
    mpf_cmp_si(self, s)
  }

  /// cmp_z
  pub fn cmp_z(&mut self, a: mpz_t) -> int_t {
    mpf_cmp_z(self, a)
  }

  /// eq ***mathematically ill-defined and should not be used***
  pub fn eq(&mut self, g: mpf_t, n: mp_bitcnt_t) -> int_t {
    mpf_eq(self, g, n)
  }

  /// sgn
  pub fn sgn(&mut self) -> int_t {
    mpf_sgn(self)
  }

  /// reldiff returns abs(self - e) / self create new instance
  pub fn reldiff(&mut self, e: mpf_t) -> Self {
    let mut t = mpf_s::new();
    mpf_reldiff(&mut t, self, e);
    t
  }

  /// sqrt create new instance
  pub fn sqrt(&mut self) -> Self {
    let mut t = mpf_s::new();
    mpf_sqrt(&mut t, self);
    t
  }

  /// sqrt create new instance
  pub fn sqrt_ui(u: ui_t) -> Self {
    let mut t = mpf_s::new();
    mpf_sqrt_ui(&mut t, u);
    t
  }

  /// abs create new instance
  pub fn abs(&mut self) -> Self {
    let mut t = mpf_s::new();
    mpf_abs(&mut t, self);
    t
  }

  /// neg create new instance
  pub fn neg(&mut self) -> Self {
    let mut t = mpf_s::new();
    mpf_neg(&mut t, self);
    t
  }

  /// sub self -= e
  pub fn sub(&mut self, e: mpf_t) -> &mut Self {
    let t = &mut mpf_s::init_set(self);
    mpf_sub(self, t, e);
    self
  }

  /// sub_ui self -= u
  pub fn sub_ui(&mut self, u: ui_t) -> &mut Self {
    let t = &mut mpf_s::init_set(self);
    mpf_sub_ui(self, t, u);
    self
  }

  /// ui_sub self = u - self
  pub fn ui_sub(&mut self, u: ui_t) -> &mut Self {
    let t = &mut mpf_s::init_set(self);
    mpf_ui_sub(self, u, t);
    self
  }

  /// add self += e
  pub fn add(&mut self, e: mpf_t) -> &mut Self {
    let t = &mut mpf_s::init_set(self);
    mpf_add(self, t, e);
    self
  }

  /// add_ui self += u
  pub fn add_ui(&mut self, u: ui_t) -> &mut Self {
    let t = &mut mpf_s::init_set(self);
    mpf_add_ui(self, t, u);
    self
  }

  /// mul self *= e
  pub fn mul(&mut self, e: mpf_t) -> &mut Self {
    let t = &mut mpf_s::init_set(self);
    mpf_mul(self, t, e);
    self
  }

  /// mul_ui self *= u
  pub fn mul_ui(&mut self, u: ui_t) -> &mut Self {
    let t = &mut mpf_s::init_set(self);
    mpf_mul_ui(self, t, u);
    self
  }

  /// mul_2exp g = f * 2**n
  pub fn mul_2exp(&mut self, n: mp_bitcnt_t) -> &mut Self {
    let t = &mut mpf_s::init_set(self);
    mpf_mul_2exp(self, t, n);
    self
  }

  /// div self /= e
  pub fn div(&mut self, e: mpf_t) -> &mut Self {
    let t = &mut mpf_s::init_set(self);
    mpf_div(self, t, e);
    self
  }

  /// div_ui self /= u
  pub fn div_ui(&mut self, u: ui_t) -> &mut Self {
    let t = &mut mpf_s::init_set(self);
    mpf_div_ui(self, t, u);
    self
  }

  /// ui_div self = u / self
  pub fn ui_div(&mut self, u: ui_t) -> &mut Self {
    let t = &mut mpf_s::init_set(self);
    mpf_ui_div(self, u, t);
    self
  }

  /// div_2exp self /= 2**n
  pub fn div_2exp(&mut self, n: mp_bitcnt_t) -> &mut Self {
    let t = &mut mpf_s::init_set(self);
    mpf_div_2exp(self, t, n);
    self
  }

  /// pow_ui f**n create new instance
  pub fn pow_ui(f: mpf_t, n: ui_t) -> Self {
    let mut t = mpf_s::new();
    mpf_pow_ui(&mut t, f, n);
    t
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

/// mpf_clears
pub fn mpf_clears(vf: &mut Vec<mpf_t>) -> () {
  vf.iter_mut().for_each(|f| {
    unsafe { __gmpf_clear(*f) } // not use __gmpf_clears
  })
}

/// mpf_clear
pub fn mpf_clear(f: mpf_t) -> () {
  unsafe { __gmpf_clear(f) }
}

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

/// mpf_cmp
pub fn mpf_cmp(f: mpf_t, g: mpf_t) -> int_t {
  unsafe { __gmpf_cmp(f, g) }
}

/// mpf_cmp_d
pub fn mpf_cmp_d(f: mpf_t, d: double_t) -> int_t {
  unsafe { __gmpf_cmp_d(f, d) }
}

/// mpf_cmp_ui
pub fn mpf_cmp_ui(f: mpf_t, u: ui_t) -> int_t {
  unsafe { __gmpf_cmp_ui(f, u) }
}

/// mpf_cmp_si
pub fn mpf_cmp_si(f: mpf_t, s: si_t) -> int_t {
  unsafe { __gmpf_cmp_si(f, s) }
}

/// mpf_cmp_z
pub fn mpf_cmp_z(f: mpf_t, a: mpz_t) -> int_t {
  unsafe { __gmpf_cmp_z(f, a) }
}

/// mpf_eq ***mathematically ill-defined and should not be used***
pub fn mpf_eq(f: mpf_t, g: mpf_t, n: mp_bitcnt_t) -> int_t {
  unsafe { __gmpf_eq(f, g, n) }
}

/// mpf_sgn
pub fn mpf_sgn(f: mpf_t) -> int_t {
//  unsafe { __gmpf_sgn(f) }
  let t = f._mp_size;
  if t < 0 { -1 } else { if t > 0 { 1 } else { 0 } }
}

/// mpf_reldiff
pub fn mpf_reldiff(g: mpf_t, f: mpf_t, e: mpf_t) -> () {
  unsafe { __gmpf_reldiff(g, f, e) }
}

/// mpf_sqrt
pub fn mpf_sqrt(g: mpf_t, f: mpf_t) -> () {
  unsafe { __gmpf_sqrt(g, f) }
}

/// mpf_sqrt_ui
pub fn mpf_sqrt_ui(g: mpf_t, u: ui_t) -> () {
  unsafe { __gmpf_sqrt_ui(g, u) }
}

/// mpf_abs
pub fn mpf_abs(g: mpf_t, f: mpf_t) -> () {
  unsafe { __gmpf_abs(g, f) }
}

/// mpf_neg
pub fn mpf_neg(g: mpf_t, f: mpf_t) -> () {
  unsafe { __gmpf_neg(g, f) }
}

/// mpf_sub g = f - e
pub fn mpf_sub(g: mpf_t, f: mpf_t, e: mpf_t) -> () {
  unsafe { __gmpf_sub(g, f, e) }
}

/// mpf_sub_ui g = f - u
pub fn mpf_sub_ui(g: mpf_t, f: mpf_t, u: ui_t) -> () {
  unsafe { __gmpf_sub_ui(g, f, u) }
}

/// mpf_ui_sub g = u - f
pub fn mpf_ui_sub(g: mpf_t, u: ui_t, f: mpf_t) -> () {
  unsafe { __gmpf_ui_sub(g, u, f) }
}

/// mpf_add g = f + e
pub fn mpf_add(g: mpf_t, f: mpf_t, e: mpf_t) -> () {
  unsafe { __gmpf_add(g, f, e) }
}

/// mpf_add_ui g = f + u
pub fn mpf_add_ui(g: mpf_t, f: mpf_t, u: ui_t) -> () {
  unsafe { __gmpf_add_ui(g, f, u) }
}

/// mpf_mul g = f * e
pub fn mpf_mul(g: mpf_t, f: mpf_t, e: mpf_t) -> () {
  unsafe { __gmpf_mul(g, f, e) }
}

/// mpf_mul_ui g = f * u
pub fn mpf_mul_ui(g: mpf_t, f: mpf_t, u: ui_t) -> () {
  unsafe { __gmpf_mul_ui(g, f, u) }
}

/// mpf_mul_2exp g = f * 2**n
pub fn mpf_mul_2exp(g: mpf_t, f: mpf_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpf_mul_2exp(g, f, n) }
}

/// mpf_div g = f / e
pub fn mpf_div(g: mpf_t, f: mpf_t, e: mpf_t) -> () {
  unsafe { __gmpf_div(g, f, e) }
}

/// mpf_div_ui g = f / u
pub fn mpf_div_ui(g: mpf_t, f: mpf_t, u: ui_t) -> () {
  unsafe { __gmpf_div_ui(g, f, u) }
}

/// mpf_ui_div g = u / f
pub fn mpf_ui_div(g: mpf_t, u: ui_t, f: mpf_t) -> () {
  unsafe { __gmpf_ui_div(g, u, f) }
}

/// mpf_div_2exp g = f / 2**n
pub fn mpf_div_2exp(g: mpf_t, f: mpf_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpf_div_2exp(g, f, n) }
}

/// mpf_pow_ui g = f**n
pub fn mpf_pow_ui(g: mpf_t, f: mpf_t, n: ui_t) -> () {
  unsafe { __gmpf_pow_ui(g, f, n) }
}
