#![doc(html_root_url = "https://docs.rs/mpir/0.1.2")]
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
  use super::minimum::simple_test;

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
  fn test_mpf() {
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
  fn test_mpir() {
    assert_eq!(simple_test(), ());
  }
}
