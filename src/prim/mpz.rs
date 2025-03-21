//! mpz
//!

use std::fmt;
use std::error::Error;
use std::collections::HashMap;

use crate::prim::{*, typ::*, randstate::*, gmp::*}; // mpf::*, mpq::*

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

/// impl Drop
impl Drop for __mpz_struct {
  fn drop(&mut self) {
    self.clear()
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

  /// init2 with prec create new instance
  pub fn init2(n: mp_bitcnt_t) -> Self {
    let mut t = mpz_s::new();
    mpz_init2(&mut t, n);
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

  /// fmtstr
  pub fn fmtstr(&mut self, b: int_t) -> String {
    mpz_get_str(None, b, self).expect("mpz fmtstr")
  }

  /// get_d (loss of digits)
  pub fn get_d(&mut self) -> double_t {
    mpz_get_d(self)
  }

  /// get_ui (loss of digits)
  pub fn get_ui(&mut self) -> ui_t {
    mpz_get_ui(self)
  }

  /// get_si (loss of digits)
  pub fn get_si(&mut self) -> si_t {
    mpz_get_si(self)
  }

  /// get_d_2exp (loss of digits)
  pub fn get_d_2exp(&mut self) -> (double_t, si_t) {
    let mut e: si_t = 0;
    let d = mpz_get_d_2exp(&mut e, self);
    (d, e)
  }

  /// swap
  pub fn swap(&mut self, b: mpz_t) -> &mut Self {
    mpz_swap(self, b);
    self
  }

  /// realloc2
  pub fn realloc2(&mut self, n: mp_bitcnt_t) -> &mut Self {
    mpz_realloc2(self, n);
    self
  }

  /// _realloc
  pub fn _realloc(&mut self, sz: mp_size_t) -> &mut Self {
    _mpz_realloc(self, sz);
    self
  }

  /// size
  pub fn size(&mut self) -> mp_size_t {
    mpz_size(self)
  }

  /// limbs_read slice
  pub fn limbs_read(&mut self) -> &mut [mp_limb_t] {
    mpz_limbs_read(self)
  }

  /// getlimbn (single element)
  pub fn getlimbn(&mut self, n: mp_size_t) -> mp_limb_t {
    mpz_getlimbn(self, n)
  }

  /// limbs_write slice
  pub fn limbs_write(&mut self, sz: mp_size_t) -> &mut [mp_limb_t] {
    mpz_limbs_write(self, sz)
  }

  /// limbs_modify slice
  pub fn limbs_modify(&mut self, sz: mp_size_t) -> &mut [mp_limb_t] {
    mpz_limbs_modify(self, sz)
  }

  /// limbs_finish (used after write or modify to update internal size)
  pub fn limbs_finish(&mut self, sz: mp_size_t) -> &mut Self {
    mpz_limbs_finish(self, sz);
    self
  }

  /// roinit_n (unsafe) slice single element
  pub fn roinit_n(&mut self, p: &mut [mp_limb_t], sz: mp_size_t) -> &mut Self {
    mpz_roinit_n(self, p, sz)
  }

  /// cmp
  pub fn cmp(&mut self, b: mpz_t) -> int_t {
    mpz_cmp(self, b)
  }

  /// cmp_d
  pub fn cmp_d(&mut self, d: double_t) -> int_t {
    mpz_cmp_d(self, d)
  }

  /// cmp_ui
  pub fn cmp_ui(&mut self, u: ui_t) -> int_t {
    mpz_cmp_ui(self, u)
  }

  /// cmp_si
  pub fn cmp_si(&mut self, s: si_t) -> int_t {
    mpz_cmp_si(self, s)
  }

  /// cmpabs
  pub fn cmpabs(&mut self, b: mpz_t) -> int_t {
    mpz_cmpabs(self, b)
  }

  /// cmpabs_d
  pub fn cmpabs_d(&mut self, d: double_t) -> int_t {
    mpz_cmpabs_d(self, d)
  }

  /// cmpabs_ui
  pub fn cmpabs_ui(&mut self, u: ui_t) -> int_t {
    mpz_cmpabs_ui(self, u)
  }

  /// sgn
  pub fn sgn(&mut self) -> int_t {
    mpz_sgn(self)
  }

  /// root nth root of self create new instance
  pub fn root(&mut self, n: ui_t) -> (Self, bool) {
    let mut t = mpz_s::init();
    let f = mpz_root(&mut t, self, n);
    (t, f)
  }

  /// rootrem (nth root of self, self - root**n) create new instance
  pub fn rootrem(&mut self, n: ui_t) -> (Self, Self) {
    let mut t = mpz_s::init();
    let mut rem = mpz_s::init();
    mpz_rootrem(&mut t, &mut rem, self, n);
    (t, rem)
  }

  /// sqrt square root of self create new instance
  pub fn sqrt(&mut self) -> Self {
    let mut t = mpz_s::init();
    mpz_sqrt(&mut t, self);
    t
  }

  /// sqrtrem (square root of self, self - root**2) create new instance
  pub fn sqrtrem(&mut self) -> (Self, Self) {
    let mut t = mpz_s::init();
    let mut rem = mpz_s::init();
    mpz_sqrtrem(&mut t, &mut rem, self);
    (t, rem)
  }

  /// perfect_power_p
  pub fn perfect_power_p(&mut self) -> bool {
    mpz_perfect_power_p(self)
  }

  /// perfect_square_p
  pub fn perfect_square_p(&mut self) -> bool {
    mpz_perfect_square_p(self)
  }

  /// primorial_ui c = 2*3*5*7*11*...*p(prev)*p(&lt;=n) create new instance
  pub fn primorial_ui(n: ui_t) -> Self {
    let mut t = mpz_s::init_set_ui(1);
    mpz_primorial_ui(&mut t, n);
    t
  }

  /// fac_ui create new instance
  pub fn fac_ui(n: ui_t) -> Self {
    let mut t = mpz_s::init_set_ui(1);
    mpz_fac_ui(&mut t, n);
    t
  }

  /// fac2_ui create new instance
  pub fn fac2_ui(n: ui_t) -> Self {
    let mut t = mpz_s::init_set_ui(1);
    mpz_2fac_ui(&mut t, n);
    t
  }

  /// mfac_uiui create new instance
  pub fn mfac_uiui(n: ui_t, m: ui_t) -> Self {
    let mut t = mpz_s::init_set_ui(1);
    mpz_mfac_uiui(&mut t, n, m);
    t
  }

  /// remove create new instance
  pub fn remove(&mut self, f: mpz_t) -> (Self, mp_bitcnt_t) {
    let mut t = mpz_s::init();
    let n = mpz_remove(&mut t, self, f);
    (t, n)
  }

  /// fib_ui create new instance
  pub fn fib_ui(n: ui_t) -> Self {
    let mut f_n = mpz_s::init_set_ui(1);
    mpz_fib_ui(&mut f_n, n);
    f_n
  }

  /// fib2_ui create new instance (f_n, f_nsub1)
  pub fn fib2_ui(n: ui_t) -> (Self, Self) {
    let mut f_n = mpz_s::init_set_ui(1);
    let mut f_nsub1 = mpz_s::init_set_ui(1);
    mpz_fib2_ui(&mut f_n, &mut f_nsub1, n);
    (f_n, f_nsub1)
  }

  /// lucnum_ui create new instance
  pub fn lucnum_ui(n: ui_t) -> Self {
    let mut l_n = mpz_s::init_set_ui(1);
    mpz_lucnum_ui(&mut l_n, n);
    l_n
  }

  /// lucnum2_ui create new instance (l_n, l_n_1)
  pub fn lucnum2_ui(n: ui_t) -> (Self, Self) {
    let mut l_n = mpz_s::init_set_ui(1);
    let mut l_n_1 = mpz_s::init_set_ui(1);
    mpz_lucnum2_ui(&mut l_n, &mut l_n_1, n);
    (l_n, l_n_1)
  }

  /// gcd create new instance
  pub fn gcd(&mut self, b: mpz_t) -> Self {
    let mut gcd = mpz_s::init_set_ui(1);
    mpz_gcd(&mut gcd, self, b);
    gcd
  }

  /// gcd_ui create new instance (gcd, gcd: ui_t)
  /// return 0 when gcd does not fit to ui_t
  pub fn gcd_ui(&mut self, u: ui_t) -> (Self, ui_t) {
    let mut gcd = mpz_s::init_set_ui(1);
    let u = mpz_gcd_ui(&mut gcd, self, u);
    (gcd, u)
  }

  /// gcdext create new instance (gcd, s, t)
  /// s and t to coefficients satisfying a*s + b*t == gcd
  pub fn gcdext(&mut self, b: mpz_t) -> (Self, Self, Self) {
    let mut gcd = mpz_s::init_set_ui(1);
    let mut s = mpz_s::init_set_ui(1);
    let mut t = mpz_s::init_set_ui(1);
    mpz_gcdext(&mut gcd, &mut s, &mut t, self, b);
    (gcd, s, t)
  }

  /// lcm create new instance
  pub fn lcm(&mut self, b: mpz_t) -> Self {
    let mut t = mpz_s::init_set_ui(1);
    mpz_lcm(&mut t, self, b);
    t
  }

  /// lcm_ui create new instance
  pub fn lcm_ui(&mut self, u: ui_t) -> Self {
    let mut t = mpz_s::init_set_ui(1);
    mpz_lcm_ui(&mut t, self, u);
    t
  }

  /// probab_prime_p 2 or 1 or 0
  pub fn probab_prime_p(&mut self, r: int_t) -> int_t {
    mpz_probab_prime_p(self, r)
  }

  /// nextprime create new instance
  pub fn nextprime(&mut self) -> Self {
    let mut t = mpz_s::init();
    mpz_nextprime(&mut t, self);
    t
  }

/*
  /// prevprime create new instance
  pub fn prevprime(&mut self) -> Self {
    let mut t = mpz_s::init();
    mpz_prevprime(&mut t, self);
    t
  }
*/

  /// invert create new instance c = inverse of a mod b ((c*a) mod b == 1)
  /// returns (undefined, 0) when not exist inverse
  pub fn invert(a: mpz_t, b: mpz_t) -> (Self, int_t) {
    let mut t = mpz_s::init();
    let r = mpz_invert(&mut t, a, b);
    (t, r)
  }

  /// jacobi 0 1 -1 (defined only for n odd)
  pub fn jacobi(&mut self, n: mpz_t) -> int_t {
    mpz_jacobi(self, n)
  }

  /// legendre 0 1 -1 (defined only for p an odd positive prime)
  pub fn legendre(&mut self, p: mpz_t) -> int_t {
    mpz_legendre(self, p)
  }

  /// kronecker
  pub fn kronecker(&mut self, n: mpz_t) -> int_t {
    mpz_kronecker(self, n)
  }

  /// kronecker_ui
  pub fn kronecker_ui(&mut self, u: ui_t) -> int_t {
    mpz_kronecker_ui(self, u)
  }

  /// kronecker_si
  pub fn kronecker_si(&mut self, s: si_t) -> int_t {
    mpz_kronecker_si(self, s)
  }

  /// ui_kronecker
  pub fn ui_kronecker(&mut self, u: ui_t) -> int_t {
    mpz_ui_kronecker(u, self)
  }

  /// si_kronecker
  pub fn si_kronecker(&mut self, s: si_t) -> int_t {
    mpz_si_kronecker(s, self)
  }

  /// bin_ui nCk create new instance
  pub fn bin_ui(n: mpz_t, k: ui_t) -> Self {
    let mut t = mpz_s::init();
    mpz_bin_ui(&mut t, n, k);
    t
  }

  /// bin_uiui nCk create new instance
  pub fn bin_uiui(n: ui_t, k: ui_t) -> Self {
    let mut t = mpz_s::init();
    mpz_bin_uiui(&mut t, n, k);
    t
  }

  /// abs create new instance
  pub fn abs(&mut self) -> Self {
    let mut t = mpz_s::new();
    mpz_abs(&mut t, self);
    t
  }

  /// neg create new instance
  pub fn neg(&mut self) -> Self {
    let mut t = mpz_s::new();
    mpz_neg(&mut t, self);
    t
  }

  /// sub self -= b
  pub fn sub(&mut self, b: mpz_t) -> &mut Self {
    let t = &mut mpz_s::init_set(self);
    mpz_sub(self, t, b);
    self
  }

  /// sub_ui self -= u
  pub fn sub_ui(&mut self, u: ui_t) -> &mut Self {
    let t = &mut mpz_s::init_set(self);
    mpz_sub_ui(self, t, u);
    self
  }

  /// ui_sub self = u - self
  pub fn ui_sub(&mut self, u: ui_t) -> &mut Self {
    let t = &mut mpz_s::init_set(self);
    mpz_ui_sub(self, u, t);
    self
  }

  /// submul self -= a * b
  pub fn submul(&mut self, a: mpz_t, b: mpz_t) -> &mut Self {
    mpz_submul(self, a, b);
    self
  }

  /// submul_ui self -= a * u
  pub fn submul_ui(&mut self, a: mpz_t, u: ui_t) -> &mut Self {
    mpz_submul_ui(self, a, u);
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

  /// cdiv_q create new instance
  pub fn cdiv_q(&mut self, d: mpz_t) -> Self {
    let mut t = mpz_s::init();
    mpz_cdiv_q(&mut t, self, d);
    t
  }

  /// cdiv_r create new instance
  pub fn cdiv_r(&mut self, d: mpz_t) -> Self {
    let mut t = mpz_s::init();
    mpz_cdiv_r(&mut t, self, d);
    t
  }

  /// cdiv_qr create new instance
  pub fn cdiv_qr(&mut self, d: mpz_t) -> (Self, Self) {
    let mut t = mpz_s::init();
    let mut r = mpz_s::init();
    mpz_cdiv_qr(&mut t, &mut r, self, d);
    (t, r)
  }

  /// cdiv_q_ui create new instance
  pub fn cdiv_q_ui(&mut self, d: ui_t) -> (Self, ui_t) {
    let mut t = mpz_s::init();
    let u = mpz_cdiv_q_ui(&mut t, self, d);
    (t, u)
  }

  /// cdiv_r_ui create new instance
  pub fn cdiv_r_ui(&mut self, d: ui_t) -> (Self, ui_t) {
    let mut t = mpz_s::init();
    let u = mpz_cdiv_r_ui(&mut t, self, d);
    (t, u)
  }

  /// cdiv_qr_ui create new instance
  pub fn cdiv_qr_ui(&mut self, d: ui_t) -> (Self, Self, ui_t) {
    let mut t = mpz_s::init();
    let mut r = mpz_s::init();
    let u = mpz_cdiv_qr_ui(&mut t, &mut r, self, d);
    (t, r, u)
  }

  /// cdiv_ui
  pub fn cdiv_ui(&mut self, d: ui_t) -> ui_t {
    mpz_cdiv_ui(self, d)
  }

  /// cdiv_q_2exp create new instance
  pub fn cdiv_q_2exp(&mut self, b: mp_bitcnt_t) -> Self {
    let mut t = mpz_s::init();
    mpz_cdiv_q_2exp(&mut t, self, b);
    t
  }

  /// cdiv_r_2exp create new instance
  pub fn cdiv_r_2exp(&mut self, b: mp_bitcnt_t) -> Self {
    let mut t = mpz_s::init();
    mpz_cdiv_r_2exp(&mut t, self, b);
    t
  }

  /// fdiv_q create new instance
  pub fn fdiv_q(&mut self, d: mpz_t) -> Self {
    let mut t = mpz_s::init();
    mpz_fdiv_q(&mut t, self, d);
    t
  }

  /// fdiv_r create new instance
  pub fn fdiv_r(&mut self, d: mpz_t) -> Self {
    let mut t = mpz_s::init();
    mpz_fdiv_r(&mut t, self, d);
    t
  }

  /// fdiv_qr create new instance
  pub fn fdiv_qr(&mut self, d: mpz_t) -> (Self, Self) {
    let mut t = mpz_s::init();
    let mut r = mpz_s::init();
    mpz_fdiv_qr(&mut t, &mut r, self, d);
    (t, r)
  }

  /// fdiv_q_ui create new instance
  pub fn fdiv_q_ui(&mut self, d: ui_t) -> (Self, ui_t) {
    let mut t = mpz_s::init();
    let u = mpz_fdiv_q_ui(&mut t, self, d);
    (t, u)
  }

  /// fdiv_r_ui create new instance
  pub fn fdiv_r_ui(&mut self, d: ui_t) -> (Self, ui_t) {
    let mut t = mpz_s::init();
    let u = mpz_fdiv_r_ui(&mut t, self, d);
    (t, u)
  }

  /// fdiv_qr_ui create new instance
  pub fn fdiv_qr_ui(&mut self, d: ui_t) -> (Self, Self, ui_t) {
    let mut t = mpz_s::init();
    let mut r = mpz_s::init();
    let u = mpz_fdiv_qr_ui(&mut t, &mut r, self, d);
    (t, r, u)
  }

  /// fdiv_ui
  pub fn fdiv_ui(&mut self, d: ui_t) -> ui_t {
    mpz_fdiv_ui(self, d)
  }

  /// fdiv_q_2exp create new instance
  pub fn fdiv_q_2exp(&mut self, b: mp_bitcnt_t) -> Self {
    let mut t = mpz_s::init();
    mpz_fdiv_q_2exp(&mut t, self, b);
    t
  }

  /// fdiv_r_2exp create new instance
  pub fn fdiv_r_2exp(&mut self, b: mp_bitcnt_t) -> Self {
    let mut t = mpz_s::init();
    mpz_fdiv_r_2exp(&mut t, self, b);
    t
  }

  /// tdiv_q create new instance
  pub fn tdiv_q(&mut self, d: mpz_t) -> Self {
    let mut t = mpz_s::init();
    mpz_tdiv_q(&mut t, self, d);
    t
  }

  /// tdiv_r create new instance
  pub fn tdiv_r(&mut self, d: mpz_t) -> Self {
    let mut t = mpz_s::init();
    mpz_tdiv_r(&mut t, self, d);
    t
  }

  /// tdiv_qr create new instance
  pub fn tdiv_qr(&mut self, d: mpz_t) -> (Self, Self) {
    let mut t = mpz_s::init();
    let mut r = mpz_s::init();
    mpz_tdiv_qr(&mut t, &mut r, self, d);
    (t, r)
  }

  /// tdiv_q_ui create new instance
  pub fn tdiv_q_ui(&mut self, d: ui_t) -> (Self, ui_t) {
    let mut t = mpz_s::init();
    let u = mpz_tdiv_q_ui(&mut t, self, d);
    (t, u)
  }

  /// tdiv_r_ui create new instance
  pub fn tdiv_r_ui(&mut self, d: ui_t) -> (Self, ui_t) {
    let mut t = mpz_s::init();
    let u = mpz_tdiv_r_ui(&mut t, self, d);
    (t, u)
  }

  /// tdiv_qr_ui create new instance
  pub fn tdiv_qr_ui(&mut self, d: ui_t) -> (Self, Self, ui_t) {
    let mut t = mpz_s::init();
    let mut r = mpz_s::init();
    let u = mpz_tdiv_qr_ui(&mut t, &mut r, self, d);
    (t, r, u)
  }

  /// tdiv_ui
  pub fn tdiv_ui(&mut self, d: ui_t) -> ui_t {
    mpz_tdiv_ui(self, d)
  }

  /// tdiv_q_2exp create new instance
  pub fn tdiv_q_2exp(&mut self, b: mp_bitcnt_t) -> Self {
    let mut t = mpz_s::init();
    mpz_tdiv_q_2exp(&mut t, self, b);
    t
  }

  /// tdiv_r_2exp create new instance
  pub fn tdiv_r_2exp(&mut self, b: mp_bitcnt_t) -> Self {
    let mut t = mpz_s::init();
    mpz_tdiv_r_2exp(&mut t, self, b);
    t
  }

  /// modulo create new instance
  pub fn modulo(&mut self, d: mpz_t) -> Self {
    let mut t = mpz_s::init();
    mpz_mod(&mut t, self, d);
    t
  }

  /// mod_ui (the result is always non-negative) create new instance
  pub fn mod_ui(&mut self, d: ui_t) -> (Self, ui_t) {
    let mut t = mpz_s::init();
    let m = mpz_mod_ui(&mut t, self, d);
    (t, m)
  }

  /// divexact create new instance
  pub fn divexact(&mut self, d: mpz_t) -> Self {
    let mut t = mpz_s::init();
    mpz_divexact(&mut t, self, d);
    t
  }

  /// divexact_ui create new instance
  pub fn divexact_ui(&mut self, d: ui_t) -> Self {
    let mut t = mpz_s::init();
    mpz_divexact_ui(&mut t, self, d);
    t
  }

  /// divisible_p
  pub fn divisible_p(&mut self, d: mpz_t) -> bool {
    mpz_divisible_p(self, d)
  }

  /// divisible_ui_p
  pub fn divisible_ui_p(&mut self, d: ui_t) -> bool {
    mpz_divisible_ui_p(self, d)
  }

  /// divisible_2exp_p
  pub fn divisible_2exp_p(&mut self, b: mp_bitcnt_t) -> bool {
    mpz_divisible_2exp_p(self, b)
  }

  /// congruent_p
  pub fn congruent_p(&mut self, c: mpz_t, d: mpz_t) -> bool {
    mpz_congruent_p(self, c, d)
  }

  /// congruent_ui_p
  pub fn congruent_ui_p(&mut self, c: ui_t, d: ui_t) -> bool {
    mpz_congruent_ui_p(self, c, d)
  }

  /// congruent_2exp_p
  pub fn congruent_2exp_p(&mut self, c: mpz_t, b: mp_bitcnt_t) -> bool {
    mpz_congruent_2exp_p(self, c, b)
  }

  /// powwm_sec (a**n) mod m ***required n &gt; 0 and m is odd*** create new instance
  pub fn powm_sec(a: mpz_t, n: mpz_t, m: mpz_t) -> Self {
    let mut t = mpz_s::new();
    mpz_powm_sec(&mut t, a, n, m);
    t
  }

  /// powm (a**n) mod m ***n &lt; 0 when exists inv a**-1 mod m*** create new instance
  pub fn powm(a: mpz_t, n: mpz_t, m: mpz_t) -> Self {
    let mut t = mpz_s::new();
    mpz_powm(&mut t, a, n, m);
    t
  }

  /// powm_ui (a**n) mod m create new instance
  pub fn powm_ui(a: mpz_t, n: ui_t, m: mpz_t) -> Self {
    let mut t = mpz_s::new();
    mpz_powm_ui(&mut t, a, n, m);
    t
  }

  /// pow_ui a**n create new instance
  pub fn pow_ui(a: mpz_t, n: ui_t) -> Self {
    let mut t = mpz_s::new();
    mpz_pow_ui(&mut t, a, n);
    t
  }

  /// ui_pow_ui a**n create new instance
  pub fn ui_pow_ui(a: ui_t, n: ui_t) -> Self {
    let mut t = mpz_s::new();
    mpz_ui_pow_ui(&mut t, a, n);
    t
  }

  /// sizeinbase
  pub fn sizeinbase(&mut self, base: int_t) -> mp_size_t {
    mpz_sizeinbase(self, base)
  }

  /// even_p
  pub fn even_p(&mut self) -> bool {
    mpz_even_p(self)
  }

  /// odd_p
  pub fn odd_p(&mut self) -> bool {
    mpz_odd_p(self)
  }

  /// fits_ulong_p
  pub fn fits_ulong_p(&mut self) -> bool {
    mpz_fits_ulong_p(self)
  }

  /// fits_slong_p
  pub fn fits_slong_p(&mut self) -> bool {
    mpz_fits_slong_p(self)
  }

  /// fits_uint_p
  pub fn fits_uint_p(&mut self) -> bool {
    mpz_fits_uint_p(self)
  }

  /// fits_sint_p
  pub fn fits_sint_p(&mut self) -> bool {
    mpz_fits_sint_p(self)
  }

  /// fits_ushort_p
  pub fn fits_ushort_p(&mut self) -> bool {
    mpz_fits_ushort_p(self)
  }

  /// fits_sshort_p
  pub fn fits_sshort_p(&mut self) -> bool {
    mpz_fits_sshort_p(self)
  }

  /// urandomb create new instance
  pub fn urandomb(r: randstate_t, nbits: mp_bitcnt_t) -> Self {
    let mut t = mpz_s::init_set_ui(0);
    mpz_urandomb(&mut t, r, nbits);
    t
  }

  /// urandomm create new instance
  pub fn urandomm(r: randstate_t, n: mpz_t) -> Self {
    let mut t = mpz_s::init_set_ui(0);
    mpz_urandomm(&mut t, r, n);
    t
  }

  /// rrandomb create new instance
  pub fn rrandomb(r: randstate_t, nbits: mp_bitcnt_t) -> Self {
    let mut t = mpz_s::init_set_ui(0);
    mpz_rrandomb(&mut t, r, nbits);
    t
  }

  /// random create new instance ***(obsoleted) urandomb or urandomm instead***
  pub fn random(max_size: mp_size_t) -> Self {
    let mut t = mpz_s::init_set_ui(0);
    mpz_random(&mut t, max_size);
    t
  }

  /// random2 create new instance
  pub fn random2(max_size: mp_size_t) -> Self {
    let mut t = mpz_s::init_set_ui(0);
    mpz_random2(&mut t, max_size);
    t
  }

  /// and create new instance
  pub fn and(&mut self, b: mpz_t) -> Self {
    let mut t = mpz_s::init();
    mpz_and(&mut t, self, b);
    t
  }

  /// ior create new instance
  pub fn ior(&mut self, b: mpz_t) -> Self {
    let mut t = mpz_s::init();
    mpz_ior(&mut t, self, b);
    t
  }

  /// xor create new instance
  pub fn xor(&mut self, b: mpz_t) -> Self {
    let mut t = mpz_s::init();
    mpz_xor(&mut t, self, b);
    t
  }

  /// com create new instance
  pub fn com(&mut self) -> Self {
    let mut t = mpz_s::init();
    mpz_com(&mut t, self);
    t
  }

  /// popcount
  pub fn popcount(&mut self) -> mp_bitcnt_t {
    mpz_popcount(self)
  }

  /// hamdist hamming distance between a and b (both sgn must be same)
  pub fn hamdist(&mut self, b: mpz_t) -> mp_bitcnt_t {
    mpz_hamdist(self, b)
  }

  /// scan0 to msb
  pub fn scan0(&mut self, s: mp_bitcnt_t) -> mp_bitcnt_t {
    mpz_scan0(self, s)
  }

  /// scan1 to msb
  pub fn scan1(&mut self, s: mp_bitcnt_t) -> mp_bitcnt_t {
    mpz_scan1(self, s)
  }

  /// clrbit
  pub fn clrbit(&mut self, n: mp_bitcnt_t) -> &mut Self {
    mpz_clrbit(self, n);
    self
  }

  /// setbit
  pub fn setbit(&mut self, n: mp_bitcnt_t) -> &mut Self {
    mpz_setbit(self, n);
    self
  }

  /// combit
  pub fn combit(&mut self, n: mp_bitcnt_t) -> &mut Self {
    mpz_combit(self, n);
    self
  }

  /// tstbit
  pub fn tstbit(&mut self, n: mp_bitcnt_t) -> bool {
    mpz_tstbit(self, n)
  }

  /// fact create new instance (slow without cache)
  pub fn fact(n: ui_t) -> Self {
    let mut t = mpz_s::init_set_ui(1);
    (1..=n).for_each(|i| { t.mul_ui(i); });
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
//    write!(f, "{}", self.fmtstr(10)) // cannot be borrowed as mutable
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

/// mpz_inits
pub fn mpz_inits(va: &mut Vec<mpz_t>) -> () {
  va.iter_mut().for_each(|a| {
    unsafe { __gmpz_init(*a) } // not use __gmpz_inits
  })
}

/// mpz_init
pub fn mpz_init(a: mpz_t) -> () {
  unsafe { __gmpz_init(a) }
}

/// mpz_init2
pub fn mpz_init2(a: mpz_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpz_init2(a, n) }
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

/// mpz_get_d
pub fn mpz_get_d(a: mpz_t) -> double_t {
  unsafe { __gmpz_get_d(a) }
}

/// mpz_get_ui
pub fn mpz_get_ui(a: mpz_t) -> ui_t {
  unsafe { __gmpz_get_ui(a) }
}

/// mpz_get_si
pub fn mpz_get_si(a: mpz_t) -> si_t {
  unsafe { __gmpz_get_si(a) }
}

/// mpz_get_d_2exp
pub fn mpz_get_d_2exp(e: &mut si_t, a: mpz_t) -> double_t {
  unsafe { __gmpz_get_d_2exp(e, a) }
}

/// mpz_swap
pub fn mpz_swap(a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_swap(a, b) }
}

/// mpz_realloc2
pub fn mpz_realloc2(a: mpz_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpz_realloc2(a, n) }
}

/// _mpz_realloc
pub fn _mpz_realloc(a: mpz_t, sz: mp_size_t) -> mp_t {
  unsafe { __gmpz_realloc(a, sz) }
}

/// mpz_array_init ***(obsoleted) do NOT use it***
pub fn mpz_array_init(a: *mut mpz_s, sz: mp_size_t, fnb: mp_size_t) -> () {
  unsafe { __gmpz_array_init(a, sz, fnb) }
}

/// mpz_size
pub fn mpz_size(a: *mut mpz_s) -> mp_size_t {
  unsafe { __gmpz_size(a) }
}

/// mpz_limbs_read slice
pub fn mpz_limbs_read(a: mpz_t) -> &mut [mp_limb_t] {
  let sz = mpz_size(a);
  unsafe { std::slice::from_raw_parts_mut(__gmpz_limbs_read(a), sz) }
}

/// mpz_getlimbn (single element)
pub fn mpz_getlimbn(a: mpz_t, n: mp_size_t) -> mp_limb_t {
  unsafe { __gmpz_getlimbn(a, n) }
}

/// mpz_limbs_write slice
pub fn mpz_limbs_write(a: mpz_t, sz: mp_size_t) -> &mut [mp_limb_t] {
  unsafe { std::slice::from_raw_parts_mut(__gmpz_limbs_write(a, sz), sz) }
}

/// mpz_limbs_modify slice
pub fn mpz_limbs_modify(a: mpz_t, sz: mp_size_t) -> &mut [mp_limb_t] {
  unsafe { std::slice::from_raw_parts_mut(__gmpz_limbs_modify(a, sz), sz) }
}

/// mpz_limbs_finish (used after write or modify to update internal size)
pub fn mpz_limbs_finish(a: mpz_t, sz: mp_size_t) -> () {
  unsafe { __gmpz_limbs_finish(a, sz) }
}

/// mpz_roinit_n (unsafe) slice single element
pub fn mpz_roinit_n<'a>(a: mpz_t,
  p: &mut [mp_limb_t], sz: mp_size_t) -> mpz_t<'a> {
  unsafe {
    let q = __gmpz_roinit_n(a, p as *mut [mp_limb_t] as *mut mp_limb_t, sz);
    &mut std::slice::from_raw_parts_mut(q, 1)[0]
  }
}

/// MPZ_ROINIT_N (unsafe) create new instance of mpz_s
#[allow(non_snake_case)]
pub fn MPZ_ROINIT_N(p: &mut [mp_limb_t], sz: mp_size_t) -> mpz_s {
  __mpz_struct {
    _mp_alloc: 0,
    _mp_size: sz as int_t,
    _mp_d: p as *mut [mp_limb_t] as *mut mp_limb_t
  }
}

/// mpz_cmp
pub fn mpz_cmp(a: mpz_t, b: mpz_t) -> int_t {
  unsafe { __gmpz_cmp(a, b) }
}

/// mpz_cmp_d
pub fn mpz_cmp_d(a: mpz_t, d: double_t) -> int_t {
  unsafe { __gmpz_cmp_d(a, d) }
}

/// mpz_cmp_ui
pub fn mpz_cmp_ui(a: mpz_t, u: ui_t) -> int_t {
  unsafe { __gmpz_cmp_ui(a, u) }
}

/// mpz_cmp_si
pub fn mpz_cmp_si(a: mpz_t, s: si_t) -> int_t {
  unsafe { __gmpz_cmp_si(a, s) }
}

/// mpz_cmpabs
pub fn mpz_cmpabs(a: mpz_t, b: mpz_t) -> int_t {
  unsafe { __gmpz_cmpabs(a, b) }
}

/// mpz_cmpabs_d
pub fn mpz_cmpabs_d(a: mpz_t, d: double_t) -> int_t {
  unsafe { __gmpz_cmpabs_d(a, d) }
}

/// mpz_cmpabs_ui
pub fn mpz_cmpabs_ui(a: mpz_t, u: ui_t) -> int_t {
  unsafe { __gmpz_cmpabs_ui(a, u) }
}

/// mpz_sgn
pub fn mpz_sgn(a: mpz_t) -> int_t {
//  unsafe { __gmpz_sgn(a) }
  let t = a._mp_size;
  if t < 0 { -1 } else { if t > 0 { 1 } else { 0 } }
}

/// mpz_root r = nth root of a
pub fn mpz_root(r: mpz_t, a: mpz_t, n: ui_t) -> bool {
  unsafe { __gmpz_root(r, a, n) != 0 }
}

/// mpz_rootrem r = nth root of u, rem = u - r**n (to the remainder)
pub fn mpz_rootrem(r: mpz_t, rem: mpz_t, u: mpz_t, n: ui_t) -> () {
  unsafe { __gmpz_rootrem(r, rem, u, n) }
}

/// mpz_sqrt r = square root of a
pub fn mpz_sqrt(r: mpz_t, a: mpz_t) -> () {
  unsafe { __gmpz_sqrt(r, a) }
}

/// mpz_sqrtrem r = square root of u, rem = u - r**2 (to the remainder)
pub fn mpz_sqrtrem(r: mpz_t, rem: mpz_t, u: mpz_t) -> () {
  unsafe { __gmpz_sqrtrem(r, rem, u) }
}

/// mpz_perfect_power_p
pub fn mpz_perfect_power_p(a: mpz_t) -> bool {
  unsafe { __gmpz_perfect_power_p(a) != 0 }
}

/// mpz_perfect_square_p
pub fn mpz_perfect_square_p(a: mpz_t) -> bool {
  unsafe { __gmpz_perfect_square_p(a) != 0 }
}

/// mpz_primorial_ui c = 2*3*5*7*11*...*p(prev)*p(&lt;=n)
pub fn mpz_primorial_ui(c: mpz_t, n: ui_t) -> () {
  unsafe { __gmpz_primorial_ui(c, n) }
}

/// mpz_fac_ui c = n!
pub fn mpz_fac_ui(c: mpz_t, n: ui_t) -> () {
  unsafe { __gmpz_fac_ui(c, n) }
}

/// mpz_2fac_ui c = n!!
pub fn mpz_2fac_ui(c: mpz_t, n: ui_t) -> () {
  unsafe { __gmpz_2fac_ui(c, n) }
}

/// mpz_mfac_uiui c = n! ** m
pub fn mpz_mfac_uiui(c: mpz_t, n: ui_t, m: ui_t) -> () {
  unsafe { __gmpz_mfac_uiui(c, n, m) }
}

/// mpz_remove
pub fn mpz_remove(c: mpz_t, a: mpz_t, f: mpz_t) -> mp_bitcnt_t {
  unsafe { __gmpz_remove(c, a, f) }
}

/// mpz_fib_ui
pub fn mpz_fib_ui(f_n: mpz_t, n: ui_t) -> () {
  unsafe { __gmpz_fib_ui(f_n, n) }
}

/// mpz_fib2_ui
pub fn mpz_fib2_ui(f_n: mpz_t, f_nsub1: mpz_t, n: ui_t) -> () {
  unsafe { __gmpz_fib2_ui(f_n, f_nsub1, n) }
}

/// mpz_lucnum_ui
pub fn mpz_lucnum_ui(l_n: mpz_t, n: ui_t) -> () {
  unsafe { __gmpz_lucnum_ui(l_n, n) }
}

/// mpz_lucnum2_ui
pub fn mpz_lucnum2_ui(l_n: mpz_t, l_n_1: mpz_t, n: ui_t) -> () {
  unsafe { __gmpz_lucnum2_ui(l_n, l_n_1, n) }
}

/// mpz_gcd
pub fn mpz_gcd(g: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_gcd(g, a, b) }
}

/// mpz_gcd_ui return 0 when gcd does not fit to ui_t
pub fn mpz_gcd_ui(g: mpz_t, a: mpz_t, u: ui_t) -> ui_t {
  unsafe { __gmpz_gcd_ui(g, a, u) }
}

/// mpz_gcdext s and t to coefficients satisfying a*s + b*t == gcd
pub fn mpz_gcdext(g: mpz_t, s: mpz_t, t: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_gcdext(g, s, t, a, b) }
}

/// mpz_lcm
pub fn mpz_lcm(c: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_lcm(c, a, b) }
}

/// mpz_lcm_ui
pub fn mpz_lcm_ui(c: mpz_t, a: mpz_t, u: ui_t) -> () {
  unsafe { __gmpz_lcm_ui(c, a, u) }
}

/// mpz_probab_prime_p 2 or 1 or 0
pub fn mpz_probab_prime_p(a: mpz_t, r: int_t) -> int_t {
  unsafe { __gmpz_probab_prime_p(a, r) }
}

/// mpz_nextprime
pub fn mpz_nextprime(c: mpz_t, a: mpz_t) -> () {
  unsafe { __gmpz_nextprime(c, a) }
}

/*
/// mpz_prevprime
pub fn mpz_prevprime(c: mpz_t, a: mpz_t) -> () {
  unsafe { __gmpz_prevprime(c, a) }
}
*/

/// mpz_invert c = inverse of a mod b ((c*a) mod b == 1)
pub fn mpz_invert(c: mpz_t, a: mpz_t, b: mpz_t) -> int_t {
  unsafe { __gmpz_invert(c, a, b) }
}

/// mpz_jacobi 0 1 -1 (defined only for n odd)
pub fn mpz_jacobi(a: mpz_t, n: mpz_t) -> int_t {
  unsafe { __gmpz_jacobi(a, n) }
}

/// mpz_legendre 0 1 -1 (defined only for p an odd positive prime)
pub fn mpz_legendre(a: mpz_t, p: mpz_t) -> int_t {
  unsafe { __gmpz_legendre(a, p) }
}

/// mpz_kronecker
pub fn mpz_kronecker(a: mpz_t, n: mpz_t) -> int_t {
/*
  unsafe { __gmpz_kronecker(a, n) }
*/
  unsafe { __gmpz_jacobi(a, n) }
}

/// mpz_kronecker_ui
pub fn mpz_kronecker_ui(a: mpz_t, u: ui_t) -> int_t {
  unsafe { __gmpz_kronecker_ui(a, u) }
}

/// mpz_kronecker_si
pub fn mpz_kronecker_si(a: mpz_t, s: si_t) -> int_t {
  unsafe { __gmpz_kronecker_si(a, s) }
}

/// mpz_ui_kronecker
pub fn mpz_ui_kronecker(u: ui_t, a: mpz_t) -> int_t {
  unsafe { __gmpz_ui_kronecker(u, a) }
}

/// mpz_si_kronecker
pub fn mpz_si_kronecker(s: si_t, a: mpz_t) -> int_t {
  unsafe { __gmpz_si_kronecker(s, a) }
}

/// mpz_bin_ui nCk
pub fn mpz_bin_ui(c: mpz_t, n: mpz_t, k: ui_t) -> () {
  unsafe { __gmpz_bin_ui(c, n, k) }
}

/// mpz_bin_uiui nCk
pub fn mpz_bin_uiui(c: mpz_t, n: ui_t, k: ui_t) -> () {
  unsafe { __gmpz_bin_uiui(c, n, k) }
}

/// mpz_abs
pub fn mpz_abs(c: mpz_t, a: mpz_t) -> () {
  unsafe { __gmpz_abs(c, a) }
}

/// mpz_neg
pub fn mpz_neg(c: mpz_t, a: mpz_t) -> () {
  unsafe { __gmpz_neg(c, a) }
}

/// mpz_sub c = a - b
pub fn mpz_sub(c: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_sub(c, a, b) }
}

/// mpz_sub_ui c = a - u
pub fn mpz_sub_ui(c: mpz_t, a: mpz_t, u: ui_t) -> () {
  unsafe { __gmpz_sub_ui(c, a, u) }
}

/// mpz_ui_sub c = u - a
pub fn mpz_ui_sub(c: mpz_t, u: ui_t, a: mpz_t) -> () {
  unsafe { __gmpz_ui_sub(c, u, a) }
}

/// mpz_submul c -= a * b
pub fn mpz_submul(c: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_submul(c, a, b) }
}

/// mpz_submul_ui c -= a * u
pub fn mpz_submul_ui(c: mpz_t, a: mpz_t, u: ui_t) -> () {
  unsafe { __gmpz_submul_ui(c, a, u) }
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

/// mpz_cdiv_q
pub fn mpz_cdiv_q(q: mpz_t, n: mpz_t, d: mpz_t) -> () {
  unsafe { __gmpz_cdiv_q(q, n, d) }
}

/// mpz_cdiv_r
pub fn mpz_cdiv_r(r: mpz_t, n: mpz_t, d: mpz_t) -> () {
  unsafe { __gmpz_cdiv_r(r, n, d) }
}

/// mpz_cdiv_qr
pub fn mpz_cdiv_qr(q: mpz_t, r: mpz_t, n: mpz_t, d: mpz_t) -> () {
  unsafe { __gmpz_cdiv_qr(q, r, n, d) }
}

/// mpz_cdiv_q_ui
pub fn mpz_cdiv_q_ui(q: mpz_t, n: mpz_t, d: ui_t) -> ui_t {
  unsafe { __gmpz_cdiv_q_ui(q, n, d) }
}

/// mpz_cdiv_r_ui
pub fn mpz_cdiv_r_ui(r: mpz_t, n: mpz_t, d: ui_t) -> ui_t {
  unsafe { __gmpz_cdiv_r_ui(r, n, d) }
}

/// mpz_cdiv_qr_ui
pub fn mpz_cdiv_qr_ui(q: mpz_t, r: mpz_t, n: mpz_t, d: ui_t) -> ui_t {
  unsafe { __gmpz_cdiv_qr_ui(q, r, n, d) }
}

/// mpz_cdiv_ui
pub fn mpz_cdiv_ui(n: mpz_t, d: ui_t) -> ui_t {
  unsafe { __gmpz_cdiv_ui(n, d) }
}

/// mpz_cdiv_q_2exp
pub fn mpz_cdiv_q_2exp(q: mpz_t, n: mpz_t, b: mp_bitcnt_t) -> () {
  unsafe { __gmpz_cdiv_q_2exp(q, n, b) }
}

/// mpz_cdiv_r_2exp
pub fn mpz_cdiv_r_2exp(r: mpz_t, n: mpz_t, b: mp_bitcnt_t) -> () {
  unsafe { __gmpz_cdiv_r_2exp(r, n, b) }
}

/// mpz_fdiv_q
pub fn mpz_fdiv_q(q: mpz_t, n: mpz_t, d: mpz_t) -> () {
  unsafe { __gmpz_fdiv_q(q, n, d) }
}

/// mpz_fdiv_r
pub fn mpz_fdiv_r(r: mpz_t, n: mpz_t, d: mpz_t) -> () {
  unsafe { __gmpz_fdiv_r(r, n, d) }
}

/// mpz_fdiv_qr
pub fn mpz_fdiv_qr(q: mpz_t, r: mpz_t, n: mpz_t, d: mpz_t) -> () {
  unsafe { __gmpz_fdiv_qr(q, r, n, d) }
}

/// mpz_fdiv_q_ui
pub fn mpz_fdiv_q_ui(q: mpz_t, n: mpz_t, d: ui_t) -> ui_t {
  unsafe { __gmpz_fdiv_q_ui(q, n, d) }
}

/// mpz_fdiv_r_ui
pub fn mpz_fdiv_r_ui(r: mpz_t, n: mpz_t, d: ui_t) -> ui_t {
  unsafe { __gmpz_fdiv_r_ui(r, n, d) }
}

/// mpz_fdiv_qr_ui
pub fn mpz_fdiv_qr_ui(q: mpz_t, r: mpz_t, n: mpz_t, d: ui_t) -> ui_t {
  unsafe { __gmpz_fdiv_qr_ui(q, r, n, d) }
}

/// mpz_fdiv_ui
pub fn mpz_fdiv_ui(n: mpz_t, d: ui_t) -> ui_t {
  unsafe { __gmpz_fdiv_ui(n, d) }
}

/// mpz_fdiv_q_2exp
pub fn mpz_fdiv_q_2exp(q: mpz_t, n: mpz_t, b: mp_bitcnt_t) -> () {
  unsafe { __gmpz_fdiv_q_2exp(q, n, b) }
}

/// mpz_fdiv_r_2exp
pub fn mpz_fdiv_r_2exp(r: mpz_t, n: mpz_t, b: mp_bitcnt_t) -> () {
  unsafe { __gmpz_fdiv_r_2exp(r, n, b) }
}

/// mpz_tdiv_q
pub fn mpz_tdiv_q(q: mpz_t, n: mpz_t, d: mpz_t) -> () {
  unsafe { __gmpz_tdiv_q(q, n, d) }
}

/// mpz_tdiv_r
pub fn mpz_tdiv_r(r: mpz_t, n: mpz_t, d: mpz_t) -> () {
  unsafe { __gmpz_tdiv_r(r, n, d) }
}

/// mpz_tdiv_qr
pub fn mpz_tdiv_qr(q: mpz_t, r: mpz_t, n: mpz_t, d: mpz_t) -> () {
  unsafe { __gmpz_tdiv_qr(q, r, n, d) }
}

/// mpz_tdiv_q_ui
pub fn mpz_tdiv_q_ui(q: mpz_t, n: mpz_t, d: ui_t) -> ui_t {
  unsafe { __gmpz_tdiv_q_ui(q, n, d) }
}

/// mpz_tdiv_r_ui
pub fn mpz_tdiv_r_ui(r: mpz_t, n: mpz_t, d: ui_t) -> ui_t {
  unsafe { __gmpz_tdiv_r_ui(r, n, d) }
}

/// mpz_tdiv_qr_ui
pub fn mpz_tdiv_qr_ui(q: mpz_t, r: mpz_t, n: mpz_t, d: ui_t) -> ui_t {
  unsafe { __gmpz_tdiv_qr_ui(q, r, n, d) }
}

/// mpz_tdiv_ui
pub fn mpz_tdiv_ui(n: mpz_t, d: ui_t) -> ui_t {
  unsafe { __gmpz_tdiv_ui(n, d) }
}

/// mpz_tdiv_q_2exp
pub fn mpz_tdiv_q_2exp(q: mpz_t, n: mpz_t, b: mp_bitcnt_t) -> () {
  unsafe { __gmpz_tdiv_q_2exp(q, n, b) }
}

/// mpz_tdiv_r_2exp
pub fn mpz_tdiv_r_2exp(r: mpz_t, n: mpz_t, b: mp_bitcnt_t) -> () {
  unsafe { __gmpz_tdiv_r_2exp(r, n, b) }
}

/// mpz_mod
pub fn mpz_mod(r: mpz_t, n: mpz_t, d: mpz_t) -> () {
  unsafe { __gmpz_mod(r, n, d) }
}

/// mpz_mod_ui
pub fn mpz_mod_ui(r: mpz_t, n: mpz_t, d: ui_t) -> ui_t {
//  unsafe { __gmpz_mod_ui(r, n, d) }
  unsafe { __gmpz_fdiv_r_ui(r, n, d) }
}

/// mpz_divexact
pub fn mpz_divexact(q: mpz_t, n: mpz_t, d: mpz_t) -> () {
  unsafe { __gmpz_divexact(q, n, d) }
}

/// mpz_divexact_ui
pub fn mpz_divexact_ui(q: mpz_t, n: mpz_t, d: ui_t) -> () {
  unsafe { __gmpz_divexact_ui(q, n, d) }
}

/// mpz_divisible_p
pub fn mpz_divisible_p(n: mpz_t, d: mpz_t) -> bool {
  unsafe { __gmpz_divisible_p(n, d) != 0 }
}

/// mpz_divisible_ui_p
pub fn mpz_divisible_ui_p(n: mpz_t, d: ui_t) -> bool {
  unsafe { __gmpz_divisible_ui_p(n, d) != 0 }
}

/// mpz_divisible_2exp_p
pub fn mpz_divisible_2exp_p(n: mpz_t, b: mp_bitcnt_t) -> bool {
  unsafe { __gmpz_divisible_2exp_p(n, b) != 0 }
}

/// mpz_congruent_p
pub fn mpz_congruent_p(n: mpz_t, c: mpz_t, d: mpz_t) -> bool {
  unsafe { __gmpz_congruent_p(n, c, d) != 0 }
}

/// mpz_congruent_ui_p
pub fn mpz_congruent_ui_p(n: mpz_t, c: ui_t, d: ui_t) -> bool {
  unsafe { __gmpz_congruent_ui_p(n, c, d) != 0 }
}

/// mpz_congruent_2exp_p
pub fn mpz_congruent_2exp_p(n: mpz_t, c: mpz_t, b: mp_bitcnt_t) -> bool {
  unsafe { __gmpz_congruent_2exp_p(n, c, b) != 0 }
}

/// mpz_powwm_sec c = (a**n) mod m ***required n &gt; 0 and m is odd***
pub fn mpz_powm_sec(c: mpz_t, a: mpz_t, n: mpz_t, m: mpz_t) -> () {
  unsafe { __gmpz_powm_sec(c, a, n, m) }
}

/// mpz_powm c = (a**n) mod m ***n &lt; 0 when exists inv a**-1 mod m***
pub fn mpz_powm(c: mpz_t, a: mpz_t, n: mpz_t, m: mpz_t) -> () {
  unsafe { __gmpz_powm(c, a, n, m) }
}

/// mpz_powm_ui c = (a**n) mod m
pub fn mpz_powm_ui(c: mpz_t, a: mpz_t, n: ui_t, m: mpz_t) -> () {
  unsafe { __gmpz_powm_ui(c, a, n, m) }
}

/// mpz_pow_ui c == a**n
pub fn mpz_pow_ui(c: mpz_t, a: mpz_t, n: ui_t) -> () {
  unsafe { __gmpz_pow_ui(c, a, n) }
}

/// mpz_ui_pow_ui c = a**n
pub fn mpz_ui_pow_ui(c: mpz_t, a: ui_t, n: ui_t) -> () {
  unsafe { __gmpz_ui_pow_ui(c, a, n) }
}

/// mpz_sizeinbase
pub fn mpz_sizeinbase(a: mpz_t, base: int_t) -> mp_size_t {
  unsafe { __gmpz_sizeinbase(a, base) }
}

/// mpz_even_p
pub fn mpz_even_p(a: mpz_t) -> bool {
/*
  unsafe { __gmpz_even_p(a) != 0 }
*/
  !mpz_odd_p(a)
}

/// mpz_odd_p
pub fn mpz_odd_p(a: mpz_t) -> bool {
/*
  unsafe { __gmpz_odd_p(a) != 0 }
*/
unsafe {
  a._mp_size != 0 && (1 & std::slice::from_raw_parts(a._mp_d, 1)[0]) != 0
}
}

/// mpz_fits_ulong_p
pub fn mpz_fits_ulong_p(a: mpz_t) -> bool {
  unsafe { __gmpz_fits_ulong_p(a) != 0 }
}

/// mpz_fits_slong_p
pub fn mpz_fits_slong_p(a: mpz_t) -> bool {
  unsafe { __gmpz_fits_slong_p(a) != 0 }
}

/// mpz_fits_uint_p
pub fn mpz_fits_uint_p(a: mpz_t) -> bool {
  unsafe { __gmpz_fits_uint_p(a) != 0 }
}

/// mpz_fits_sint_p
pub fn mpz_fits_sint_p(a: mpz_t) -> bool {
  unsafe { __gmpz_fits_sint_p(a) != 0 }
}

/// mpz_fits_ushort_p
pub fn mpz_fits_ushort_p(a: mpz_t) -> bool {
  unsafe { __gmpz_fits_ushort_p(a) != 0 }
}

/// mpz_fits_sshort_p
pub fn mpz_fits_sshort_p(a: mpz_t) -> bool {
  unsafe { __gmpz_fits_sshort_p(a) != 0 }
}

/// mpz_urandomb
pub fn mpz_urandomb(c: mpz_t, r: randstate_t, nbits: mp_bitcnt_t) -> () {
  unsafe { __gmpz_urandomb(c, r, nbits) }
}

/// mpz_urandomm
pub fn mpz_urandomm(c: mpz_t, r: randstate_t, n: mpz_t) -> () {
  unsafe { __gmpz_urandomm(c, r, n) }
}

/// mpz_rrandomb
pub fn mpz_rrandomb(c: mpz_t, r: randstate_t, nbits: mp_bitcnt_t) -> () {
  unsafe { __gmpz_rrandomb(c, r, nbits) }
}

/// mpz_random ***(obsoleted) use mpz_urandomb or mpz_urandomm instead***
pub fn mpz_random(c: mpz_t, max_size: mp_size_t) -> () {
  unsafe { __gmpz_random(c, max_size) }
}

/// mpz_random2
pub fn mpz_random2(c: mpz_t, max_size: mp_size_t) -> () {
  unsafe { __gmpz_random2(c, max_size) }
}

/// mpz_and
pub fn mpz_and(c: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_and(c, a, b) }
}

/// mpz_ior
pub fn mpz_ior(c: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_ior(c, a, b) }
}

/// mpz_xor
pub fn mpz_xor(c: mpz_t, a: mpz_t, b: mpz_t) -> () {
  unsafe { __gmpz_xor(c, a, b) }
}

/// mpz_com
pub fn mpz_com(c: mpz_t, a: mpz_t) -> () {
  unsafe { __gmpz_com(c, a) }
}

/// mpz_popcount
pub fn mpz_popcount(a: mpz_t) -> mp_bitcnt_t {
  unsafe { __gmpz_popcount(a) }
}

/// mpz_hamdist hamming distance between a and b (both sgn must be same)
pub fn mpz_hamdist(a: mpz_t, b: mpz_t) -> mp_bitcnt_t {
  unsafe { __gmpz_hamdist(a, b) }
}

/// mpz_scan0 to msb
pub fn mpz_scan0(a: mpz_t, s: mp_bitcnt_t) -> mp_bitcnt_t {
  unsafe { __gmpz_scan0(a, s) }
}

/// mpz_scan1 to msb
pub fn mpz_scan1(a: mpz_t, s: mp_bitcnt_t) -> mp_bitcnt_t {
  unsafe { __gmpz_scan1(a, s) }
}

/// mpz_clrbit
pub fn mpz_clrbit(c: mpz_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpz_clrbit(c, n) }
}

/// mpz_setbit
pub fn mpz_setbit(c: mpz_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpz_setbit(c, n) }
}

/// mpz_combit
pub fn mpz_combit(c: mpz_t, n: mp_bitcnt_t) -> () {
  unsafe { __gmpz_combit(c, n) }
}

/// mpz_tstbit
pub fn mpz_tstbit(a: mpz_t, n: mp_bitcnt_t) -> bool {
  unsafe { __gmpz_tstbit(a, n) != 0 }
}
