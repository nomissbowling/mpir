//! prim
//!

pub mod typ;
pub mod mpz;
pub mod mpf;
pub mod mpq;
pub mod gmp;

use crate::prim::{typ::*, gmp::*}; // mpz::*, mpf::*, mpq::*

/// trait SNew
pub trait SNew {
  /// new
  fn new() -> Self;
  /// as_ptr
  fn as_ptr(&mut self) -> mp_t { self as *mut Self as mp_t }
}

/// to_u8z (&amp;str)
#[macro_export]
macro_rules! to_u8z {
  ($s:expr) => { (String::from($s)+"\0").as_bytes() }
}
pub use to_u8z;

/// term_z ([u8])
#[macro_export]
macro_rules! term_z {
  ($u:expr) => { (String::from_utf8($u.to_vec()).unwrap()+"\0").as_bytes() }
}
pub use term_z;

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

/// mp_get_memory_functions
pub fn mp_get_memory_functions(
  alloc: &mut FnPtrAllocate,
  realloc: &mut FnPtrReallocate,
  free: &mut FnPtrFree) -> () {
  unsafe { __gmp_get_memory_functions(alloc, realloc, free) }
}
