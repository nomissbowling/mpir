//! mpf ops test
//!

use crate::*;

/// ops test
/// expected on the single thread for mpf_set_default_prec
pub fn ops_test() {
  mpf_set_default_prec(64); // 64 bits default

  // cmp mpf_s
  let a = mpf_s::init_set_d(1.0);
  let b = mpf_s::init_set_d(2.0);
  assert_eq!(a < b, true);
  assert_eq!(a == b, false);
  assert_eq!(a > b, false);

  // sub mpf_s
  let a = mpf_s::init_set_ui(1);
  let b = mpf_s::init_set_ui(2);
  let c = a - b;
  assert_eq!(format!("{}", c), "-0.1e+1");

  // sub mpf_t
  let a = &mpf_s::init_set_ui(1);
  let c = a - 2 as ui_t;
  assert_eq!(format!("{}", c), "-0.1e+1");

  // sub ui_t
  let a = &mpf_s::init_set_ui(2);
  let c = 1 as ui_t - a;
  assert_eq!(format!("{}", c), "-0.1e+1");

  // sub assign mpf_t (now same as mpf_s with *a)
  let a = &mut mpf_s::init_set_si(1);
  let b = &mpf_s::init_set_si(2);
  *a -= b;
  assert_eq!(format!("{}", a), "-0.1e+1");

  // sub assign mpf_t (now same as mpf_s with *a)
  let a = &mut mpf_s::init_set_si(1);
  *a -= 2 as ui_t;
  assert_eq!(format!("{}", a), "-0.1e+1");

  // add mpf_s
  let a = mpf_s::init_set_ui(1);
  let b = mpf_s::init_set_si(-2);
  let c = a + b;
  assert_eq!(format!("{}", c), "-0.1e+1");

  // mul mpf_s
  let a = mpf_s::init_set_ui(1);
  let b = mpf_s::init_set_si(-2);
  let c = a * b;
  assert_eq!(format!("{}", c), "-0.2e+1");

  // div mpf_s
  let a = mpf_s::init_set_ui(1);
  let b = mpf_s::init_set_si(-2);
  let c = a / b;
  assert_eq!(format!("{}", c), "-0.5e+0");

  // div mpf_s
  let a = mpf_s::init_set_si(-1);
  let c = a / 2 as ui_t;
  assert_eq!(format!("{}", c), "-0.5e+0");

  // div ui_t
  let a = mpf_s::init_set_si(-2);
  let c = 1 as ui_t / a;
  assert_eq!(format!("{}", c), "-0.5e+0");

  // neg mpf_s
  let a = mpf_s::init_set_d(-2.5);
  let c = -a;
  assert_eq!(format!("{}", c), "0.25e+1");

  // neg mpf_r
  let a = &mpf_s::init_set_d(1.25);
  let c = -a;
  assert_eq!(format!("{}", c), "-0.125e+1");
}
