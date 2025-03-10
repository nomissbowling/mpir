//! minimum test for mpir
//!

use std::error::Error;
use std::collections::HashMap;

use crate::*;

/// calc napier
pub fn calc_napier(x: mpf_t, d: ui_t) -> Result<mpf_s, Box<dyn Error>> {
  let mut e = mpf_s::init_set_ui(0);
//  e.set_str("2.71828182845904523536", 10); // when d = 21
  let g = &mut mpf_s::init_set_ui(0);
  let m = &mut HashMap::<ui_t, mpz_s>::new();
  (0..=d).into_iter().for_each(|i| {
    let n = &mut mpz_s::fact_cached(i, m);
    let f = &mut mpf_s::pow_ui(x, i);
    e.add(f.div(g.set_z(n)));
//    println!("i {} g {} f {} e {}", i, g, f, e);
  });
  Ok(e)
}

/// simple test
pub fn simple_test() {
  // mpz (c style)
  let a = &mut mpz_s::new();
  mpz_init_set_si(a, -123);
//assert_eq!(gmp_printf("[%Zd]\n", a), ()); // -123
  assert_eq!(mpz_get_str(None, 10, a).expect("z"), "-123");
  assert_eq!(format!("{}", a), "-123");

  // mpz (to be operator)
  let b = &mut mpz_s::init_set_ui(654);
  assert_eq!(format!("{}", a.add(b).add(b)), "1185"); // a + b + b
  let c = &mut mpz_s::init_set_si(-1);
  assert_eq!(format!("{}", a.addmul(c, b).addmul(b, c)), "-123"); // a - b - b
  assert_eq!(format!("{}", a.mul(c.set_si(-2))), "246"); // -123 * -2
  assert_eq!(format!("{}", a.mul_si(-1)), "-246"); // 246 * -1
  assert_eq!(format!("{}", a.mul_2exp(20)), "-257949696"); // -246 * 2**20

  // mpf (c style)
  let f = &mut mpf_s::new();
  mpf_init_set_d(f, -0.3);
//assert_eq!(gmp_printf_1f("[%.*Ff]\n", 17, f), ()); // -0.29999999999999999
  assert_eq!(mpf_get_fmtstr(f, 10, 17).expect("f"), "-0.29999999999999999e+0");
//assert_eq!(gmp_printf_1f("[%.*Ff]\n", 20, f), ()); // -0.29999999999999998890
  assert_eq!(mpf_get_fmtstr(f, 10, 20).expect("f"), "-0.2999999999999999889e+0");
  assert_eq!(format!("{}", f), "-0.2999999999999999889e+0"); // cut off last 0

  mpf_set_d(f, -30.0);
  assert_eq!(format!("{}", f), "-0.3e+2");

  mpf_set_d(f, -33.0);
  assert_eq!(format!("{}", f), "-0.33e+2");

  mpf_set_d(f, -33.3); // f64 (double) precision about 16 significant digits
  assert_eq!(format!("{}", f), "-0.33299999999999997158e+2"); // 20 digits

  mpf_set_d(f, 999.0); // multiple-precision 999.0 / -30.0 = -0.333e+2
  let e = &mut mpf_s::new();
  mpf_init_set_d(e, -30.0);
  let g = &mut mpf_s::new();
  mpf_init_set_d(g, 0.0);
  mpf_div(g, f, e);
  assert_eq!(format!("{}", g), "-0.333e+2");

  mpf_set_d(f, -1.0); // -1.0 / 3.0 = -0.33333333333333333333e+0
  mpf_set_d(e, 3.0);
  mpf_set_d(g, 0.0);
  mpf_div(g, f, e);
  assert_eq!(format!("{}", g), "-0.33333333333333333333e+0");

  mpf_set_d(e, 3.0); // 2 / 3.0 = 0.66666666666666666667e+0
  mpf_ui_div(g, 2, e);
  assert_eq!(format!("{}", g), "0.66666666666666666667e+0");

  mpf_set_d(f, 2.0); // 2.0 / 3 = 0.66666666666666666667e+0
  mpf_div_ui(g, f, 3);
  assert_eq!(format!("{}", g), "0.66666666666666666667e+0");

  mpf_set_d(g, 24.0); // 24.0 / 10.0 = 0.24e+1 (prepare f = g / e)
  mpf_set_d(e, 10.0);
  mpf_div(f, g, e); // not use mpf_set_d(f, 2.4) to avoid double precision 2.4
  assert_eq!(format!("{}", f), "0.24e+1");
  mpf_div_2exp(g, f, 3); // 0.24e+1 / 2**3 = 0.3e+0
  assert_eq!(format!("{}", g), "0.3e+0");

  mpf_set_str(f, "-4.8", 10); // not use mpf_set_d(f, -4.8)
  assert_eq!(format!("{}", f), "-0.48e+1");
  mpf_div_2exp(g, f, 3); // -0.48e+1 / 2**3 = -0.59999999999999999999e+0
  assert_eq!(format!("{}", g), "-0.59999999999999999999e+0");

  mpf_set_d(f, 5.0); // sqrt(5.0) = 0.22360679774997896964e+1
  mpf_sqrt(g, f);
  assert_eq!(format!("{}", g), "0.22360679774997896964e+1");

  // mpf (to be operator)
  assert_eq!(format!("{}", a.set_ui(1).mul_2exp(100)), // 1 * 2**100
    "1267650600228229401496703205376");
  assert_eq!(format!("{}", f.set_z(a).div_2exp(100)), // 2**100 / 2**100
    "0.1e+1");

  // mpz and mpf (prepare and reset)
  let a = &mut mpz_s::init();
  let f = &mut mpf_s::init();
  let g = &mut mpf_s::init();
/*
  // ***must NOT call*** auto called clear
  a.clear(); mpz_init(a);
  f.clear(); mpf_init(f);
  g.clear(); mpf_init(g);
*/
/*
  // ***must NOT call*** auto called clear
  mpz_clears(&mut vec![a]); mpz_init(a);
  mpf_clears(&mut vec![g, f]); mpf_init(f); mpf_init(g);
*/

  // mpz and mpf (to be operator) check about significant digits
  assert_eq!(format!("{}", a.set_str("987654321098765432109", 10)),
    "987654321098765432109"); // 21 digits
  assert_eq!(format!("{}", f.set_z(a).div(g.set_str("1.0e+11", 10))), // drift
    "0.98765432109876543211e+10"); // 20 digits by default formatter
  assert_eq!(mpf_get_fmtstr(f, 10, 22).expect("fmtstr"), // check to 22 digits
    "0.987654321098765432109e+10"); // 21 digits ok

  // mpf (to be operator)
  assert_eq!(format!("{}", f.set_ui(3).ui_div(1)),
    "0.33333333333333333333e+0"); // 1 / 3
  assert_eq!(format!("{}", g.set_ui(1).div_ui(3)),
    "0.33333333333333333333e+0"); // 1 / 3
  assert_eq!(format!("{}", f.set_ui(3).ui_div(1)),
    format!("{}", g.set_ui(1).div_ui(3))); // 1 / 3
  assert_eq!(format!("{}", f.set_ui(3).ui_div(2)),
    "0.66666666666666666667e+0"); // 2 / 3
  assert_eq!(format!("{}", g.set_ui(2).div_ui(3)),
    "0.66666666666666666667e+0"); // 2 / 3
  assert_eq!(format!("{}", f.set_ui(3).ui_div(2)),
    format!("{}", g.set_ui(2).div_ui(3))); // 2 / 3

  // mpz fact (to be operator)
  let facts = vec![
    "1", "1", "2", "6", "24", "120", "720", "5040", "40320", "362880", // 0-9
    "3628800", "39916800", "479001600", "6227020800", "87178291200", // 10-14
    "1307674368000", "20922789888000", "355687428096000", // 15-17
    "6402373705728000", "121645100408832000", "2432902008176640000"]; // 18-20
  (0..=20).into_iter().for_each(|n: usize| {
    let t = &mut mpz_s::fact(n as ui_t);
    assert_eq!(format!("{}! = {}", n, t), format!("{}! = {}", n, facts[n]));
    let u = &mut mpz_s::fac_ui(n as ui_t);
    assert_eq!(format!("{}! = {}", n, t), format!("{}! = {}", n, u));
  });

  // mpz fact (to be operator) cached
  let m = &mut HashMap::<ui_t, mpz_s>::new();
  (0..=20).into_iter().for_each(|n: usize| {
    let t = &mut mpz_s::fact_cached(n as ui_t, m);
    assert_eq!(format!("{}! = {}", n, t), format!("{}! = {}", n, facts[n]));
  });

  // mpq (c style)
  let q = &mut mpq_s::new();
  mpq_init(q);
  mpq_set_ui(q, 2, 8);
//assert_eq!(gmp_printf("[%#40Qx]\n", q), ()); // [ ... 0x2/0x8]
  assert_eq!(mpq_get_str(None, 10, q).expect("q"), "2/8");
  assert_eq!(format!("{}", q), "2/8");

  // mpq (to be operator)
  let q = &mut mpq_s::init();
  assert_eq!(format!("{}", q.set_ui(2, 8)), "2/8");

  // mpz (to be operator)
  assert!(a.set_si(0).sgn() == 0);
  assert!(a.set_si(1).sgn() > 0);
  assert!(a.set_si(-1).sgn() < 0);
  assert!(a.cmp(b.set_si(-1)) == 0);
  assert!(a.cmp_d(-10.0) > 0);
  assert!(a.cmp_ui(10) < 0);
  assert!(a.cmp_si(-10) > 0);
  assert!(a.cmpabs(b.set_si(-10)) < 0);
  assert!(a.cmpabs_d(-10.0) < 0);
  assert!(a.cmpabs_ui(10) < 0);

  // mpf (to be operator)
  assert!(f.set_si(0).sgn() == 0);
  assert!(f.set_si(1).sgn() > 0);
  assert!(f.set_si(-1).sgn() < 0);
  assert!(f.cmp(g.set_si(-10)) > 0);
  assert!(f.cmp_d(-10.0) > 0);
  assert!(f.cmp_ui(1) < 0);
  assert!(f.cmp_si(-10) > 0);
  assert!(f.cmp_z(a) == 0);
  assert!(f.cmp_z(a.set_si(-20)) > 0);
  assert!(f.cmp_z(a.set_ui(20)) < 0);

  // mpq (to be operator)
  assert!(q.set_si(0, 1).sgn() == 0);
  assert!(q.set_si(1, 1).sgn() > 0);
  assert!(q.set_si(-1, 1).sgn() < 0);

  // mpf prec (c style)
//  assert_eq!(mpf_get_default_prec(), 64); // may be 64
  mpf_set_default_prec(100); // 100 set to 128 bits (step by 2**n)
//  assert_eq!(mpf_get_default_prec(), 128); // may be 128 (about 40 digits)

  // mpf significant digits (to be operator)
  let f = &mut mpf_s::init_set_str("1.0e-19", 10);
  let e = &mut mpf_s::init_set_str("1.0e-50", 10);
  assert_eq!(mpf_get_fmtstr(e, 10, 60).expect("fmtstr"), "0.1e-49");
  assert_eq!(mpf_get_fmtstr(f, 10, 60).expect("fmtstr"), "0.1e-18");
  // f.add(e) as 0.99999999999999999999e-19 without mpf_set_default_prec(100)
  assert_eq!(mpf_get_fmtstr(f.add(e), 10, 60).expect("fmtstr"),
    "0.1000000000000000000000000000000099999999e-18");

  // mpf calc napier (to be operator)
  let d: ui_t = 21; // expect 21 digits 2.718281828459045235360287471352 ...
  let e = &mut calc_napier(&mut mpf_s::init_set_d(1.0), d).expect("exp");
  assert_eq!(format!("{}", e),
    "0.27182818284590452354e+1");
  assert_eq!(mpf_get_fmtstr(e, 10, 21).expect("fmtstr"),
    "0.271828182845904523536e+1");

  let d: ui_t = 26; // overwrite recalc
  let e = &mut calc_napier(&mut mpf_s::init_set_d(1.0), d).expect("exp");
  assert_eq!(format!("{}", e),
    "0.27182818284590452354e+1");
  assert_eq!(mpf_get_fmtstr(e, 10, 26).expect("fmtstr"),
    "0.27182818284590452353602875e+1");
}
