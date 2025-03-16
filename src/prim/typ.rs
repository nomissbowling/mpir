//! typ
//!

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
pub type ui_t = u32; // not mp_limb_t
/// signed
#[allow(non_camel_case_types)]
pub type si_t = i32; // not mp_limb_signed_t
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
