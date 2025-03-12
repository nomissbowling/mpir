//! mpz
//!

use std::fmt;
use std::error::Error;
use std::collections::HashMap;

use crate::prim::{*, typ::*, gmp::*}; // mpf::*, mpq::*

/// __mpz_struct
// not use #[derive(Clone)]
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

/// impl Drop
impl Drop for __mpz_struct {
  fn drop(&mut self) {
    self.clear()
  }
}

/// impl mpz_s
impl __mpz_struct {
  /// clear
  pub fn clear(&mut self) -> () {
    mpz_clear(self)
  }

  /// init create new instance
  pub fn init() -> Self {
    let mut t = mpz_s::new();
    mpz_init(&mut t);
    t
  }

  /// init2 with prec create new instance
  pub fn init2(n: mp_bitcnt_t) -> Self {
    let mut t = mpz_s::new();
    mpz_init2(&mut t, n);
    t
  }

  /// init_set create new instance
  pub fn init_set(a: mpz_t) -> Self {
    let mut t = mpz_s::new();
    mpz_init_set(&mut t, a);
    t
  }

  /// init_set_ui create new instance
  pub fn init_set_ui(u: ui_t) -> Self {
    let mut t = mpz_s::new();
    mpz_init_set_ui(&mut t, u);
    t
  }

  /// init_set_si create new instance
  pub fn init_set_si(s: si_t) -> Self {
    let mut t = mpz_s::new();
    mpz_init_set_si(&mut t, s);
    t
  }

  /// init_set_d create new instance
  pub fn init_set_d(d: double_t) -> Self {
    let mut t = mpz_s::new();
    mpz_init_set_d(&mut t, d);
    t
  }

  /// init_set_str create new instance
  pub fn init_set_str(s: &str, b: int_t) -> Self {
    let mut t = mpz_s::new();
    mpz_init_set_str(&mut t, s, b);
    t
  }

  /// set self = a
  pub fn set(&mut self, a: mpz_t) -> &mut Self {
    mpz_set(self, a);
    self
  }

  /// set_ui self = u
  pub fn set_ui(&mut self, u: ui_t) -> &mut Self {
    mpz_set_ui(self, u);
    self
  }

  /// set_si self = s
  pub fn set_si(&mut self, s: si_t) -> &mut Self {
    mpz_set_si(self, s);
    self
  }

  /// set_d self = d
  pub fn set_d(&mut self, d: double_t) -> &mut Self {
    mpz_set_d(self, d);
    self
  }

  /// set_str self from str
  pub fn set_str(&mut self, s: &str, b: int_t) -> &mut Self {
    mpz_set_str(self, s, b);
    self
  }

  /// fmtstr
  pub fn fmtstr(&mut self, b: int_t) -> String {
    mpz_get_str(None, b, self).expect("mpz fmtstr")
  }

  /// cmp
  pub fn cmp(&mut self, b: mpz_t) -> int_t {
    mpz_cmp(self, b)
  }

  /// cmp_d
  pub fn cmp_d(&mut self, d: double_t) -> int_t {
    mpz_cmp_d(self, d)
  }

  /// cmp_ui
  pub fn cmp_ui(&mut self, u: ui_t) -> int_t {
    mpz_cmp_ui(self, u)
  }

  /// cmp_si
  pub fn cmp_si(&mut self, s: si_t) -> int_t {
    mpz_cmp_si(self, s)
  }

  /// cmpabs
  pub fn cmpabs(&mut self, b: mpz_t) -> int_t {
    mpz_cmpabs(self, b)
  }

  /// cmpabs_d
  pub fn cmpabs_d(&mut self, d: double_t) -> int_t {
    mpz_cmpabs_d(self, d)
  }

  /// cmpabs_ui
  pub fn cmpabs_ui(&mut self, u: ui_t) -> int_t {
    mpz_cmpabs_ui(self, u)
  }

  /// sgn
  pub fn sgn(&mut self) -> int_t {
    mpz_sgn(self)
  }

  /// fac_ui create new instance
  pub fn fac_ui(n: ui_t) -> Self {
    let mut t = mpz_s::init_set_ui(1);
    mpz_fac_ui(&mut t, n);
    t
  }

  /// fac2_ui create new instance
  pub fn fac2_ui(n: ui_t) -> Self {
    let mut t = mpz_s::init_set_ui(1);
    mpz_2fac_ui(&mut t, n);
    t
  }

  /// mfac_uiui create new instance
  pub fn mfac_uiui(n: ui_t, m: ui_t) -> Self {
    let mut t = mpz_s::init_set_ui(1);
    mpz_mfac_uiui(&mut t, n, m);
    t
  }

  /// abs create new instance
  pub fn abs(&mut self) -> Self {
    let mut t = mpz_s::new();
    mpz_abs(&mut t, self);
    t
  }

  /// neg create new instance
  pub fn neg(&mut self) -> Self {
    let mut t = mpz_s::new();
    mpz_neg(&mut t, self);
    t
  }

  /// sub self -= b
  pub fn sub(&mut self, b: mpz_t) -> &mut Self {
    let t = &mut mpz_s::init_set(self);
    mpz_sub(self, t, b);
    self
  }

  /// sub_ui self -= u
  pub fn sub_ui(&mut self, u: ui_t) -> &mut Self {
    let t = &mut mpz_s::init_set(self);
    mpz_sub_ui(self, t, u);
    self
  }

  /// ui_sub self = u - self
  pub fn ui_sub(&mut self, u: ui_t) -> &mut Self {
    let t = &mut mpz_s::init_set(self);
    mpz_ui_sub(self, u, t);
    self
  }

  /// submul self -= a * b
  pub fn submul(&mut self, a: mpz_t, b: mpz_t) -> &mut Self {
    mpz_submul(self, a, b);
    self
  }

  /// submul_ui self -= a * u
  pub fn submul_ui(&mut self, a: mpz_t, u: ui_t) -> &mut Self {
    mpz_submul_ui(self, a, u);
    self
  }

  /// add self += b
  pub fn add(&mut self, b: mpz_t) -> &mut Self {
    let t = &mut mpz_s::init_set(self);
    mpz_add(self, t, b);
    self
  }

  /// add_ui self += u
  pub fn add_ui(&mut self, u: ui_t) -> &mut Self {
    let t = &mut mpz_s::init_set(self);
    mpz_add_ui(self, t, u);
    self
  }

  /// addmul self += a * b
  pub fn addmul(&mut self, a: mpz_t, b: mpz_t) -> &mut Self {
    mpz_addmul(self, a, b);
    self
  }

  /// addmul_ui self += a * u
  pub fn addmul_ui(&mut self, a: mpz_t, u: ui_t) -> &mut Self {
    mpz_addmul_ui(self, a, u);
    self
  }

  /// mul self *= b
  pub fn mul(&mut self, b: mpz_t) -> &mut Self {
    let t = &mut mpz_s::init_set(self);
    mpz_mul(self, t, b);
    self
  }

  /// mul_ui self *= u
  pub fn mul_ui(&mut self, u: ui_t) -> &mut Self {
    let t = &mut mpz_s::init_set(self);
    mpz_mul_ui(self, t, u);
    self
  }

  /// mul_si self *= s
  pub fn mul_si(&mut self, s: si_t) -> &mut Self {
    let t = &mut mpz_s::init_set(self);
    mpz_mul_si(self, t, s);
    self
  }

  /// mul_2exp self *= 2**n
  pub fn mul_2exp(&mut self, n: mp_bitcnt_t) -> &mut Self {
    let t = &mut mpz_s::init_set(self);
    mpz_mul_2exp(self, t, n);
    self
  }


  /// powwm_sec (a**n) mod m ***required n &gt; 0 and m is odd*** create new instance
  pub fn powm_sec(a: mpz_t, n: mpz_t, m: mpz_t) -> Self {
    let mut t = mpz_s::new();
    mpz_powm_sec(&mut t, a, n, m);
    t
  }

  /// powm (a**n) mod m ***n &lt; 0 when exists inv a**-1 mod m*** create new instance
  pub fn powm(a: mpz_t, n: mpz_t, m: mpz_t) -> Self {
    let mut t = mpz_s::new();
    mpz_powm(&mut t, a, n, m);
    t
  }

  /// powm_ui (a**n) mod m create new instance
  pub fn powm_ui(a: mpz_t, n: ui_t, m: mpz_t) -> Self {
    let mut t = mpz_s::new();
    mpz_powm_ui(&mut t, a, n, m);
    t
  }

  /// pow_ui a**n create new instance
  pub fn pow_ui(a: mpz_t, n: ui_t) -> Self {
    let mut t = mpz_s::new();
    mpz_pow_ui(&mut t, a, n);
    t
  }

  /// ui_pow_ui a**n create new instance
  pub fn ui_pow_ui(a: ui_t, n: ui_t) -> Self {
    let mut t = mpz_s::new();
    mpz_ui_pow_ui(&mut t, a, n);
    t
  }

  /// fact create new instance (slow without cache)
  pub fn fact(n: ui_t) -> Self {
    let mut t = mpz_s::init_set_ui(1);
    (1..=n).into_iter().for_each(|i| { t.mul_ui(i); });
    t
  }

  /// fact cached
  pub fn fact_cached(n: ui_t, m: &mut HashMap<ui_t, mpz_s>) -> Self {
/*  // duplex mutable borrow m
    let e = m.entry(n).or_insert(if n == 0 { mpz_s::init_set_ui(1) }
      else { let mut t = mpz_s::fact_cached(n - 1, m); t.mul_ui(n); t }
    );
    mpz_s::init_set(e) // as clone
*/
    // early return to avoid duplex mutable borrow m
    if let Some(e) = m.get_mut(&n) { return mpz_s::init_set(e); } // as clone
    let mut e = if n == 0 { mpz_s::init_set_ui(1) }
      else { let mut t = mpz_s::fact_cached(n - 1, m); t.mul_ui(n); t };
    m.insert(n, mpz_s::init_set(&mut e)); // as clone
    e
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
//    write!(f, "{}", self.fmtstr(10)) // cannot be borrowed as mutable
    write!(f, "{}", mpz_get_str(None, 10, self).expect("mpz str"))
  }
}

/// mpz_s
#[allow(non_camel_case_types)]
pub type mpz_s = __mpz_struct; // [__mpz_struct; 1]
/// mpz_t
#[allow(non_camel_case_types)]
pub type mpz_t<'a> = &'a mut mpz_s; // *mut mpz_s

/// mpz_clears
pub fn mpz_clears(va: &mut Vec<mpz_t>) -> () {
  va.iter_mut().for_each(|a| {
    unsafe { __gmpz_clear(*a) } // not use __gmpz_clears
  })
}

/// mpz_clear
pub fn mpz_clear(a: mpz_t) -> () {
  unsafe { __gmpz_clear(a) }
}

/// mpz_inits
pub fn mpz_inits(va: &mut Vec<mpz_t>) -> () {
  va.iter_mut().for_each(|a| {
    unsafe { __gmpz_init(*a) } // not use __gmpz_inits
  })
}

/// mpz_init
pub fn mpz_init(a: mpz_t) -> () {
  unsafe { __gmpz_init(a) }
}

/// mpz_init2
pub fn mpz_init2(a: mpz_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpz_init2(a, n) }
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

/// mpz_cmp
pub fn mpz_cmp(a: mpz_t, b: mpz_t) -> int_t {
  unsafe { __gmpz_cmp(a, b) }
}

/// mpz_cmp_d
pub fn mpz_cmp_d(a: mpz_t, d: double_t) -> int_t {
  unsafe { __gmpz_cmp_d(a, d) }
}

/// mpz_cmp_ui
pub fn mpz_cmp_ui(a: mpz_t, u: ui_t) -> int_t {
  unsafe { __gmpz_cmp_ui(a, u) }
}

/// mpz_cmp_si
pub fn mpz_cmp_si(a: mpz_t, s: si_t) -> int_t {
  unsafe { __gmpz_cmp_si(a, s) }
}

/// mpz_cmpabs
pub fn mpz_cmpabs(a: mpz_t, b: mpz_t) -> int_t {
  unsafe { __gmpz_cmpabs(a, b) }
}

/// mpz_cmpabs_d
pub fn mpz_cmpabs_d(a: mpz_t, d: double_t) -> int_t {
  unsafe { __gmpz_cmpabs_d(a, d) }
}

/// mpz_cmpabs_ui
pub fn mpz_cmpabs_ui(a: mpz_t, u: ui_t) -> int_t {
  unsafe { __gmpz_cmpabs_ui(a, u) }
}

/// mpz_sgn
pub fn mpz_sgn(a: mpz_t) -> int_t {
//  unsafe { __gmpz_sgn(a) }
  let t = a._mp_size;
  if t < 0 { -1 } else { if t > 0 { 1 } else { 0 } }
}

/// mpz_fac_ui c = n!
pub fn mpz_fac_ui(c: mpz_t, n: ui_t) -> () {
  unsafe { __gmpz_fac_ui(c, n) }
}

/// mpz_2fac_ui c = n!!
pub fn mpz_2fac_ui(c: mpz_t, n: ui_t) -> () {
  unsafe { __gmpz_2fac_ui(c, n) }
}

/// mpz_mfac_uiui c = n! ** m
pub fn mpz_mfac_uiui(c: mpz_t, n: ui_t, m: ui_t) -> () {
  unsafe { __gmpz_mfac_uiui(c, n, m) }
}

/// mpz_abs
pub fn mpz_abs(c: mpz_t, a: mpz_t) -> () {
  unsafe { __gmpz_abs(c, a) }
}

/// mpz_neg
pub fn mpz_neg(c: mpz_t, a: mpz_t) -> () {
  unsafe { __gmpz_neg(c, a) }
}

/// mpz_sub c = a - b
pub fn mpz_sub(c: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_sub(c, a, b) }
}

/// mpz_sub_ui c = a - u
pub fn mpz_sub_ui(c: mpz_t, a: mpz_t, u: ui_t) -> () {
  unsafe { __gmpz_sub_ui(c, a, u) }
}

/// mpz_ui_sub c = u - a
pub fn mpz_ui_sub(c: mpz_t, u: ui_t, a: mpz_t) -> () {
  unsafe { __gmpz_ui_sub(c, u, a) }
}

/// mpz_submul c -= a * b
pub fn mpz_submul(c: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_submul(c, a, b) }
}

/// mpz_submul_ui c -= a * u
pub fn mpz_submul_ui(c: mpz_t, a: mpz_t, u: ui_t) -> () {
  unsafe { __gmpz_submul_ui(c, a, u) }
}

/// mpz_add c = a + b
pub fn mpz_add(c: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_add(c, a, b) }
}

/// mpz_add_ui c = a + u
pub fn mpz_add_ui(c: mpz_t, a: mpz_t, u: ui_t) -> () {
  unsafe { __gmpz_add_ui(c, a, u) }
}

/// mpz_addmul c += a * b
pub fn mpz_addmul(c: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_addmul(c, a, b) }
}

/// mpz_addmul_ui c += a * u
pub fn mpz_addmul_ui(c: mpz_t, a: mpz_t, u: ui_t) -> () {
  unsafe { __gmpz_addmul_ui(c, a, u) }
}

/// mpz_mul c = a * b
pub fn mpz_mul(c: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_mul(c, a, b) }
}

/// mpz_mul_ui c = a * u
pub fn mpz_mul_ui(c: mpz_t, a: mpz_t, u: ui_t) -> () {
  unsafe { __gmpz_mul_ui(c, a, u) }
}

/// mpz_mul_si c = a * s
pub fn mpz_mul_si(c: mpz_t, a: mpz_t, s: si_t) -> () {
  unsafe { __gmpz_mul_si(c, a, s) }
}

/// mpz_mul_2exp c = a * 2**n
pub fn mpz_mul_2exp(c: mpz_t, a: mpz_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpz_mul_2exp(c, a, n) }
}

/// mpz_powwm_sec c = (a**n) mod m ***required n &gt; 0 and m is odd***
pub fn mpz_powm_sec(c: mpz_t, a: mpz_t, n: mpz_t, m: mpz_t) -> () {
  unsafe { __gmpz_powm_sec(c, a, n, m) }
}

/// mpz_powm c = (a**n) mod m ***n &lt; 0 when exists inv a**-1 mod m***
pub fn mpz_powm(c: mpz_t, a: mpz_t, n: mpz_t, m: mpz_t) -> () {
  unsafe { __gmpz_powm(c, a, n, m) }
}

/// mpz_powm_ui c = (a**n) mod m
pub fn mpz_powm_ui(c: mpz_t, a: mpz_t, n: ui_t, m: mpz_t) -> () {
  unsafe { __gmpz_powm_ui(c, a, n, m) }
}

/// mpz_pow_ui c == a**n
pub fn mpz_pow_ui(c: mpz_t, a: mpz_t, n: ui_t) -> () {
  unsafe { __gmpz_pow_ui(c, a, n) }
}

/// mpz_ui_pow_ui c = a**n
pub fn mpz_ui_pow_ui(c: mpz_t, a: ui_t, n: ui_t) -> () {
  unsafe { __gmpz_ui_pow_ui(c, a, n) }
}
