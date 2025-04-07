//! mpq ops test
//!

use crate::*;

/// ops test
pub fn ops_test() {
  // cmp mpq_t
  let a = &mpq_s::from((-2 as si_t, 4));
  let b = &mpq_s::from((-4 as si_t, 8));
  assert_eq!(a < b, false);
  assert_eq!(a == b, true);
  assert_eq!(a > b, false);

  // sub mpq_s
  let a = mpq_s::from((-2 as si_t, 4));
  let b = mpq_s::from((-1 as si_t, 8));
  let c = a - b;
  assert_eq!(format!("{}", c), "-3/8");

  // sub mpq_t
  let a = &mpq_s::from((-2 as si_t, 4));
  let b = &mpq_s::from((-1 as si_t, 8));
  let c = a - b;
  assert_eq!(format!("{}", c), "-3/8");

  // sub assign mpq_s
  let mut a = mpq_s::from((-2 as si_t, 4));
  let b = mpq_s::from((-1 as si_t, 8));
  a -= b;
  assert_eq!(format!("{}", a), "-3/8");

  // sub assign mpq_t (now same as mpq_s with *a)
  let a = &mut mpq_s::from((-2 as si_t, 4));
  let b = &mpq_s::from((-1 as si_t, 8));
  *a -= b;
  assert_eq!(format!("{}", a), "-3/8");

  // add mpq_s
  let a = mpq_s::from((-2 as si_t, 4));
  let b = mpq_s::from((-1 as si_t, 8));
  let c = a + b;
  assert_eq!(format!("{}", c), "-5/8");

  // mul mpq_s
  let a = mpq_s::from((-2 as si_t, 3));
  let b = mpq_s::from((-1 as si_t, 8));
  let c = a * b;
  assert_eq!(c, mpq_s::from((1 as si_t, 12)));

  // div mpq_s
  let a = mpq_s::from((-2 as si_t, 3));
  let b = mpq_s::from((-8 as si_t, 9));
  let c = a / b;
  assert_eq!(c, mpq_s::from((3 as si_t, 4)));

  // neg mpq_s
  let a = mpq_s::from((-2 as si_t, 5));
  let c = -a;
  assert_eq!(format!("{}", c), "2/5");

  // neg mpq_r
  let a = &mpq_s::from((1, 2 as ui_t));
  let c = -a;
  assert_eq!(format!("{}", c), "-1/2");
}
