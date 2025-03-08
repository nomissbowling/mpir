//! mpir
//!

use std::fmt;
use std::error::Error;

// assume _WIN64 (_LONG_LONG_LIMB = 1)
/// c_long or c_int (or test i64)
#[allow(non_camel_case_types)]
pub type int_t = i32;
/// unsigned long long (_LONG_LONG_LIMB)
#[allow(non_camel_case_types)]
pub type mp_limb_t = u64;
/// long long (_LONG_LONG_LIMB)
#[allow(non_camel_case_types)]
pub type mp_limb_signed_t = i64;
/// size_t
#[allow(non_camel_case_types)]
pub type mp_size_t = usize;
/// long
#[allow(non_camel_case_types)]
pub type mp_exp_t = i32;
/// *mut c_void
#[allow(non_camel_case_types)]
pub type mp_t = *mut u8;
/// unsigned
#[allow(non_camel_case_types)]
pub type ui_t = mp_limb_t;
/// signed
#[allow(non_camel_case_types)]
pub type si_t = mp_limb_signed_t;
/// c_double
#[allow(non_camel_case_types)]
pub type double_t = f64;

/// allocate
pub type FnPtrAllocate = unsafe extern "C" fn(sz: mp_size_t) -> mp_t;
/// reallocate
pub type FnPtrReallocate = unsafe extern "C" fn(p: mp_t, z: mp_size_t, sz: mp_size_t) -> mp_t;
/// free
pub type FnPtrFree = unsafe extern "C" fn(p: mp_t, z: mp_size_t) -> ();

/// to_u8z (&amp;str)
#[macro_export]
macro_rules! to_u8z {
  ($s:expr) => { (String::from($s)+"\0").as_bytes() }
}
// pub use to_u8z;

/// term_z ([u8])
#[macro_export]
macro_rules! term_z {
  ($u:expr) => { (String::from_utf8($u.to_vec()).unwrap()+"\0").as_bytes() }
}
// pub use term_z;

/// get length from u8z buffer pointer (terminated 0)
pub fn u8zlen(p: *mut u8) -> usize {
  let mut l = 0usize;
unsafe {
  loop { // infinit
    if std::slice::from_raw_parts_mut(p, l + 1)[l] == 0u8 { break; }
    l += 1;
  }
}
  l
}

/// get Option vec u8 from u8z buffer pointer (ff: true to free)
pub fn u8zvec(p: *mut u8, ff: bool) -> Option<Vec<u8>> {
unsafe {
  if p == 0 as *mut u8 { None }
  else {
    let l = u8zlen(p);
    let r = std::slice::from_raw_parts_mut(p, l).to_vec();
    if ff {
      let mut mp_free: FnPtrFree = __gmp_free_func; // dummy
      __gmp_get_memory_functions(
        0 as *mut FnPtrAllocate, 0 as *mut FnPtrReallocate, &mut mp_free);
      mp_free(p, l + 1);
    }
    Some(r)
  }
}
}

/// trait SNew
pub trait SNew {
  fn new() -> Self;
  fn as_ptr(&mut self) -> mp_t { self as *mut Self as mp_t }
}

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

// #[link(name="libgmp-10", kind="static")]
// #[link(name="libgmp-10", kind="dylib")]
// #[link(name="libgmp-10")]
extern "C" {
  pub fn __gmp_printf(f: *const u8, a: mp_t, b: mp_t, c: mp_t, d: mp_t) -> ();

  pub fn __gmpz_init(a: *mut mpz_s) -> ();
  pub fn __gmpz_init_set(a: *mut mpz_s, b: *mut mpz_s) -> ();
  pub fn __gmpz_init_set_ui(a: *mut mpz_s, u: ui_t) -> ();
  pub fn __gmpz_init_set_si(a: *mut mpz_s, s: si_t) -> ();
  pub fn __gmpz_init_set_d(a: *mut mpz_s, d: double_t) -> ();
  pub fn __gmpz_init_set_str(a: *mut mpz_s, s: *const u8, b: int_t) -> ();
  pub fn __gmpz_set(a: *mut mpz_s, b: *mut mpz_s) -> ();
  pub fn __gmpz_set_ui(a: *mut mpz_s, u: ui_t) -> ();
  pub fn __gmpz_set_si(a: *mut mpz_s, s: si_t) -> ();
  pub fn __gmpz_set_d(a: *mut mpz_s, d: double_t) -> ();
  pub fn __gmpz_set_str(a: *mut mpz_s, s: *const u8, b: int_t) -> ();
  pub fn __gmpz_get_str(s: *mut u8, b: int_t, a: *const mpz_s) -> *mut u8;

  pub fn __gmpf_init(f: *mut mpf_s) -> ();
  pub fn __gmpf_init_set(f: *mut mpf_s, g: *mut mpf_s) -> ();
  pub fn __gmpf_init_set_ui(f: *mut mpf_s, u: ui_t) -> ();
  pub fn __gmpf_init_set_si(f: *mut mpf_s, s: si_t) -> ();
  pub fn __gmpf_init_set_d(f: *mut mpf_s, d: double_t) -> ();
  pub fn __gmpf_init_set_str(f: *mut mpf_s, s: *const u8, b: int_t) -> ();
  pub fn __gmpf_set(f: *mut mpf_s, g: *mut mpf_s) -> ();
  pub fn __gmpf_set_ui(f: *mut mpf_s, u: ui_t) -> ();
  pub fn __gmpf_set_si(f: *mut mpf_s, s: si_t) -> ();
  pub fn __gmpf_set_d(f: *mut mpf_s, d: double_t) -> ();
  pub fn __gmpf_set_str(f: *mut mpf_s, s: *const u8, b: int_t) -> ();
  pub fn __gmpf_set_z(f: *mut mpf_s, a: *mut mpz_s) -> ();
  pub fn __gmpf_get_str(s: *mut u8,
    e: *mut mp_exp_t, b: int_t, d: mp_size_t, f: *const mpf_s) -> *mut u8;

  pub fn __gmpf_sqrt(g: *mut mpf_s, f: *mut mpf_s) -> ();

  pub fn __gmpq_init(q: *mut mpq_s) -> ();
  pub fn __gmpq_set_ui(q: *mut mpq_s, u: ui_t, f: ui_t) -> ();
  pub fn __gmpq_set_si(q: *mut mpq_s, s: si_t, f: ui_t) -> ();
  pub fn __gmpq_set_d(q: *mut mpq_s, d: double_t) -> ();
  pub fn __gmpq_set_str(q: *mut mpq_s, s: *const u8, b: int_t) -> ();
  pub fn __gmpq_get_str(s: *mut u8, b: int_t, q: *const mpq_s) -> *mut u8;

  pub fn __gmp_get_memory_functions(
    alloc: *mut FnPtrAllocate,
    realloc: *mut FnPtrReallocate,
    free: *mut FnPtrFree) -> ();
  pub fn __gmp_allocate_func(sz: mp_size_t) -> mp_t;
  pub fn __gmp_reallocate_func(p: mp_t, z: mp_size_t, sz: mp_size_t) -> mp_t;
  pub fn __gmp_free_func(p: mp_t, z: mp_size_t) -> ();
}

/// gmp_printf
pub fn gmp_printf<'a, T: SNew>(f: &str, a: &'a mut T) -> () {
  gmp_printf_u8z(to_u8z!(f), a)
}

/// gmp_printf_u8z
pub fn gmp_printf_u8z<'a, T: SNew>(f: &[u8], a: &'a mut T) -> () {
  unsafe {
    __gmp_printf(f as *const [u8] as *const u8,
      a.as_ptr(), 0 as mp_t, 0 as mp_t, 0 as mp_t)
  }
}

/// gmp_printf_1f
pub fn gmp_printf_1f<'a, T: SNew>(f: &str,
  p: int_t, a: &'a mut T) -> () {
  gmp_printf_u8z_1f(to_u8z!(f), p, a)
}

/// gmp_printf_u8z_1f
pub fn gmp_printf_u8z_1f<'a, T: SNew>(f: &[u8],
  p: int_t, a: &'a mut T) -> () {
  unsafe {
    __gmp_printf(f as *const [u8] as *const u8,
      p as mp_t, a.as_ptr(), 0 as mp_t, 0 as mp_t)
  }
}

/// gmp_printf_2f
pub fn gmp_printf_2f<'a, T: SNew>(f: &str,
  p: int_t, a: &'a mut T, q: int_t, b: &'a mut T) -> () {
  gmp_printf_u8z_2f(to_u8z!(f), p, a, q, b)
}

/// gmp_printf_u8z_2f
pub fn gmp_printf_u8z_2f<'a, T: SNew>(f: &[u8],
  p: int_t, a: &'a mut T, q: int_t, b: &'a mut T) -> () {
  unsafe {
    __gmp_printf(f as *const [u8] as *const u8,
      p as mp_t, a.as_ptr(), q as mp_t, b.as_ptr())
  }
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

/// mpf_set_str
pub fn mpf_set_str(f: mpf_t, s: &str, b: int_t) -> () {
  mpf_set_str_u8z(f, to_u8z!(s), b)
}

/// mpf_set_str_u8z
pub fn mpf_set_str_u8z(f: mpf_t, s: &[u8], b: int_t) -> () {
  unsafe { __gmpf_set_str(f, s as *const [u8] as *const u8, b) }
}

/// mpf_set_z
pub fn mpf_set_z(f: mpf_t, a: mpz_t) -> () {
  unsafe { __gmpf_set_z(f, a) }
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

/// mpq_init
pub fn mpq_init(q: mpq_t) -> () {
  unsafe { __gmpq_init(q) }
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

/// mp_get_memory_functions
pub fn mp_get_memory_functions(
  alloc: &mut FnPtrAllocate,
  realloc: &mut FnPtrReallocate,
  free: &mut FnPtrFree) -> () {
  unsafe { __gmp_get_memory_functions(alloc, realloc, free) }
}
