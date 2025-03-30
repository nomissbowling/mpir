//! mpq ops test
//!

use crate::*;

/// ops test
pub fn ops_test() {
  // cmp mpq_t
  let a = &mut mpq_s::init();
  a.set_si(-2, 4);
  let b = &mut mpq_s::init();
  b.set_si(-4, 8);
  assert_eq!(a < b, false);
  assert_eq!(a == b, true);
  assert_eq!(a > b, false);

  // sub mpq_s
  let mut a = mpq_s::init();
  a.set_si(-2, 4);
  let mut b = mpq_s::init();
  b.set_si(-1, 8);
  let c = a - b;
  assert_eq!(format!("{}", c), "-3/8");

  // sub mpq_t
  let a = &mut mpq_s::init();
  a.set_si(-2, 4);
  let b = &mut mpq_s::init();
  b.set_si(-1, 8);
  let c = a - b;
  assert_eq!(format!("{}", c), "-3/8");

  // sub assign mpq_s
  let mut a = mpq_s::init();
  a.set_si(-2, 4);
  let mut b = mpq_s::init();
  b.set_si(-1, 8);
  a -= b;
  assert_eq!(format!("{}", a), "-3/8");

  // sub assign mpq_t (now same as mpq_s with *a)
  let a = &mut mpq_s::init();
  a.set_si(-2, 4);
  let b = &mut mpq_s::init();
  b.set_si(-1, 8);
  *a -= b;
  assert_eq!(format!("{}", a), "-3/8");

  // add mpq_s
  let mut a = mpq_s::init();
  a.set_si(-2, 4);
  let mut b = mpq_s::init();
  b.set_si(-1, 8);
  let c = a + b;
  assert_eq!(format!("{}", c), "-5/8");

  // mul mpq_s
  let mut a = mpq_s::init();
  a.set_si(-2, 3);
  let mut b = mpq_s::init();
  b.set_si(-1, 8);
  let c = a * b;
  assert_eq!(c, *mpq_s::init().set_si(1, 12));

  // div mpq_s
  let mut a = mpq_s::init();
  a.set_si(-2, 3);
  let mut b = mpq_s::init();
  b.set_si(-8, 9);
  let c = a / b;
  assert_eq!(c, *mpq_s::init().set_si(3, 4));

  // neg mpq_s
  let mut a = mpq_s::init();
  a.set_si(-2, 5);
  let c = -a;
  assert_eq!(format!("{}", c), "2/5");

  // neg mpq_r
  let a = &mut mpq_s::init();
  a.set_ui(1, 2);
  let c = -a;
  assert_eq!(format!("{}", c), "-1/2");
}
