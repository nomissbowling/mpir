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
