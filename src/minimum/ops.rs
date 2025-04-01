//! ops test
//!

pub mod mpz;
pub mod mpf;
pub mod mpq;

use crate::*;

/// ops test
/// expected on the single thread for mpf_set_default_prec
pub fn ops_test() {
  mpf_set_default_prec(64); // 64 bits default

  // inv_f
  let a = &mpz_s::from(-3);
  let mut f = a.inv_f();
  assert_eq!(format!("{}", f), "-0.33333333333333333333e+0");
  f *= 3;
  assert_eq!(format!("{}", f), "-0.1e+1");

  // inv_f
  let a = &mpz_s::from(-2);
  let f = a.inv_f();
  assert_eq!(format!("{}", f), "-0.5e+0");

  // inv_q
  let a = &mpz_s::from(-2);
  let q = &mut a.inv_q();
  assert_eq!(format!("{}", q), "1/-2");
  assert_eq!(format!("{}", q.reduce()), "1/-2");
  assert_eq!(format!("{}", q.inv()), "-2");

  // mpf from mpq
  let f = mpf_s::from(&*q);
  assert_eq!(format!("{}", f), "-0.5e+0");
  assert_eq!(format!("{}", f.inv()), "-0.2e+1");

  // mpq from &str
  let f = &mut mpq_s::from("9/-24");
  assert_eq!(format!("{}", f), "9/-24");
  assert_eq!(format!("{}", f.reduce()), "3/-8");

  // mpf from &str
  let f = mpf_s::from("-5");
  assert_eq!(format!("{}", f), "-0.5e+1");

  // mpz from &str
  let f = mpz_s::from("-5");
  assert_eq!(format!("{}", f), "-5");

  // mpz_r * mpf_r
  let a = &mpz_s::init_set_si(-3);
  let f = &mpf_s::init_set_ui(2);
  let c = a * f;
  assert_eq!(format!("{}", c), "-0.6e+1");

  // mpf_r * mpz_r
  let f = &mpf_s::init_set_si(-3);
  let a = &mpz_s::init_set_ui(2);
  let c = 2 as ui_t / f * a;
  assert_eq!(format!("{}", c), "-0.13333333333333333333e+1");

  // mpf_r *= mpz_r (same as mpf_s with *f)
  let f = &mut mpf_s::init_set_si(-3);
  let a = &mpz_s::init_set_ui(2);
  *f *= a;
  assert_eq!(format!("{}", f), "-0.6e+1");

  // mpz_r / mpf_r
  let a = &mpz_s::init_set_si(-3);
  let f = &mpf_s::init_set_ui(2);
  let c = a / f;
  assert_eq!(format!("{}", c), "-0.15e+1");

  // mpf_r / mpz_r
  let f = &mpf_s::init_set_si(-3);
  let a = &mpz_s::init_set_ui(2);
  let c = 2 as ui_t / f / a;
  assert_eq!(format!("{}", c), "-0.33333333333333333333e+0");

  // mpf_r /= mpz_r (same as mpf_s with *f)
  let f = &mut mpf_s::init_set_si(-3);
  let a = &mpz_s::init_set_ui(2);
  *f /= a;
  assert_eq!(format!("{}", f), "-0.15e+1");

  // mpq_r when den is not 1
  let a = &mpz_s::init_set_ui(16);
  let b = &mpz_s::init_set_ui(24);
  let q = &mut mpq_s::frac(a, b); // binding to borrow
  assert_eq!(format!("{}", q), "16/24");
  assert_eq!(format!("{}", q.reduce()), "2/3");

  // mpq_r when den is not 1
  let q = &mut mpq_s::frac(&mpz_s::init_set_ui(8), &mpz_s::init_set_ui(24));
  *q *= mpz_s::init_set_ui(2);
  assert_eq!(format!("{}", q), "8/12");
  assert_eq!(format!("{}", q.reduce()), "2/3");

  // mpq_r when den is not 1
  let q = &mut mpq_s::frac(&mpz_s::init_set_ui(16), &mpz_s::init_set_ui(4));
  *q *= mpz_s::init_set_ui(2);
  assert_eq!(format!("{}", q), "16/2");
  assert_eq!(format!("{}", q.reduce()), "8");

  // mpq_r when den is not 1
  let q = &mut mpq_s::frac(&mpz_s::init_set_ui(8), &mpz_s::init_set_ui(2));
  assert_eq!(format!("{}", q), "8/2");
  assert_eq!(format!("{}", q.reduce()), "4");

  // mpq_r when den is 1
  let q = &mut mpq_s::frac(&mpz_s::init_set_ui(4), &mpz_s::init_set_ui(1));
  assert_eq!(format!("{}", q), "4");

  // mpz_r * mpq_r
  let a = &mpz_s::init_set_si(-3);
  let q = &mut mpq_s::init();
  let c = a * q.set_si(-1, 6);
  assert_eq!(format!("{}", c), "1/2");

  // mpq_r * mpz_r
  let q = &mut mpq_s::init();
  let a = &mpz_s::init_set_si(10);
  let c = q.set_si(-2, 5) * a;
  assert_eq!(format!("{}", c), "-4");

  // mpq_r *= mpz_r
  let q = &mut mpq_s::init();
  let a = &mpz_s::init_set_si(-3);
  *q.set_si(-1, 6) *= a;
  assert_eq!(format!("{}", q), "1/2");

  // mpz_r / mpq_r
  let a = &mpz_s::init_set_si(-3);
  let q = &mut mpq_s::init();
  let c = a / q.set_si(-1, 6);
  assert_eq!(format!("{}", c), "18");

  // mpq_r / mpz_r
  let q = &mut mpq_s::init();
  let a = &mpz_s::init_set_si(10);
  let c = q.set_si(-2, 5) / a;
  assert_eq!(format!("{}", c), "-1/25");

  // mpq_r /= mpz_r
  let q = &mut mpq_s::init();
  let a = &mpz_s::init_set_si(-3);
  *q.set_si(-12, 2) /= a;
  assert_eq!(format!("{}", q), "4/2");
  assert_eq!(format!("{}", q.reduce()), "2");

  // mpq_r * mpf_r
  let a = &mpz_s::init_set_si(2);
  let b = &mpz_s::init_set_si(-3);
  let f = &mpf_s::init_set_ui(2);
  let c = &mpq_s::frac(a, b) * f;
  assert_eq!(format!("{}", c), "-0.13333333333333333333e+1");

  // mpf_r * mpq_r
  let f = &mpf_s::init_set_si(-3);
  let q = &mut mpq_s::init();
  let c = f * q.set_si(-1, 6);
  assert_eq!(format!("{}", c), "0.5e+0");

  // mpf_r *= mpq_r
  let f = &mut mpf_s::init_set_si(-3);
  let q = &mut mpq_s::init();
  *f *= q.set_si(-1, 6);
  assert_eq!(format!("{}", f), "0.5e+0");

  // mpq_r / mpf_r
  let a = &mpz_s::init_set_si(2);
  let b = &mpz_s::init_set_si(-3);
  let f = &mpf_s::init_set_ui(2);
  let c = &mpq_s::frac(a, b) / f;
  assert_eq!(format!("{}", c), "-0.33333333333333333333e+0");

  // mpf_r / mpq_r
  let f = &mpf_s::init_set_si(-3);
  let q = &mut mpq_s::init();
  let c = f / q.set_si(-1, 6);
  assert_eq!(format!("{}", c), "0.18e+2");

  // mpf_r /= mpq_r
  let f = &mut mpf_s::init_set_si(-3);
  let q = &mut mpq_s::init();
  *f /= q.set_si(-1, 6);
  assert_eq!(format!("{}", f), "0.18e+2");
}
