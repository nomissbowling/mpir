#![doc(html_root_url = "https://docs.rs/mpir/0.2.0")]
//! partial Rust porting of mpir multiple precision library based on gmp mpfr
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
pub use crate::prim::{*, typ::*, mpz::*, mpf::*, mpq::*}; // gmp::*

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
    calc_mpf_prec64_test, // single thread
    calc_mpq_test,
    compare_test, // single thread
    significant_digits_test, // single thread
    calc_napier_test}; // single thread
  use serial_test::serial;

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn test_mpz() {
    let a = &mut mpz_s::new();
    mpz_init_set_si(a, -123);
    assert_eq!(format!("{:?}", a),
      "1, -1 000000000000007b");
  }

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_mpf() {
    mpf_set_default_prec(64); // 64 bits default
    let f = &mut mpf_s::new();
    mpf_init_set_d(f, -0.3);
    assert_eq!(format!("{:?}", f),
      "2, -2, 0 0000000000000000 4ccccccccccccc00");
  }

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn test_mpq() {
    let q = &mut mpq_s::new();
    mpq_init(q);
    mpq_set_ui(q, 2, 8);
    assert_eq!(format!("{:?}", q),
      "(1, 1 0000000000000002) / (1, 1 0000000000000008)");
  }

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn test_calc_mpz() {
    assert_eq!(calc_mpz_test(), ());
  }

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn test_calc_fact() {
    assert_eq!(calc_fact_test(), ());
  }

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn test_calc_fib() {
    assert_eq!(calc_fib_test(), ());
  }

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn test_calc_gcd() {
    assert_eq!(calc_gcd_test(), ());
  }

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn test_calc_lcm() {
    assert_eq!(calc_lcm_test(), ());
  }

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_calc_mpf_prec64() {
    assert_eq!(calc_mpf_prec64_test(), ());
  }

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn test_calc_mpq() {
    assert_eq!(calc_mpq_test(), ());
  }

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_compare() {
    assert_eq!(compare_test(), ());
  }

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_significant_digits() {
    assert_eq!(significant_digits_test(), ());
  }

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  #[serial] // expected on the single thread for mpf_set_default_prec
  fn test_calc_napier() {
    assert_eq!(calc_napier_test(), ());
  }
}
