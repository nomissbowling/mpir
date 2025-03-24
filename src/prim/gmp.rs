//! gmp
//!

use crate::prim::{typ::*, mpz::*, mpf::*, mpq::*, randstate::*};

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
  /// __gmpz_inits *** CAUTION *** (assume just 1 fake parameters after first)
  pub fn __gmpz_inits(a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_init
  pub fn __gmpz_init(a: *mut mpz_s) -> ();
  /// __gmpz_init2
  pub fn __gmpz_init2(a: *mut mpz_s, n: mp_bitcnt_t) -> ();
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

  /// __gmpz_get_d
  pub fn __gmpz_get_d(a: *mut mpz_s) -> double_t;
  /// __gmpz_get_ui
  pub fn __gmpz_get_ui(a: *mut mpz_s) -> ui_t;
  /// __gmpz_get_si
  pub fn __gmpz_get_si(a: *mut mpz_s) -> si_t;
  /// __gmpz_get_d_2exp
  pub fn __gmpz_get_d_2exp(e: *mut si_t, a: *mut mpz_s) -> double_t;

  /// __gmpz_swap
  pub fn __gmpz_swap(a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_realloc2
  pub fn __gmpz_realloc2(a: *mut mpz_s, n: mp_bitcnt_t) -> ();
  /// __gmpz_realloc
  pub fn __gmpz_realloc(a: *mut mpz_s, sz: mp_size_t) -> mp_t;
  /// __gmpz_array_init ***(obsoleted) do NOT use it***
  pub fn __gmpz_array_init(a: *mut mpz_s, sz: mp_size_t, fnb: mp_size_t) -> ();
  /// __gmpz_size
  pub fn __gmpz_size(a: *const mpz_s) -> mp_size_t;
  /// __gmpz_limbs_read (unsafe pointer to array mpz_size elements)
  pub fn __gmpz_limbs_read(a: *const mpz_s) -> *const mp_limb_t;
  /// __gmpz_getlimbn (single element)
  pub fn __gmpz_getlimbn(a: *mut mpz_s, n: mp_size_t) -> mp_limb_t;
  /// __gmpz_limbs_write (unsafe pointer to array sz elements) be reallocated
  pub fn __gmpz_limbs_write(a: *mut mpz_s, sz: mp_size_t) -> *mut mp_limb_t;
  /// __gmpz_limbs_modify (unsafe pointer to array sz elements) be reallocated
  pub fn __gmpz_limbs_modify(a: *mut mpz_s, sz: mp_size_t) -> *mut mp_limb_t;
  /// __gmpz_limbs_finish (used after write or modify to update internal size)
  pub fn __gmpz_limbs_finish(a: *mut mpz_s, sz: mp_size_t) -> ();
  /// __gmpz_roinit_n (unsafe)
  pub fn __gmpz_roinit_n(a: *mut mpz_s,
    p: *mut mp_limb_t, sz: mp_size_t) -> *mut mpz_s;

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
/*
  /// __gmpz_sgn
  pub fn __gmpz_sgn(a: *const mpz_s) -> int_t;
*/

  /// __gmpz_root r = nth root of a
  pub fn __gmpz_root(r: *mut mpz_s, a: *mut mpz_s, n: ui_t) -> int_t;
  /// __gmpz_rootrem r = nth root of u, rem = u - r**n (to the remainder)
  pub fn __gmpz_rootrem(r: *mut mpz_s, rem: *mut mpz_s, u: *mut mpz_s, n: ui_t) -> ();
  /// __gmpz_sqrt r = square root of a
  pub fn __gmpz_sqrt(r: *mut mpz_s, a: *mut mpz_s) -> ();
  /// __gmpz_sqrtrem r = square root of u, rem = u - r**2 (to the remainder)
  pub fn __gmpz_sqrtrem(r: *mut mpz_s, rem: *mut mpz_s, u: *mut mpz_s) -> ();
  /// __gmpz_perfect_power_p
  pub fn __gmpz_perfect_power_p(a: *mut mpz_s) -> int_t;
  /// __gmpz_perfect_square_p
  pub fn __gmpz_perfect_square_p(a: *mut mpz_s) -> int_t;

  /// __gmpz_primorial_ui c = 2*3*5*7*11*...*p(prev)*p(&lt;=n)
  pub fn __gmpz_primorial_ui(c: *mut mpz_s, n: ui_t) -> ();

  /// __gmpz_fac_ui c = n!
  pub fn __gmpz_fac_ui(c: *mut mpz_s, n: ui_t) -> ();
  /// __gmpz_2fac_ui c = n!!
  pub fn __gmpz_2fac_ui(c: *mut mpz_s, n: ui_t) -> ();
  /// __gmpz_mfac_uiui c = n! ** m
  pub fn __gmpz_mfac_uiui(c: *mut mpz_s, n: ui_t, m: ui_t) -> ();

  /// __gmpz_remove
  pub fn __gmpz_remove(c: *mut mpz_s,
    a: *mut mpz_s, f: *mut mpz_s) -> mp_bitcnt_t;

  /// __gmpz_fib_ui
  pub fn __gmpz_fib_ui(f_n: *mut mpz_s, n: ui_t) -> ();
  /// __gmpz_fib2_ui
  pub fn __gmpz_fib2_ui(f_n: *mut mpz_s, f_nsub1: *mut mpz_s, n: ui_t) -> ();

  /// __gmpz_lucnum_ui
  pub fn __gmpz_lucnum_ui(l_n: *mut mpz_s, n: ui_t) -> ();
  /// __gmpz_lucnum2_ui
  pub fn __gmpz_lucnum2_ui(l_n: *mut mpz_s, l_n_1: *mut mpz_s, n: ui_t) -> ();

  /// __gmpz_gcd
  pub fn __gmpz_gcd(g: *mut mpz_s, a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_gcd_ui return 0 when gcd does not fit to ui_t
  pub fn __gmpz_gcd_ui(g: *mut mpz_s, a: *mut mpz_s, u: ui_t) -> ui_t;
  /// __gmpz_gcdext s and t to coefficients satisfying a*s + b*t == gcd
  pub fn __gmpz_gcdext(g: *mut mpz_s, s: *mut mpz_s, t: *mut mpz_s,
    a: *mut mpz_s, b: *mut mpz_s) -> ();

  /// __gmpz_lcm
  pub fn __gmpz_lcm(c: *mut mpz_s, a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_lcm_ui
  pub fn __gmpz_lcm_ui(c: *mut mpz_s, a: *mut mpz_s, u: ui_t) -> ();

  /// __gmpz_probab_prime_p 2 or 1 or 0
  pub fn __gmpz_probab_prime_p(a: *mut mpz_s, r: int_t) -> int_t;
  /// __gmpz_nextprime
  pub fn __gmpz_nextprime(c: *mut mpz_s, a: *mut mpz_s) -> ();
/*
  /// __gmpz_prevprime
  pub fn __gmpz_prevprime(c: *mut mpz_s, a: *mut mpz_s) -> ();
*/

  /// __gmpz_invert c = inverse of a mod b ((c*a) mod b == 1)
  pub fn __gmpz_invert(c: *mut mpz_s, a: *mut mpz_s, b: *mut mpz_s) -> int_t;

  /// __gmpz_jacobi 0 1 -1 (defined only for n odd)
  pub fn __gmpz_jacobi(a: *mut mpz_s, n: *mut mpz_s) -> int_t;
  /// __gmpz_legendre 0 1 -1 (defined only for p an odd positive prime)
  pub fn __gmpz_legendre(a: *mut mpz_s, p: *mut mpz_s) -> int_t;
/*
  /// __gmpz_kronecker
  pub fn __gmpz_kronecker(a: *mut mpz_s, n: *mut mpz_s) -> int_t;
*/
  /// __gmpz_kronecker_ui
  pub fn __gmpz_kronecker_ui(a: *mut mpz_s, u: ui_t) -> int_t;
  /// __gmpz_kronecker_si
  pub fn __gmpz_kronecker_si(a: *mut mpz_s, s: si_t) -> int_t;
  /// __gmpz_ui_kronecker
  pub fn __gmpz_ui_kronecker(u: ui_t, a: *mut mpz_s) -> int_t;
  /// __gmpz_si_kronecker
  pub fn __gmpz_si_kronecker(s: si_t, a: *mut mpz_s) -> int_t;

  /// __gmpz_bin_ui nCk
  pub fn __gmpz_bin_ui(c: *mut mpz_s, n: *mut mpz_s, k: ui_t) -> ();
  /// __gmpz_bin_uiui nCk
  pub fn __gmpz_bin_uiui(c: *mut mpz_s, n: ui_t, k: ui_t) -> ();

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

  /// __gmpz_cdiv_q
  pub fn __gmpz_cdiv_q(q: *mut mpz_s, n: *mut mpz_s, d: *mut mpz_s) -> ();
  /// __gmpz_cdiv_r
  pub fn __gmpz_cdiv_r(r: *mut mpz_s, n: *mut mpz_s, d: *mut mpz_s) -> ();
  /// __gmpz_cdiv_qr
  pub fn __gmpz_cdiv_qr(q: *mut mpz_s, r: *mut mpz_s,
    n: *mut mpz_s, d: *mut mpz_s) -> ();
  /// __gmpz_cdiv_q_ui
  pub fn __gmpz_cdiv_q_ui(q: *mut mpz_s, n: *mut mpz_s, d: ui_t) -> ui_t;
  /// __gmpz_cdiv_r_ui
  pub fn __gmpz_cdiv_r_ui(r: *mut mpz_s, n: *mut mpz_s, d: ui_t) -> ui_t;
  /// __gmpz_cdiv_qr_ui
  pub fn __gmpz_cdiv_qr_ui(q: *mut mpz_s, r: *mut mpz_s,
    n: *mut mpz_s, d: ui_t) -> ui_t;
  /// __gmpz_cdiv_ui
  pub fn __gmpz_cdiv_ui(n: *mut mpz_s, d: ui_t) -> ui_t;
  /// __gmpz_cdiv_q_2exp
  pub fn __gmpz_cdiv_q_2exp(q: *mut mpz_s,
    n: *mut mpz_s, b: mp_bitcnt_t) -> ();
  /// __gmpz_cdiv_r_2exp
  pub fn __gmpz_cdiv_r_2exp(r: *mut mpz_s,
    n: *mut mpz_s, b: mp_bitcnt_t) -> ();

  /// __gmpz_fdiv_q
  pub fn __gmpz_fdiv_q(q: *mut mpz_s, n: *mut mpz_s, d: *mut mpz_s) -> ();
  /// __gmpz_fdiv_r
  pub fn __gmpz_fdiv_r(r: *mut mpz_s, n: *mut mpz_s, d: *mut mpz_s) -> ();
  /// __gmpz_fdiv_qr
  pub fn __gmpz_fdiv_qr(q: *mut mpz_s, r: *mut mpz_s,
    n: *mut mpz_s, d: *mut mpz_s) -> ();
  /// __gmpz_fdiv_q_ui
  pub fn __gmpz_fdiv_q_ui(q: *mut mpz_s, n: *mut mpz_s, d: ui_t) -> ui_t;
  /// __gmpz_fdiv_r_ui
  pub fn __gmpz_fdiv_r_ui(r: *mut mpz_s, n: *mut mpz_s, d: ui_t) -> ui_t;
  /// __gmpz_fdiv_qr_ui
  pub fn __gmpz_fdiv_qr_ui(q: *mut mpz_s, r: *mut mpz_s,
    n: *mut mpz_s, d: ui_t) -> ui_t;
  /// __gmpz_fdiv_ui
  pub fn __gmpz_fdiv_ui(n: *mut mpz_s, d: ui_t) -> ui_t;
  /// __gmpz_fdiv_q_2exp
  pub fn __gmpz_fdiv_q_2exp(q: *mut mpz_s,
    n: *mut mpz_s, b: mp_bitcnt_t) -> ();
  /// __gmpz_fdiv_r_2exp
  pub fn __gmpz_fdiv_r_2exp(r: *mut mpz_s,
    n: *mut mpz_s, b: mp_bitcnt_t) -> ();

  /// __gmpz_tdiv_q
  pub fn __gmpz_tdiv_q(q: *mut mpz_s, n: *mut mpz_s, d: *mut mpz_s) -> ();
  /// __gmpz_tdiv_r
  pub fn __gmpz_tdiv_r(r: *mut mpz_s, n: *mut mpz_s, d: *mut mpz_s) -> ();
  /// __gmpz_tdiv_qr
  pub fn __gmpz_tdiv_qr(q: *mut mpz_s, r: *mut mpz_s,
    n: *mut mpz_s, d: *mut mpz_s) -> ();
  /// __gmpz_tdiv_q_ui
  pub fn __gmpz_tdiv_q_ui(q: *mut mpz_s, n: *mut mpz_s, d: ui_t) -> ui_t;
  /// __gmpz_tdiv_r_ui
  pub fn __gmpz_tdiv_r_ui(r: *mut mpz_s, n: *mut mpz_s, d: ui_t) -> ui_t;
  /// __gmpz_tdiv_qr_ui
  pub fn __gmpz_tdiv_qr_ui(q: *mut mpz_s, r: *mut mpz_s,
    n: *mut mpz_s, d: ui_t) -> ui_t;
  /// __gmpz_tdiv_ui
  pub fn __gmpz_tdiv_ui(n: *mut mpz_s, d: ui_t) -> ui_t;
  /// __gmpz_tdiv_q_2exp
  pub fn __gmpz_tdiv_q_2exp(q: *mut mpz_s,
    n: *mut mpz_s, b: mp_bitcnt_t) -> ();
  /// __gmpz_tdiv_r_2exp
  pub fn __gmpz_tdiv_r_2exp(r: *mut mpz_s,
    n: *mut mpz_s, b: mp_bitcnt_t) -> ();

  /// __gmpz_mod
  pub fn __gmpz_mod(r: *mut mpz_s, n: *mut mpz_s, d: *mut mpz_s) -> ();
/*
  /// __gmpz_mod_ui (use __gmpz_fdiv_r_ui)
  pub fn __gmpz_mod_ui(r: *mut mpz_s, n: *mut mpz_s, d: ui_t) -> ui_t;
*/

  /// __gmpz_divexact
  pub fn __gmpz_divexact(q: *mut mpz_s, n: *mut mpz_s, d: *mut mpz_s) -> ();
  /// __gmpz_divexact_ui
  pub fn __gmpz_divexact_ui(q: *mut mpz_s, n: *mut mpz_s, d: ui_t) -> ();

  /// __gmpz_divisible_p
  pub fn __gmpz_divisible_p(n: *mut mpz_s, d: *mut mpz_s) -> int_t;
  /// __gmpz_divisible_ui_p
  pub fn __gmpz_divisible_ui_p(n: *mut mpz_s, d: ui_t) -> int_t;
  /// __gmpz_divisible_2exp_p
  pub fn __gmpz_divisible_2exp_p(n: *mut mpz_s, b: mp_bitcnt_t) -> int_t;

  /// __gmpz_congruent_p
  pub fn __gmpz_congruent_p(n: *mut mpz_s,
    c: *mut mpz_s, d: *mut mpz_s) -> int_t;
  /// __gmpz_congruent_ui_p
  pub fn __gmpz_congruent_ui_p(n: *mut mpz_s,
    c: ui_t, d: ui_t) -> int_t;
  /// __gmpz_congruent_2exp_p
  pub fn __gmpz_congruent_2exp_p(n: *mut mpz_s,
    c: *mut mpz_s, b: mp_bitcnt_t) -> int_t;

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

  /// __gmpz_sizeinbase
  pub fn __gmpz_sizeinbase(a: *mut mpz_s, base: int_t) -> mp_size_t;
/*
  /// __gmpz_even_p
  pub fn __gmpz_even_p(a: *mut mpz_s) -> int_t;
  /// __gmpz_odd_p
  pub fn __gmpz_odd_p(a: *mut mpz_s) -> int_t;
*/
  /// __gmpz_fits_ulong_p
  pub fn __gmpz_fits_ulong_p(a: *mut mpz_s) -> int_t;
  /// __gmpz_fits_slong_p
  pub fn __gmpz_fits_slong_p(a: *mut mpz_s) -> int_t;
  /// __gmpz_fits_uint_p
  pub fn __gmpz_fits_uint_p(a: *mut mpz_s) -> int_t;
  /// __gmpz_fits_sint_p
  pub fn __gmpz_fits_sint_p(a: *mut mpz_s) -> int_t;
  /// __gmpz_fits_ushort_p
  pub fn __gmpz_fits_ushort_p(a: *mut mpz_s) -> int_t;
  /// __gmpz_fits_sshort_p
  pub fn __gmpz_fits_sshort_p(a: *mut mpz_s) -> int_t;

  /// __gmpz_urandomb
  pub fn __gmpz_urandomb(c: *mut mpz_s,
    r: *mut randstate_s, nbits: mp_bitcnt_t) -> ();
  /// __gmpz_urandomm
  pub fn __gmpz_urandomm(c: *mut mpz_s,
    r: *mut randstate_s, n: *mut mpz_s) -> ();
  /// __gmpz_rrandomb
  pub fn __gmpz_rrandomb(c: *mut mpz_s,
    r: *mut randstate_s, nbits: mp_bitcnt_t) -> ();
  /// __gmpz_random ***(obsoleted) use mpz_urandomb or mpz_urandomm instead***
  pub fn __gmpz_random(c: *mut mpz_s, max_size: mp_size_t) -> ();
  /// __gmpz_random2
  pub fn __gmpz_random2(c: *mut mpz_s, max_size: mp_size_t) -> ();

  /// __gmpz_and
  pub fn __gmpz_and(c: *mut mpz_s, a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_ior
  pub fn __gmpz_ior(c: *mut mpz_s, a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_xor
  pub fn __gmpz_xor(c: *mut mpz_s, a: *mut mpz_s, b: *mut mpz_s) -> ();
  /// __gmpz_com
  pub fn __gmpz_com(c: *mut mpz_s, a: *mut mpz_s) -> ();
  /// __gmpz_popcount
  pub fn __gmpz_popcount(a: *mut mpz_s) -> mp_bitcnt_t;
  /// __gmpz_hamdist hamming distance between a and b (both sgn must be same)
  pub fn __gmpz_hamdist(a: *mut mpz_s, b: *mut mpz_s) -> mp_bitcnt_t;
  /// __gmpz_scan0 to msb
  pub fn __gmpz_scan0(a: *mut mpz_s, s: mp_bitcnt_t) -> mp_bitcnt_t;
  /// __gmpz_scan1 to msb
  pub fn __gmpz_scan1(a: *mut mpz_s, s: mp_bitcnt_t) -> mp_bitcnt_t;
  /// __gmpz_clrbit
  pub fn __gmpz_clrbit(c: *mut mpz_s, n: mp_bitcnt_t) -> ();
  /// __gmpz_setbit
  pub fn __gmpz_setbit(c: *mut mpz_s, n: mp_bitcnt_t) -> ();
  /// __gmpz_combit
  pub fn __gmpz_combit(c: *mut mpz_s, n: mp_bitcnt_t) -> ();
  /// __gmpz_tstbit
  pub fn __gmpz_tstbit(a: *mut mpz_s, n: mp_bitcnt_t) -> int_t;

  /// __gmpf_clears *** CAUTION *** (assume just 1 fake parameters after first)
  pub fn __gmpf_clears(f: *mut mpf_s, g: *mut mpf_s) -> ();
  /// __gmpf_clear
  pub fn __gmpf_clear(f: *mut mpf_s) -> ();
  /// __gmpf_inits *** CAUTION *** (assume just 1 fake parameters after first)
  pub fn __gmpf_inits(f: *mut mpf_s, g: *mut mpf_s) -> ();
  /// __gmpf_init
  pub fn __gmpf_init(f: *mut mpf_s) -> ();
  /// __gmpf_init2
  pub fn __gmpf_init2(f: *mut mpf_s, n: mp_bitcnt_t) -> ();
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

  /// __gmpf_get_d
  pub fn __gmpf_get_d(f: *mut mpf_s) -> double_t;
  /// __gmpf_get_ui
  pub fn __gmpf_get_ui(f: *mut mpf_s) -> ui_t;
  /// __gmpf_get_si
  pub fn __gmpf_get_si(f: *mut mpf_s) -> si_t;
  /// __gmpf_get_d_2exp
  pub fn __gmpf_get_d_2exp(e: *mut si_t, f: *mut mpf_s) -> double_t;

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
/*
  /// __gmpf_sgn
  pub fn __gmpf_sgn(f: *const mpf_s) -> int_t;
*/
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

  /// __gmpf_ceil
  pub fn __gmpf_ceil(g: *mut mpf_s, f: *mut mpf_s) -> ();
  /// __gmpf_floor
  pub fn __gmpf_floor(g: *mut mpf_s, f: *mut mpf_s) -> ();
  /// __gmpf_trunc
  pub fn __gmpf_trunc(g: *mut mpf_s, f: *mut mpf_s) -> ();

  /// __gmpf_integer_p
  pub fn __gmpf_integer_p(f: *mut mpf_s) -> int_t;
  /// __gmpf_fits_ulong_p
  pub fn __gmpf_fits_ulong_p(f: *mut mpf_s) -> int_t;
  /// __gmpf_fits_slong_p
  pub fn __gmpf_fits_slong_p(f: *mut mpf_s) -> int_t;
  /// __gmpf_fits_uint_p
  pub fn __gmpf_fits_uint_p(f: *mut mpf_s) -> int_t;
  /// __gmpf_fits_sint_p
  pub fn __gmpf_fits_sint_p(f: *mut mpf_s) -> int_t;
  /// __gmpf_fits_ushort_p
  pub fn __gmpf_fits_ushort_p(f: *mut mpf_s) -> int_t;
  /// __gmpf_fits_sshort_p
  pub fn __gmpf_fits_sshort_p(f: *mut mpf_s) -> int_t;

  /// __gmpf_urandomb (must init random state before)
  pub fn __gmpf_urandomb(g: *mut mpf_s, state: *mut randstate_s,
    nbits: mp_bitcnt_t) -> ();
  /// __gmpf_random2
  pub fn __gmpf_random2(g: *mut mpf_s, max_size: mp_size_t, e: mp_exp_t) -> ();

  /// __gmpf_get_default_prec
  pub fn __gmpf_get_default_prec() -> mp_bitcnt_t;
  /// __gmpf_get_prec
  pub fn __gmpf_get_prec(f: *mut mpf_s) -> mp_bitcnt_t;
  /// __gmpf_set_default_prec
  pub fn __gmpf_set_default_prec(n: mp_bitcnt_t) -> ();
  /// __gmpf_set_prec
  pub fn __gmpf_set_prec(f: *mut mpf_s, n: mp_bitcnt_t) -> ();
  /// __gmpf_set_prec_raw
  pub fn __gmpf_set_prec_raw(f: *mut mpf_s, n: mp_bitcnt_t) -> ();

  /// __gmpq_clears *** CAUTION *** (assume just 1 fake parameters after first)
  pub fn __gmpq_clears(q: *mut mpq_s, r: *mut mpq_s) -> ();
  /// __gmpq_clear
  pub fn __gmpq_clear(q: *mut mpq_s) -> ();
  /// __gmpq_inits *** CAUTION *** (assume just 1 fake parameters after first)
  pub fn __gmpq_inits(q: *mut mpq_s, r: *mut mpq_s) -> ();
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

  /// __gmpq_get_d
  pub fn __gmpq_get_d(q: *mut mpq_s) -> double_t;
  /// __gmpq_get_num
  pub fn __gmpq_get_num(num: *mut mpz_s, q: *mut mpq_s) -> ();
  /// __gmpq_get_den
  pub fn __gmpq_get_den(den: *mut mpz_s, q: *mut mpq_s) -> ();
/*
  /// __gmpq_numref (unsafe)
  pub fn __gmpq_numref(q: *mut mpq_s) -> *mut mpz_s;
  /// __gmpq_denref (unsafe)
  pub fn __gmpq_denref(q: *mut mpq_s) -> *mut mpz_s;
*/

  /// __gmpq_swap
  pub fn __gmpq_swap(q: *mut mpq_s, r: *mut mpq_s) -> ();

  /// __gmpq_cmp
  pub fn __gmpq_cmp(q: *mut mpq_s, r: *mut mpq_s) -> int_t;
  /// __gmpq_cmp_z
  pub fn __gmpq_cmp_z(q: *mut mpq_s, a: *mut mpz_s) -> int_t;
  /// __gmpq_cmp_ui
  pub fn __gmpq_cmp_ui(q: *mut mpq_s, u: ui_t) -> int_t;
  /// __gmpq_cmp_si
  pub fn __gmpq_cmp_si(q: *mut mpq_s, s: si_t) -> int_t;
  /// __gmpq_equal
  pub fn __gmpq_equal(q: *mut mpq_s, r: *mut mpq_s) -> int_t;
/*
  /// __gmpq_sgn
  pub fn __gmpq_sgn(q: *const mpq_s) -> int_t;
*/

  /// __gmpq_inv p = q**-1
  pub fn __gmpq_inv(p: *mut mpq_s, q: *mut mpq_s) -> ();
  /// __gmpq_abs
  pub fn __gmpq_abs(p: *mut mpq_s, q: *mut mpq_s) -> ();
  /// __gmpq_neg
  pub fn __gmpq_neg(p: *mut mpq_s, q: *mut mpq_s) -> ();

  /// __gmpq_sub p = q - r
  pub fn __gmpq_sub(p: *mut mpq_s, q: *mut mpq_s, r: *mut mpq_s) -> ();
  /// __gmpq_add p = q + r
  pub fn __gmpq_add(p: *mut mpq_s, q: *mut mpq_s, r: *mut mpq_s) -> ();

  /// __gmpq_mul p = q * r
  pub fn __gmpq_mul(p: *mut mpq_s, q: *mut mpq_s, r: *mut mpq_s) -> ();
  /// __gmpq_mul_2exp p = q * 2**n
  pub fn __gmpq_mul_2exp(p: *mut mpq_s, q: *mut mpq_s, n: mp_bitcnt_t) -> ();

  /// __gmpq_div p = q / r
  pub fn __gmpq_div(p: *mut mpq_s, q: *mut mpq_s, r: *mut mpq_s) -> ();
  /// __gmpq_div_2exp p = q / 2**n
  pub fn __gmpq_div_2exp(p: *mut mpq_s, q: *mut mpq_s, n: mp_bitcnt_t) -> ();

  /// __gmp_randclear
  pub fn __gmp_randclear(r: *mut randstate_s) -> ();
  /// __gmp_randinit ***more than 1 args (obsoleted)***
  pub fn __gmp_randinit(r: *mut randstate_s,
    a: gmp_randalg_t, sz: mp_bitcnt_t) -> ();
  /// __gmp_randinit_set copy
  pub fn __gmp_randinit_set(r: *mut randstate_s, s: *mut randstate_s) -> ();
  /// __gmp_randinit_default
  pub fn __gmp_randinit_default(r: *mut randstate_s) -> ();
  /// __gmp_randinit_mt
  pub fn __gmp_randinit_mt(r: *mut randstate_s) -> ();
  /// __gmp_randinit_lc_2exp x = (a*x + c) mod 2**m2e
  pub fn __gmp_randinit_lc_2exp(r: *mut randstate_s,
    a: *mut mpz_s, c: ui_t, m2e: mp_bitcnt_t) -> ();
  /// __gmp_randinit_lc_2exp_size
  pub fn __gmp_randinit_lc_2exp_size(r: *mut randstate_s,
    sz: mp_bitcnt_t) -> int_t;

  /// __gmp_randseed
  pub fn __gmp_randseed(r: *mut randstate_s, seed: *mut mpz_s) -> ();
  /// __gmp_randseed_ui
  pub fn __gmp_randseed_ui(r: *mut randstate_s, seed: ui_t) -> ();

  /// __gmp_urandomb_ui
  pub fn __gmp_urandomb_ui(r: *mut randstate_s, nbits: ui_t) -> ui_t;
  /// __gmp_urandomm_ui
  pub fn __gmp_urandomm_ui(r: *mut randstate_s, n: ui_t) -> ui_t;

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
