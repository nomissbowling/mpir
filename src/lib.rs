#![doc(html_root_url = "https://docs.rs/mpir/0.3.5")]
//! partial Rust porting of mpir multiple precision library based on gmp mpfr
//!
//! Cargo test with [-- --nocapture] or with [-- --show-output]
//!
//! see also [sum_arctan_gregory() source](https://docs.rs/mpir/latest/mpir/prim/mpf/struct.__mpf_struct.html#method.sum_arctan_gregory)
//!
//! ```Rust
//! // inv_f
//! let a = &mpz_s::from(-3);
//! let mut f = a.inv_f();
//! assert_eq!(format!("{}", f), "-0.33333333333333333333e+0");
//! f *= 3;
//! assert_eq!(format!("{}", f), "-0.1e+1");
//!
//! // inv_f
//! let a = &mpz_s::from(-2);
//! let f = a.inv_f();
//! assert_eq!(format!("{}", f), "-0.5e+0");
//!
//! // inv_q
//! let a = &mpz_s::from(-2);
//! let q = &mut a.inv_q();
//! assert_eq!(format!("{}", q), "1/-2");
//! assert_eq!(format!("{}", q.reduce()), "1/-2");
//! assert_eq!(format!("{}", q.inv()), "-2");
//!
//! // mpf from mpq
//! let f = mpf_s::from(&*q);
//! assert_eq!(format!("{}", f), "-0.5e+0");
//! assert_eq!(format!("{}", f.inv()), "-0.2e+1");
//!
//! // mpq from &str
//! let f = &mut mpq_s::from("9/-24");
//! assert_eq!(format!("{}", f), "9/-24");
//! assert_eq!(format!("{}", f.reduce()), "3/-8");
//!
//! // mpf from &str
//! let f = mpf_s::from("-5");
//! assert_eq!(format!("{}", f), "-0.5e+1");
//!
//! // mpz from &str
//! let f = mpz_s::from("-5");
//! assert_eq!(format!("{}", f), "-5");
//! ```
//!
//! # Requirements
//!
//! - [ gmp ]( https://gmplib.org/ )
//! - [ mpir ]( https://github.com/ChillMagic/MPIR-Binary )
//!
//! in the running directory
//!
//! - libgmp-10.dll
//! - libgmpxx-4.dll (optional)
//! - mpir.dll
//!
//! see also
//!
//! - [ https://crates.io/crates/mpir ]( https://crates.io/crates/mpir )
//! - [ https://github.com/nomissbowling/mpir ]( https://github.com/nomissbowling/mpir )
//!

pub mod prim;
pub use crate::prim::{*, typ::*, mpz::*, mpf::*, mpq::*, randstate::*};
// pub use crate::prim::gmp::*;

pub mod ops;
pub use crate::ops::{*};

pub mod convert;
pub use crate::convert::{*};

pub mod util;
pub use crate::util::{*};

pub mod minimum;

#[cfg(test)]
mod tests {
  use super::*;
  use super::minimum::{
    calc_mpz_test,
    calc_fact_test,
    calc_fib_test,
    calc_gcd_test,
    calc_lcm_test,
    calc_mod_prime_test,
    calc_binomial_coefficient_test,
    calc_mpf_prec64_test, // single thread
    calc_rand_test, // single thread
    calc_fit_test, // single thread
    calc_logical_test,
    calc_mpq_test,
    ops, // ops_test for mpz mpf mpq complex (multi thread and single thread)
    compare_test, // single thread
    significant_digits_test, // single thread
    calc_pi_gauss_legendre_test, // single thread
    calc_pi_euler_test, // single thread
    calc_pi_leibniz_test, // single thread
    calc_pi_machin_test, // single thread
    calc_pi_takano_test, // single thread
    calc_napier_test, // single thread
    ept_test};
  use serial_test::serial;

  #[test]
  fn test_mpz() {
    let a = &mut mpz_s::new();
    mpz_init_set_si(a, -123);
    assert_eq!(format!("{:?}", a),
      "1, -1 000000000000007b");
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_mpf() {
    mpf_set_default_prec(64); // 64 bits default
    let f = &mut mpf_s::new();
    mpf_init_set_d(f, -0.3);
    assert_eq!(format!("{:?}", f),
      "2, -2, 0 0000000000000000 4ccccccccccccc00");
  }

  #[test]
  fn test_mpq() {
    let q = &mut mpq_s::new();
    mpq_init(q);
    mpq_set_ui(q, 2, 8);
    assert_eq!(format!("{:?}", q),
      "(1, 1 0000000000000002) / (1, 1 0000000000000008)");
  }

  #[test]
  fn test_calc_mpz() {
    assert_eq!(calc_mpz_test(), ());
  }

  #[test]
  fn test_calc_fact() {
    assert_eq!(calc_fact_test(), ());
  }

  #[test]
  fn test_calc_fib() {
    assert_eq!(calc_fib_test(), ());
  }

  #[test]
  fn test_calc_gcd() {
    assert_eq!(calc_gcd_test(), ());
  }

  #[test]
  fn test_calc_lcm() {
    assert_eq!(calc_lcm_test(), ());
  }

  #[test]
  fn test_calc_mod_prime() {
    assert_eq!(calc_mod_prime_test(), ());
  }

  #[test]
  fn test_calc_binomial_coefficient() {
    assert_eq!(calc_binomial_coefficient_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_calc_mpf_prec64() {
    assert_eq!(calc_mpf_prec64_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_calc_rand() {
    assert_eq!(calc_rand_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_calc_fit() {
    assert_eq!(calc_fit_test(), ());
  }

  #[test]
  fn test_calc_logical() {
    assert_eq!(calc_logical_test(), ());
  }

  #[test]
  fn test_calc_mpq() {
    assert_eq!(calc_mpq_test(), ());
  }

  #[test]
  fn test_ops_mpz() {
    assert_eq!(ops::mpz::ops_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_ops_mpf() {
    assert_eq!(ops::mpf::ops_test(), ());
  }

  #[test]
  fn test_ops_mpq() {
    assert_eq!(ops::mpq::ops_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_ops() {
    assert_eq!(ops::ops_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_compare() {
    assert_eq!(compare_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_significant_digits() {
    assert_eq!(significant_digits_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_calc_pi_gauss_legendre() {
    assert_eq!(calc_pi_gauss_legendre_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_calc_pi_euler() {
    assert_eq!(calc_pi_euler_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_calc_pi_leibniz() {
    assert_eq!(calc_pi_leibniz_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_calc_pi_machin() {
    assert_eq!(calc_pi_machin_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_calc_pi_takano() {
    assert_eq!(calc_pi_takano_test(), ());
  }

  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_calc_napier() {
    assert_eq!(calc_napier_test(), ());
  }

  #[test]
  fn test_ept() {
    assert_eq!(ept_test(), ());
  }
}
