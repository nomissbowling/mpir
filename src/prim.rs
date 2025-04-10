//! prim
//!

pub mod typ;
pub mod mpz;
pub mod mpf;
pub mod mpq;
pub mod randstate;
pub mod gmp;

use crate::prim::{typ::*, gmp::*}; // mpz::*, mpf::*, mpq::*, randstate::*

/// trait SNew private
trait SNew {
  /// new
  fn new() -> Self;
}

/// trait AsPtr
pub trait AsPtr {
  /// as_ptr
  fn as_ptr(&self) -> mp_r { self as *const Self as mp_r }
}

/// trait AsPtrMut
pub trait AsPtrMut {
  /// as_ptr_mut
  fn as_ptr_mut(&mut self) -> mp_t { self as *mut Self as mp_t }
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
#[inline]
pub fn gmp_printf<'a, T: AsPtr>(f: &str, a: &'a T) -> () {
  gmp_printf_u8z(to_u8z!(f), a)
}

/// gmp_printf_u8z
#[inline]
pub fn gmp_printf_u8z<'a, T: AsPtr>(f: &[u8], a: &'a T) -> () {
  unsafe {
    __gmp_printf(f as *const [u8] as mp_r,
      a.as_ptr(), 0 as mp_r, 0 as mp_r, 0 as mp_r)
  }
}

/// gmp_printf_1f
#[inline]
pub fn gmp_printf_1f<'a, T: AsPtr>(f: &str, p: int_t, a: &'a T) -> () {
  gmp_printf_u8z_1f(to_u8z!(f), p, a)
}

/// gmp_printf_u8z_1f
#[inline]
pub fn gmp_printf_u8z_1f<'a, T: AsPtr>(f: &[u8], p: int_t, a: &'a T) -> () {
  unsafe {
    __gmp_printf(f as *const [u8] as mp_r,
      p as mp_r, a.as_ptr(), 0 as mp_r, 0 as mp_r)
  }
}

/// gmp_printf_2f
#[inline]
pub fn gmp_printf_2f<'a, T: AsPtr>(f: &str,
  p: int_t, a: &'a T, q: int_t, b: &'a T) -> () {
  gmp_printf_u8z_2f(to_u8z!(f), p, a, q, b)
}

/// gmp_printf_u8z_2f
#[inline]
pub fn gmp_printf_u8z_2f<'a, T: AsPtr>(f: &[u8],
  p: int_t, a: &'a T, q: int_t, b: &'a T) -> () {
  unsafe {
    __gmp_printf(f as *const [u8] as mp_r,
      p as mp_r, a.as_ptr(), q as mp_r, b.as_ptr())
  }
}

/// mp_get_memory_functions
#[inline]
pub fn mp_get_memory_functions(
  alloc: &mut FnPtrAllocate,
  realloc: &mut FnPtrReallocate,
  free: &mut FnPtrFree) -> () {
  unsafe { __gmp_get_memory_functions(alloc, realloc, free) }
}
