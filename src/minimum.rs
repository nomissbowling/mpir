//! minimum test for mpir
//!

use crate::*;

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

  // mpq (c style)
  let q = &mut mpq_s::new();
  mpq_init(q);
  mpq_set_ui(q, 2, 8);
//assert_eq!(gmp_printf("[%#40Qx]\n", q), ()); // [ ... 0x2/0x8]
  assert_eq!(mpq_get_str(None, 10, q).expect("q"), "2/8");
  assert_eq!(format!("{}", q), "2/8");
}
