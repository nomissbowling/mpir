//! mpf
//!

use std::fmt;
use std::error::Error;
use std::mem::MaybeUninit;

use crate::prim::{*, typ::*, mpz::*, mpq::*, randstate::*, gmp::*};

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
  /// This is acrobatic way, new() MUST be called with mpf_s::init*()
  #[inline]
  fn new() -> Self {
/*
    __mpf_struct {
      _mp_prec: 0,
      _mp_size: 0,
      _mp_exp: 0,
      _mp_d: 0 as *mut mp_limb_t
    }
*/
unsafe {
    let prec = MaybeUninit::<int_t>::uninit();
    let sz = MaybeUninit::<int_t>::uninit();
    let exp = MaybeUninit::<mp_exp_t>::uninit();
    let d = MaybeUninit::<*mut mp_limb_t>::uninit();
    __mpf_struct {
      _mp_prec: prec.assume_init(),
      _mp_size: sz.assume_init(),
      _mp_exp: exp.assume_init(),
      _mp_d: d.assume_init()
    }
}
  }
}

/// impl Drop
impl Drop for __mpf_struct {
  fn drop(&mut self) {
    self.clear()
  }
}

/// impl AsPtr
impl AsPtr for __mpf_struct {}

/// impl AsPtrMut
impl AsPtrMut for __mpf_struct {}

/// impl mpf_s
impl __mpf_struct {
  /// clear
  #[inline]
  pub fn clear(&mut self) -> () {
    mpf_clear(self)
  }

  /// init create new instance
  #[inline]
  pub fn init() -> Self {
    let mut t = mpf_s::new();
    mpf_init(&mut t);
    t
  }

  /// init2 with prec create new instance
  #[inline]
  pub fn init2(n: mp_bitcnt_t) -> Self {
    let mut t = mpf_s::new();
    mpf_init2(&mut t, n);
    t
  }

  /// init_set create new instance
  #[inline]
  pub fn init_set(f: mpf_r) -> Self {
    let mut t = mpf_s::new();
    mpf_init_set(&mut t, f);
    t
  }

  /// init_set_ui create new instance
  #[inline]
  pub fn init_set_ui(u: ui_t) -> Self {
    let mut t = mpf_s::new();
    mpf_init_set_ui(&mut t, u);
    t
  }

  /// init_set_si create new instance
  #[inline]
  pub fn init_set_si(s: si_t) -> Self {
    let mut t = mpf_s::new();
    mpf_init_set_si(&mut t, s);
    t
  }

  /// init_set_d create new instance
  #[inline]
  pub fn init_set_d(d: double_t) -> Self {
    let mut t = mpf_s::new();
    mpf_init_set_d(&mut t, d);
    t
  }

  /// init_set_str create new instance
  #[inline]
  pub fn init_set_str(s: &str, b: int_t) -> Self {
    let mut t = mpf_s::new();
    mpf_init_set_str(&mut t, s, b);
    t
  }

  /// fmtstr
  #[inline]
  pub fn fmtstr(&self, b: int_t, d: mp_size_t) -> String {
    mpf_get_fmtstr(self, b, d).expect("mpf fmtstr")
  }

  /// set self = f
  #[inline]
  pub fn set(&mut self, f: mpf_r) -> &mut Self {
    mpf_set(self, f);
    self
  }

  /// set_ui self = u
  #[inline]
  pub fn set_ui(&mut self, u: ui_t) -> &mut Self {
    mpf_set_ui(self, u);
    self
  }

  /// set_si self = s
  #[inline]
  pub fn set_si(&mut self, s: si_t) -> &mut Self {
    mpf_set_si(self, s);
    self
  }

  /// set_d self = d
  #[inline]
  pub fn set_d(&mut self, d: double_t) -> &mut Self {
    mpf_set_d(self, d);
    self
  }

  /// set_z self = a
  #[inline]
  pub fn set_z(&mut self, a: mpz_r) -> &mut Self {
    mpf_set_z(self, a);
    self
  }

  /// set_str self from str
  #[inline]
  pub fn set_str(&mut self, s: &str, b: int_t) -> &mut Self {
    mpf_set_str(self, s, b);
    self
  }

  /// get_d (loss of digits)
  #[inline]
  pub fn get_d(&self) -> double_t {
    mpf_get_d(self)
  }

  /// get_ui (loss of digits)
  #[inline]
  pub fn get_ui(&self) -> ui_t {
    mpf_get_ui(self)
  }

  /// get_si (loss of digits)
  #[inline]
  pub fn get_si(&self) -> si_t {
    mpf_get_si(self)
  }

  /// get_d_2exp (loss of digits)
  #[inline]
  pub fn get_d_2exp(&self) -> (double_t, si_t) {
    let mut e: si_t = 0;
    let d = mpf_get_d_2exp(&mut e, self);
    (d, e)
  }

  /// cmp
  #[inline]
  pub fn cmp(&self, g: mpf_r) -> int_t {
    mpf_cmp(self, g)
  }

  /// cmp_d
  #[inline]
  pub fn cmp_d(&self, d: double_t) -> int_t {
    mpf_cmp_d(self, d)
  }

  /// cmp_ui
  #[inline]
  pub fn cmp_ui(&self, u: ui_t) -> int_t {
    mpf_cmp_ui(self, u)
  }

  /// cmp_si
  #[inline]
  pub fn cmp_si(&self, s: si_t) -> int_t {
    mpf_cmp_si(self, s)
  }

  /// cmp_z
  #[inline]
  pub fn cmp_z(&self, a: mpz_r) -> int_t {
    mpf_cmp_z(self, a)
  }

  /// eq ***mathematically ill-defined and should not be used***
  #[inline]
  pub fn eq(&self, g: mpf_r, n: mp_bitcnt_t) -> int_t {
    mpf_eq(self, g, n)
  }

  /// sgn
  #[inline]
  pub fn sgn(&self) -> int_t {
    mpf_sgn(self)
  }

  /// reldiff returns abs(self - e) / self create new instance
  #[inline]
  pub fn reldiff(&self, e: mpf_r) -> Self {
    let mut t = mpf_s::init();
    mpf_reldiff(&mut t, self, e);
    t
  }

  /// sqrt create new instance
  #[inline]
  pub fn sqrt(&self) -> Self {
    let mut t = mpf_s::init();
    mpf_sqrt(&mut t, self);
    t
  }

  /// sqrt_ui create new instance
  #[inline]
  pub fn sqrt_ui(u: ui_t) -> Self {
    let mut t = mpf_s::init();
    mpf_sqrt_ui(&mut t, u);
    t
  }

  /// abs create new instance
  #[inline]
  pub fn abs(&self) -> Self {
    let mut t = mpf_s::init();
    mpf_abs(&mut t, self);
    t
  }

  /// neg create new instance
  #[inline]
  pub fn neg(&self) -> Self {
    let mut t = mpf_s::init();
    mpf_neg(&mut t, self);
    t
  }

  /// sub self -= e
  #[inline]
  pub fn sub(&mut self, e: mpf_r) -> &mut Self {
    mpf_sub(self, &mpf_s::init_set(self), e);
    self
  }

  /// sub_ui self -= u
  #[inline]
  pub fn sub_ui(&mut self, u: ui_t) -> &mut Self {
    mpf_sub_ui(self, &mpf_s::init_set(self), u);
    self
  }

  /// ui_sub self = u - self
  #[inline]
  pub fn ui_sub(&mut self, u: ui_t) -> &mut Self {
    mpf_ui_sub(self, u, &mpf_s::init_set(self));
    self
  }

  /// add self += e
  #[inline]
  pub fn add(&mut self, e: mpf_r) -> &mut Self {
    mpf_add(self, &mpf_s::init_set(self), e);
    self
  }

  /// add_ui self += u
  #[inline]
  pub fn add_ui(&mut self, u: ui_t) -> &mut Self {
    mpf_add_ui(self, &mpf_s::init_set(self), u);
    self
  }

  /// mul self *= e
  #[inline]
  pub fn mul(&mut self, e: mpf_r) -> &mut Self {
    mpf_mul(self, &mpf_s::init_set(self), e);
    self
  }

  /// mul_ui self *= u
  #[inline]
  pub fn mul_ui(&mut self, u: ui_t) -> &mut Self {
    mpf_mul_ui(self, &mpf_s::init_set(self), u);
    self
  }

  /// mul_2exp self *= 2**n
  #[inline]
  pub fn mul_2exp(&mut self, n: mp_bitcnt_t) -> &mut Self {
    mpf_mul_2exp(self, &mpf_s::init_set(self), n);
    self
  }

  /// div self /= e
  #[inline]
  pub fn div(&mut self, e: mpf_r) -> &mut Self {
    mpf_div(self, &mpf_s::init_set(self), e);
    self
  }

  /// div_ui self /= u
  #[inline]
  pub fn div_ui(&mut self, u: ui_t) -> &mut Self {
    mpf_div_ui(self, &mpf_s::init_set(self), u);
    self
  }

  /// ui_div self = u / self
  #[inline]
  pub fn ui_div(&mut self, u: ui_t) -> &mut Self {
    mpf_ui_div(self, u, &mpf_s::init_set(self));
    self
  }

  /// div_2exp self /= 2**n
  #[inline]
  pub fn div_2exp(&mut self, n: mp_bitcnt_t) -> &mut Self {
    mpf_div_2exp(self, &mpf_s::init_set(self), n);
    self
  }

  /// pow_ui f**n create new instance
  #[inline]
  pub fn pow_ui(f: mpf_r, n: ui_t) -> Self {
    let mut t = mpf_s::init(); // ***NEVER*** use new()
    mpf_pow_ui(&mut t, f, n);
    t
  }

  /// ceil create new instance
  #[inline]
  pub fn ceil(&self) -> Self {
    let mut t = mpf_s::init();
    mpf_ceil(&mut t, self);
    t
  }

  /// floor create new instance
  #[inline]
  pub fn floor(&self) -> Self {
    let mut t = mpf_s::init();
    mpf_floor(&mut t, self);
    t
  }

  /// trunc create new instance
  #[inline]
  pub fn trunc(&self) -> Self {
    let mut t = mpf_s::init();
    mpf_trunc(&mut t, self);
    t
  }

  /// integer_p
  #[inline]
  pub fn integer_p(&self) -> bool {
    mpf_integer_p(self)
  }

  /// fits_ulong_p
  #[inline]
  pub fn fits_ulong_p(&self) -> bool {
    mpf_fits_ulong_p(self)
  }

  /// fits_slong_p
  #[inline]
  pub fn fits_slong_p(&self) -> bool {
    mpf_fits_slong_p(self)
  }

  /// fits_uint_p
  #[inline]
  pub fn fits_uint_p(&self) -> bool {
    mpf_fits_uint_p(self)
  }

  /// fits_sint_p
  #[inline]
  pub fn fits_sint_p(&self) -> bool {
    mpf_fits_sint_p(self)
  }

  /// fits_ushort_p
  #[inline]
  pub fn fits_ushort_p(&self) -> bool {
    mpf_fits_ushort_p(self)
  }

  /// fits_sshort_p
  #[inline]
  pub fn fits_sshort_p(&self) -> bool {
    mpf_fits_sshort_p(self)
  }

  /// urandomb (must init random state before) create new instance
  #[inline]
  pub fn urandomb(state: randstate_t, nbits: mp_bitcnt_t) -> Self {
    let mut t = mpf_s::init();
    mpf_urandomb(&mut t, state, nbits);
    t
  }

  /// random2 create new instance
  #[inline]
  pub fn random2(max_size: mp_size_t, e: mp_exp_t) -> Self {
    let mut t = mpf_s::init();
    mpf_random2(&mut t, max_size, e);
    t
  }

  /// get_prec
  #[inline]
  pub fn get_prec(&self) -> mp_bitcnt_t {
    mpf_get_prec(self)
  }

  /// set_prec
  #[inline]
  pub fn set_prec(&mut self, n: mp_bitcnt_t) -> () {
    mpf_set_prec(self, n)
  }

  /// set_prec_raw
  #[inline]
  pub fn set_prec_raw(&mut self, n: mp_bitcnt_t) -> () {
    mpf_set_prec_raw(self, n)
  }

  /// calc_bits_from_digits
  pub fn calc_bits_from_digits(d: mp_size_t) -> mp_bitcnt_t {
    (10f64.log2() * (d + 1) as f64) as mp_bitcnt_t
  }

  /// calc_digits_from_bits
  pub fn calc_digits_from_bits(n: mp_bitcnt_t) -> mp_size_t {
    (n as f64 / 10f64.log2()) as mp_size_t
  }

  /// init_set_z create new instance
  #[inline]
  pub fn init_set_z(a: mpz_r) -> Self {
    let mut t = mpf_s::init();
    mpf_set_z(&mut t, a);
    t
  }

  /// init_set_q create new instance
  #[inline]
  pub fn init_set_q(q: mpq_r) -> Self {
    mpf_s::init_set_z(q.numref()) / mpf_s::init_set_z(q.denref())
  }

  /// inv create new instance
  #[inline]
  pub fn inv(&self) -> Self {
    let mut t = mpf_s::init();
    mpf_ui_div(&mut t, 1, self);
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
    write!(f, "{}", self.fmtstr(10, 20))
  }
}

/// mpf_s
#[allow(non_camel_case_types)]
pub type mpf_s = __mpf_struct; // [__mpf_struct; 1]
/// mpf_t
#[allow(non_camel_case_types)]
pub type mpf_t<'a> = &'a mut mpf_s; // *mut mpf_s
/// mpf_r
#[allow(non_camel_case_types)]
pub type mpf_r<'a> = &'a mpf_s; // *const mpf_s

/// mpf_clears
pub fn mpf_clears(vf: &mut Vec<mpf_t>) -> () {
  vf.iter_mut().for_each(|f| {
    unsafe { __gmpf_clear(*f) } // not use __gmpf_clears
  })
}

/// mpf_clear
#[inline]
pub fn mpf_clear(f: mpf_t) -> () {
  unsafe { __gmpf_clear(f) }
}

/// mpf_inits
pub fn mpf_inits(vf: &mut Vec<mpf_t>) -> () {
  vf.iter_mut().for_each(|f| {
    unsafe { __gmpf_init(*f) } // not use __gmpf_inits
  })
}

/// mpf_init
#[inline]
pub fn mpf_init(f: mpf_t) -> () {
  unsafe { __gmpf_init(f) }
}

/// mpf_init2
#[inline]
pub fn mpf_init2(f: mpf_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpf_init2(f, n) }
}

/// mpf_init_set
#[inline]
pub fn mpf_init_set(f: mpf_t, g: mpf_r) -> () {
  unsafe { __gmpf_init_set(f, g) }
}

/// mpf_init_set_ui
#[inline]
pub fn mpf_init_set_ui(f: mpf_t, u: ui_t) -> () {
  unsafe { __gmpf_init_set_ui(f, u) }
}

/// mpf_init_set_si
#[inline]
pub fn mpf_init_set_si(f: mpf_t, s: si_t) -> () {
  unsafe { __gmpf_init_set_si(f, s) }
}

/// mpf_init_set_d
#[inline]
pub fn mpf_init_set_d(f: mpf_t, d: double_t) -> () {
  unsafe { __gmpf_init_set_d(f, d) }
}

/// mpf_init_set_str
#[inline]
pub fn mpf_init_set_str(f: mpf_t, s: &str, b: int_t) -> () {
  mpf_init_set_str_u8z(f, to_u8z!(s), b)
}

/// mpf_init_set_str_u8z
#[inline]
pub fn mpf_init_set_str_u8z(f: mpf_t, s: &[u8], b: int_t) -> () {
  unsafe { __gmpf_init_set_str(f, s as *const [u8] as *const u8, b) }
}

/// mpf_set
#[inline]
pub fn mpf_set(f: mpf_t, g: mpf_r) -> () {
  unsafe { __gmpf_set(f, g) }
}

/// mpf_set_ui
#[inline]
pub fn mpf_set_ui(f: mpf_t, u: ui_t) -> () {
  unsafe { __gmpf_set_ui(f, u) }
}

/// mpf_set_si
#[inline]
pub fn mpf_set_si(f: mpf_t, s: si_t) -> () {
  unsafe { __gmpf_set_si(f, s) }
}

/// mpf_set_d
#[inline]
pub fn mpf_set_d(f: mpf_t, d: double_t) -> () {
  unsafe { __gmpf_set_d(f, d) }
}

/// mpf_set_z
#[inline]
pub fn mpf_set_z(f: mpf_t, a: mpz_r) -> () {
  unsafe { __gmpf_set_z(f, a) }
}

/// mpf_set_str
#[inline]
pub fn mpf_set_str(f: mpf_t, s: &str, b: int_t) -> () {
  mpf_set_str_u8z(f, to_u8z!(s), b)
}

/// mpf_set_str_u8z
#[inline]
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

/// mpf_get_d
#[inline]
pub fn mpf_get_d(f: mpf_r) -> double_t {
  unsafe { __gmpf_get_d(f) }
}

/// mpf_get_ui
#[inline]
pub fn mpf_get_ui(f: mpf_r) -> ui_t {
  unsafe { __gmpf_get_ui(f) }
}

/// mpf_get_si
#[inline]
pub fn mpf_get_si(f: mpf_r) -> si_t {
  unsafe { __gmpf_get_si(f) }
}

/// mpf_get_d_2exp
#[inline]
pub fn mpf_get_d_2exp(e: &mut si_t, f: mpf_r) -> double_t {
  unsafe { __gmpf_get_d_2exp(e, f) }
}

/// mpf_cmp
#[inline]
pub fn mpf_cmp(f: mpf_r, g: mpf_r) -> int_t {
  unsafe { __gmpf_cmp(f, g) }
}

/// mpf_cmp_d
#[inline]
pub fn mpf_cmp_d(f: mpf_r, d: double_t) -> int_t {
  unsafe { __gmpf_cmp_d(f, d) }
}

/// mpf_cmp_ui
#[inline]
pub fn mpf_cmp_ui(f: mpf_r, u: ui_t) -> int_t {
  unsafe { __gmpf_cmp_ui(f, u) }
}

/// mpf_cmp_si
#[inline]
pub fn mpf_cmp_si(f: mpf_r, s: si_t) -> int_t {
  unsafe { __gmpf_cmp_si(f, s) }
}

/// mpf_cmp_z
#[inline]
pub fn mpf_cmp_z(f: mpf_r, a: mpz_r) -> int_t {
  unsafe { __gmpf_cmp_z(f, a) }
}

/// mpf_eq ***mathematically ill-defined and should not be used***
#[inline]
pub fn mpf_eq(f: mpf_r, g: mpf_r, n: mp_bitcnt_t) -> int_t {
  unsafe { __gmpf_eq(f, g, n) }
}

/// mpf_sgn
#[inline]
pub fn mpf_sgn(f: mpf_r) -> int_t {
//  unsafe { __gmpf_sgn(f) }
  let t = f._mp_size;
  if t < 0 { -1 } else { if t > 0 { 1 } else { 0 } }
}

/// mpf_reldiff
#[inline]
pub fn mpf_reldiff(g: mpf_t, f: mpf_r, e: mpf_r) -> () {
  unsafe { __gmpf_reldiff(g, f, e) }
}

/// mpf_sqrt
#[inline]
pub fn mpf_sqrt(g: mpf_t, f: mpf_r) -> () {
  unsafe { __gmpf_sqrt(g, f) }
}

/// mpf_sqrt_ui
#[inline]
pub fn mpf_sqrt_ui(g: mpf_t, u: ui_t) -> () {
  unsafe { __gmpf_sqrt_ui(g, u) }
}

/// mpf_abs
#[inline]
pub fn mpf_abs(g: mpf_t, f: mpf_r) -> () {
  unsafe { __gmpf_abs(g, f) }
}

/// mpf_neg
#[inline]
pub fn mpf_neg(g: mpf_t, f: mpf_r) -> () {
  unsafe { __gmpf_neg(g, f) }
}

/// mpf_sub g = f - e
#[inline]
pub fn mpf_sub(g: mpf_t, f: mpf_r, e: mpf_r) -> () {
  unsafe { __gmpf_sub(g, f, e) }
}

/// mpf_sub_ui g = f - u
#[inline]
pub fn mpf_sub_ui(g: mpf_t, f: mpf_r, u: ui_t) -> () {
  unsafe { __gmpf_sub_ui(g, f, u) }
}

/// mpf_ui_sub g = u - f
#[inline]
pub fn mpf_ui_sub(g: mpf_t, u: ui_t, f: mpf_r) -> () {
  unsafe { __gmpf_ui_sub(g, u, f) }
}

/// mpf_add g = f + e
#[inline]
pub fn mpf_add(g: mpf_t, f: mpf_r, e: mpf_r) -> () {
  unsafe { __gmpf_add(g, f, e) }
}

/// mpf_add_ui g = f + u
#[inline]
pub fn mpf_add_ui(g: mpf_t, f: mpf_r, u: ui_t) -> () {
  unsafe { __gmpf_add_ui(g, f, u) }
}

/// mpf_mul g = f * e
#[inline]
pub fn mpf_mul(g: mpf_t, f: mpf_r, e: mpf_r) -> () {
  unsafe { __gmpf_mul(g, f, e) }
}

/// mpf_mul_ui g = f * u
#[inline]
pub fn mpf_mul_ui(g: mpf_t, f: mpf_r, u: ui_t) -> () {
  unsafe { __gmpf_mul_ui(g, f, u) }
}

/// mpf_mul_2exp g = f * 2**n
#[inline]
pub fn mpf_mul_2exp(g: mpf_t, f: mpf_r, n: mp_bitcnt_t) -> () {
  unsafe { __gmpf_mul_2exp(g, f, n) }
}

/// mpf_div g = f / e
#[inline]
pub fn mpf_div(g: mpf_t, f: mpf_r, e: mpf_r) -> () {
  unsafe { __gmpf_div(g, f, e) }
}

/// mpf_div_ui g = f / u
#[inline]
pub fn mpf_div_ui(g: mpf_t, f: mpf_r, u: ui_t) -> () {
  unsafe { __gmpf_div_ui(g, f, u) }
}

/// mpf_ui_div g = u / f
#[inline]
pub fn mpf_ui_div(g: mpf_t, u: ui_t, f: mpf_r) -> () {
  unsafe { __gmpf_ui_div(g, u, f) }
}

/// mpf_div_2exp g = f / 2**n
#[inline]
pub fn mpf_div_2exp(g: mpf_t, f: mpf_r, n: mp_bitcnt_t) -> () {
  unsafe { __gmpf_div_2exp(g, f, n) }
}

/// mpf_pow_ui g = f**n
#[inline]
pub fn mpf_pow_ui(g: mpf_t, f: mpf_r, n: ui_t) -> () {
  unsafe { __gmpf_pow_ui(g, f, n) }
}

/// mpf_ceil
#[inline]
pub fn mpf_ceil(g: mpf_t, f: mpf_r) -> () {
  unsafe { __gmpf_ceil(g, f) }
}

/// mpf_floor
#[inline]
pub fn mpf_floor(g: mpf_t, f: mpf_r) -> () {
  unsafe { __gmpf_floor(g, f) }
}

/// mpf_trunc
#[inline]
pub fn mpf_trunc(g: mpf_t, f: mpf_r) -> () {
  unsafe { __gmpf_trunc(g, f) }
}

/// mpf_integer_p
#[inline]
pub fn mpf_integer_p(f: mpf_r) -> bool {
  unsafe { __gmpf_integer_p(f) != 0 }
}

/// mpf_fits_ulong_p
#[inline]
pub fn mpf_fits_ulong_p(f: mpf_r) -> bool {
  unsafe { __gmpf_fits_ulong_p(f) != 0 }
}

/// mpf_fits_slong_p
#[inline]
pub fn mpf_fits_slong_p(f: mpf_r) -> bool {
  unsafe { __gmpf_fits_slong_p(f) != 0 }
}

/// mpf_fits_uint_p
#[inline]
pub fn mpf_fits_uint_p(f: mpf_r) -> bool {
  unsafe { __gmpf_fits_uint_p(f) != 0 }
}

/// mpf_fits_sint_p
#[inline]
pub fn mpf_fits_sint_p(f: mpf_r) -> bool {
  unsafe { __gmpf_fits_sint_p(f) != 0 }
}

/// mpf_fits_ushort_p
#[inline]
pub fn mpf_fits_ushort_p(f: mpf_r) -> bool {
  unsafe { __gmpf_fits_ushort_p(f) != 0 }
}

/// mpf_fits_sshort_p
#[inline]
pub fn mpf_fits_sshort_p(f: mpf_r) -> bool {
  unsafe { __gmpf_fits_sshort_p(f) != 0 }
}

/// mpf_urandomb (must init random state before)
#[inline]
pub fn mpf_urandomb(g: mpf_t, state: randstate_t, nbits: mp_bitcnt_t) -> () {
  unsafe { __gmpf_urandomb(g, state, nbits) }
}

/// mpf_random2
#[inline]
pub fn mpf_random2(g: mpf_t, max_size: mp_size_t, e: mp_exp_t) -> () {
  unsafe { __gmpf_random2(g, max_size, e) }
}

/// mpf_get_default_prec
#[inline]
pub fn mpf_get_default_prec() -> mp_bitcnt_t {
  unsafe { __gmpf_get_default_prec() }
}

/// mpf_get_prec
#[inline]
pub fn mpf_get_prec(f: mpf_r) -> mp_bitcnt_t {
  unsafe { __gmpf_get_prec(f) }
}

/// mpf_set_default_prec
#[inline]
pub fn mpf_set_default_prec(n: mp_bitcnt_t) -> () {
  unsafe { __gmpf_set_default_prec(n) }
}

/// mpf_set_prec
#[inline]
pub fn mpf_set_prec(f: mpf_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpf_set_prec(f, n) }
}

/// mpf_set_prec_raw
#[inline]
pub fn mpf_set_prec_raw(f: mpf_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpf_set_prec_raw(f, n) }
}
