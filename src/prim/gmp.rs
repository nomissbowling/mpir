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

  /// __gmpz_cmp
  pub fn __gmpz_cmp(a: *mut mpz_s, b: *mut mpz_s) -> int_t;
  /// __gmpz_cmp_d
  pub fn __gmpz_cmp_d(a: *mut mpz_s, d: double_t) -> int_t;
  /// __gmpz_cmp_ui
  pub fn __gmpz_cmp_ui(a: *mut mpz_s, u: ui_t) -> int_t;
  /// __gmpz_cmp_si
  pub fn __gmpz_cmp_si(a: *mut mpz_s, s: si_t) -> int_t;
  /// __gmpz_cmpabs
  pub fn __gmpz_cmpabs(a: *mut mpz_s, b: *mut mpz_s) -> int_t;
  /// __gmpz_cmpabs_d
  pub fn __gmpz_cmpabs_d(a: *mut mpz_s, d: double_t) -> int_t;
  /// __gmpz_cmpabs_ui
  pub fn __gmpz_cmpabs_ui(a: *mut mpz_s, u: ui_t) -> int_t;
  /// __gmpz_sgn
  pub fn __gmpz_sgn(a: *mut mpz_s) -> int_t;

  /// __gmpz_fac_ui c = n!
  pub fn __gmpz_fac_ui(c: *mut mpz_s, n: ui_t) -> ();
  /// __gmpz_2fac_ui c = n!!
  pub fn __gmpz_2fac_ui(c: *mut mpz_s, n: ui_t) -> ();
  /// __gmpz_mfac_uiui c = n! ** m
  pub fn __gmpz_mfac_uiui(c: *mut mpz_s, n: ui_t, m: ui_t) -> ();

  /// __gmpz_abs
  pub fn __gmpz_abs(c: *mut mpz_s, a: *mut mpz_s) -> ();
  /// __gmpz_neg
  pub fn __gmpz_neg(c: *mut mpz_s, a: *mut mpz_s) -> ();

  /// __gmpz_sub c = a - b
  pub fn __gmpz_sub(c: *mut mpz_s, a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_sub_ui c = a - u
  pub fn __gmpz_sub_ui(c: *mut mpz_s, a: *mut mpz_s, u: ui_t) -> ();
  /// __gmpz_ui_sub c = u - a
  pub fn __gmpz_ui_sub(c: *mut mpz_s, u: ui_t, a: *mut mpz_s) -> ();
  /// __gmpz_submul c -= a * b
  pub fn __gmpz_submul(c: *mut mpz_s, a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_submul_ui c -= a * u
  pub fn __gmpz_submul_ui(c: *mut mpz_s, a: *mut mpz_s, u: ui_t) -> ();

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

  /// __gmpz_powwm_sec c = (a**n) mod m ***required n &gt; 0 and m is odd***
  pub fn __gmpz_powm_sec(c: *mut mpz_s,
    a: *mut mpz_s, n: *mut mpz_s, m: *mut mpz_s) -> ();
  /// __gmpz_powm c = (a**n) mod m ***n &lt; 0 when exists inv a**-1 mod m***
  pub fn __gmpz_powm(c: *mut mpz_s,
    a: *mut mpz_s, n: *mut mpz_s, m: *mut mpz_s) -> ();
  /// __gmpz_powm_ui c = (a**n) mod m
  pub fn __gmpz_powm_ui(c: *mut mpz_s,
    a: *mut mpz_s, n: ui_t, m: *mut mpz_s) -> ();
  /// __gmpz_pow_ui c == a**n
  pub fn __gmpz_pow_ui(c: *mut mpz_s, a: *mut mpz_s, n: ui_t) -> ();
  /// __gmpz_ui_pow_ui c = a**n
  pub fn __gmpz_ui_pow_ui(c: *mut mpz_s, a: ui_t, n: ui_t) -> ();

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

  /// __gmpf_cmp
  pub fn __gmpf_cmp(f: *mut mpf_s, g: *mut mpf_s) -> int_t;
  /// __gmpf_cmp_d
  pub fn __gmpf_cmp_d(f: *mut mpf_s, d: double_t) -> int_t;
  /// __gmpf_cmp_ui
  pub fn __gmpf_cmp_ui(f: *mut mpf_s, u: ui_t) -> int_t;
  /// __gmpf_cmp_si
  pub fn __gmpf_cmp_si(f: *mut mpf_s, s: si_t) -> int_t;
  /// __gmpf_cmp_z
  pub fn __gmpf_cmp_z(f: *mut mpf_s, a: *mut mpz_s) -> int_t;
  /// __gmpf_eq ***mathematically ill-defined and should not be used***
  pub fn __gmpf_eq(f: *mut mpf_s, g: *mut mpf_s, n: mp_bitcnt_t) -> int_t;
  /// __gmpf_sgn
  pub fn __gmpf_sgn(f: *mut mpf_s) -> int_t;
  /// __gmpf_reldiff
  pub fn __gmpf_reldiff(g: *mut mpf_s, f: *mut mpf_s, e: *mut mpf_s) -> ();

  /// __gmpf_sqrt
  pub fn __gmpf_sqrt(g: *mut mpf_s, f: *mut mpf_s) -> ();
  /// __gmpf_sqrt_ui
  pub fn __gmpf_sqrt_ui(g: *mut mpf_s, u: ui_t) -> ();

  /// __gmpf_abs
  pub fn __gmpf_abs(g: *mut mpf_s, f: *mut mpf_s) -> ();
  /// __gmpf_neg
  pub fn __gmpf_neg(g: *mut mpf_s, f: *mut mpf_s) -> ();

  /// __gmpf_sub g = f - e
  pub fn __gmpf_sub(g: *mut mpf_s, f: *mut mpf_s, e: *mut mpf_s) -> ();
  /// __gmpf_sub_ui g = f - u
  pub fn __gmpf_sub_ui(g: *mut mpf_s, f: *mut mpf_s, u: ui_t) -> ();
  /// __gmpf_ui_sub g = u - f
  pub fn __gmpf_ui_sub(g: *mut mpf_s, u: ui_t, f: *mut mpf_s) -> ();

  /// __gmpf_add g = f + e
  pub fn __gmpf_add(g: *mut mpf_s, f: *mut mpf_s, e: *mut mpf_s) -> ();
  /// __gmpf_add_ui g = f + u
  pub fn __gmpf_add_ui(g: *mut mpf_s, f: *mut mpf_s, u: ui_t) -> ();
  /// __gmpf_mul g = f * e
  pub fn __gmpf_mul(g: *mut mpf_s, f: *mut mpf_s, e: *mut mpf_s) -> ();
  /// __gmpf_mul_ui g = f * u
  pub fn __gmpf_mul_ui(g: *mut mpf_s, f: *mut mpf_s, u: ui_t) -> ();
  /// __gmpf_mul_2exp g = f * 2**n
  pub fn __gmpf_mul_2exp(g: *mut mpf_s, f: *mut mpf_s, n: mp_bitcnt_t) -> ();

  /// __gmpf_div g = f / e
  pub fn __gmpf_div(g: *mut mpf_s, f: *mut mpf_s, e: *mut mpf_s) -> ();
  /// __gmpf_div_ui g = f / u
  pub fn __gmpf_div_ui(g: *mut mpf_s, f: *mut mpf_s, u: ui_t) -> ();
  /// __gmpf_ui_div g = u / f
  pub fn __gmpf_ui_div(g: *mut mpf_s, u: ui_t, f: *mut mpf_s) -> ();
  /// __gmpf_div_2exp g = f / 2**n
  pub fn __gmpf_div_2exp(g: *mut mpf_s, f: *mut mpf_s, n: mp_bitcnt_t) -> ();

  /// __gmpf_pow_ui g = f**n
  pub fn __gmpf_pow_ui(g: *mut mpf_s, f: *mut mpf_s, n: ui_t) -> ();

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
