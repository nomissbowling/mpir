//! gmp
//!

use crate::prim::{typ::*, mpz::*, mpf::*, mpq::*};

// #[link(name="libgmp-10", kind="static")]
// #[link(name="libgmp-10", kind="dylib")]
// #[link(name="libgmp-10")]
extern "C" {
  /// __gmp_printf *** CAUTION *** (assume just 4 fake parameters after fmt)
  pub fn __gmp_printf(f: *const u8, a: mp_t, b: mp_t, c: mp_t, d: mp_t) -> ();

  /// __gmpz_clears *** CAUTION *** (assume just 1 fake parameters after first)
  pub fn __gmpz_clears(a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_clear
  pub fn __gmpz_clear(a: *mut mpz_s) -> ();
  /// __gmpz_init
  pub fn __gmpz_init(a: *mut mpz_s) -> ();
  /// __gmpz_init_set
  pub fn __gmpz_init_set(a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_init_set_ui
  pub fn __gmpz_init_set_ui(a: *mut mpz_s, u: ui_t) -> ();
  /// __gmpz_init_set_si
  pub fn __gmpz_init_set_si(a: *mut mpz_s, s: si_t) -> ();
  /// __gmpz_init_set_d
  pub fn __gmpz_init_set_d(a: *mut mpz_s, d: double_t) -> ();
  /// __gmpz_init_set_str
  pub fn __gmpz_init_set_str(a: *mut mpz_s, s: *const u8, b: int_t) -> ();
  /// __gmpz_set
  pub fn __gmpz_set(a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_set_ui
  pub fn __gmpz_set_ui(a: *mut mpz_s, u: ui_t) -> ();
  /// __gmpz_set_si
  pub fn __gmpz_set_si(a: *mut mpz_s, s: si_t) -> ();
  /// __gmpz_set_d
  pub fn __gmpz_set_d(a: *mut mpz_s, d: double_t) -> ();
  /// __gmpz_set_str
  pub fn __gmpz_set_str(a: *mut mpz_s, s: *const u8, b: int_t) -> ();
  /// __gmpz_get_str
  pub fn __gmpz_get_str(s: *mut u8, b: int_t, a: *const mpz_s) -> *mut u8;

  /// __gmpz_add c = a + b
  pub fn __gmpz_add(c: *mut mpz_s, a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_add_ui c = a + u
  pub fn __gmpz_add_ui(c: *mut mpz_s, a: *mut mpz_s, u: ui_t) -> ();
  /// __gmpz_addmul c += a * b
  pub fn __gmpz_addmul(c: *mut mpz_s, a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_addmul_ui c += a * u
  pub fn __gmpz_addmul_ui(c: *mut mpz_s, a: *mut mpz_s, u: ui_t) -> ();
  /// __gmpz_mul c = a * b
  pub fn __gmpz_mul(c: *mut mpz_s, a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_mul_ui c = a * u
  pub fn __gmpz_mul_ui(c: *mut mpz_s, a: *mut mpz_s, u: ui_t) -> ();
  /// __gmpz_mul_si c = a * s
  pub fn __gmpz_mul_si(c: *mut mpz_s, a: *mut mpz_s, s: si_t) -> ();
  /// __gmpz_mul_2exp c = a * 2**n
  pub fn __gmpz_mul_2exp(c: *mut mpz_s, a: *mut mpz_s, n: mp_bitcnt_t) -> ();

  /// __gmpf_clears *** CAUTION *** (assume just 1 fake parameters after first)
  pub fn __gmpf_clears(f: *mut mpf_s, g: *mut mpf_s) -> ();
  /// __gmpf_clear
  pub fn __gmpf_clear(f: *mut mpf_s) -> ();
  /// __gmpf_init
  pub fn __gmpf_init(f: *mut mpf_s) -> ();
  /// __gmpf_init_set
  pub fn __gmpf_init_set(f: *mut mpf_s, g: *mut mpf_s) -> ();
  /// __gmpf_init_set_ui
  pub fn __gmpf_init_set_ui(f: *mut mpf_s, u: ui_t) -> ();
  /// __gmpf_init_set_si
  pub fn __gmpf_init_set_si(f: *mut mpf_s, s: si_t) -> ();
  /// __gmpf_init_set_d
  pub fn __gmpf_init_set_d(f: *mut mpf_s, d: double_t) -> ();
  /// __gmpf_init_set_str
  pub fn __gmpf_init_set_str(f: *mut mpf_s, s: *const u8, b: int_t) -> ();
  /// __gmpf_set
  pub fn __gmpf_set(f: *mut mpf_s, g: *mut mpf_s) -> ();
  /// __gmpf_set_ui
  pub fn __gmpf_set_ui(f: *mut mpf_s, u: ui_t) -> ();
  /// __gmpf_set_si
  pub fn __gmpf_set_si(f: *mut mpf_s, s: si_t) -> ();
  /// __gmpf_set_d
  pub fn __gmpf_set_d(f: *mut mpf_s, d: double_t) -> ();
  /// __gmpf_set_z
  pub fn __gmpf_set_z(f: *mut mpf_s, a: *mut mpz_s) -> ();
  /// __gmpf_set_str
  pub fn __gmpf_set_str(f: *mut mpf_s, s: *const u8, b: int_t) -> ();
  /// __gmpf_get_str
  pub fn __gmpf_get_str(s: *mut u8,
    e: *mut mp_exp_t, b: int_t, d: mp_size_t, f: *const mpf_s) -> *mut u8;

  /// __gmpf_sqrt
  pub fn __gmpf_sqrt(g: *mut mpf_s, f: *mut mpf_s) -> ();
  /// __gmpf_div g = f / e
  pub fn __gmpf_div(g: *mut mpf_s, f: *mut mpf_s, e: *mut mpf_s) -> ();
  /// __gmpf_ui_div g = u / f
  pub fn __gmpf_ui_div(g: *mut mpf_s, u: ui_t, f: *mut mpf_s) -> ();
  /// __gmpf_div_ui g = f / u
  pub fn __gmpf_div_ui(g: *mut mpf_s, f: *mut mpf_s, u: ui_t) -> ();
  /// __gmpf_div_2exp g = f / 2**n
  pub fn __gmpf_div_2exp(g: *mut mpf_s, f: *mut mpf_s, n: mp_bitcnt_t) -> ();

  /// __gmpq_clears *** CAUTION *** (assume just 1 fake parameters after first)
  pub fn __gmpq_clears(q: *mut mpq_s, r: *mut mpq_s) -> ();
  /// __gmpq_clear
  pub fn __gmpq_clear(q: *mut mpq_s) -> ();
  /// __gmpq_init
  pub fn __gmpq_init(q: *mut mpq_s) -> ();
  /// __gmpq_set
  pub fn __gmpq_set(q: *mut mpq_s, r: *mut mpq_s) -> ();
  /// __gmpq_set_ui
  pub fn __gmpq_set_ui(q: *mut mpq_s, u: ui_t, f: ui_t) -> ();
  /// __gmpq_set_si
  pub fn __gmpq_set_si(q: *mut mpq_s, s: si_t, f: ui_t) -> ();
  /// __gmpq_set_d
  pub fn __gmpq_set_d(q: *mut mpq_s, d: double_t) -> ();
  /// __gmpq_set_z
  pub fn __gmpq_set_z(q: *mut mpq_s, a: *mut mpz_s) -> ();
  /// __gmpq_set_f
  pub fn __gmpq_set_f(q: *mut mpq_s, f: *mut mpf_s) -> ();
  /// __gmpq_set_num
  pub fn __gmpq_set_num(q: *mut mpq_s, num: *mut mpz_s) -> ();
  /// __gmpq_set_den
  pub fn __gmpq_set_den(q: *mut mpq_s, den: *mut mpz_s) -> ();
  /// __gmpq_set_str
  pub fn __gmpq_set_str(q: *mut mpq_s, s: *const u8, b: int_t) -> ();
  /// __gmpq_get_str
  pub fn __gmpq_get_str(s: *mut u8, b: int_t, q: *const mpq_s) -> *mut u8;

  /// ___gmp_get_memory_functions
  pub fn __gmp_get_memory_functions(
    alloc: *mut FnPtrAllocate,
    realloc: *mut FnPtrReallocate,
    free: *mut FnPtrFree) -> ();
  /// ___gmp_allocate_func
  pub fn __gmp_allocate_func(sz: mp_size_t) -> mp_t;
  /// ___gmp_reallocate_func
  pub fn __gmp_reallocate_func(p: mp_t, z: mp_size_t, sz: mp_size_t) -> mp_t;
  /// __gmp_free_func
  pub fn __gmp_free_func(p: mp_t, z: mp_size_t) -> ();
}
