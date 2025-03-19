//! typ
//!

use std::ffi::{c_ulong, c_long};

// assume _WIN64 (_LONG_LONG_LIMB = 1)
/// c_int or c_long ***must be checked later***
#[allow(non_camel_case_types)]
pub type int_t = i32; // c_int unix also win: i32 ***must be checked later***
/// unsigned long long (_LONG_LONG_LIMB)
#[allow(non_camel_case_types)]
pub type mp_limb_t = u64; // c_ulonglong
/// long long (_LONG_LONG_LIMB)
#[allow(non_camel_case_types)]
pub type mp_limb_signed_t = i64; // c_longlong
/// size_t
#[allow(non_camel_case_types)]
pub type mp_size_t = usize;
/// long
#[allow(non_camel_case_types)]
pub type mp_exp_t = c_long; // unix: i64, win: i32
/// *mut c_void
#[allow(non_camel_case_types)]
pub type mp_t = *mut u8;
/// unsigned long
#[allow(non_camel_case_types)]
pub type ui_t = c_ulong; // unix: u64, win: u32 (not same as mp_limb_t)
/// signed long
#[allow(non_camel_case_types)]
pub type si_t = c_long; // unix: i64, win: i32 (not same as mp_limb_signed_t)
/// c_double
#[allow(non_camel_case_types)]
pub type double_t = f64;
/// bitcnt
#[allow(non_camel_case_types)]
pub type mp_bitcnt_t = ui_t;

/// allocate
pub type FnPtrAllocate = unsafe extern "C" fn(sz: mp_size_t) -> mp_t;
/// reallocate
pub type FnPtrReallocate = unsafe extern "C" fn(p: mp_t, z: mp_size_t, sz: mp_size_t) -> mp_t;
/// free
pub type FnPtrFree = unsafe extern "C" fn(p: mp_t, z: mp_size_t) -> ();
