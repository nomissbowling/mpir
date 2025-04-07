//! mpz ops test
//!

use crate::*;

/// ops test
pub fn ops_test() {
  // cmp mpz_s
  let a = mpz_s::from(!0 as si_t);
  let b = mpz_s::from(!0 as si_t);
  assert_eq!(a == b, true);
  assert_eq!(a != !b, true);

  // cmp mpz_r
  let a = &mpz_s::from(!0 as si_t);
  let b = &mpz_s::from(!0 as si_t);
  assert_eq!(a == b, true);
  assert_eq!(!a != *b, true); // mpz_s mpz_s

  // cmp mpz_s
  let a = mpz_s::from(-3);
  let b = mpz_s::from(-3);
  assert_eq!(a < b, false);
  assert_eq!(a == b, true);
  assert_eq!(a > b, false);

  // cmp mpz_s
  let a = mpz_s::from(-3);
  let b = mpz_s::from(-5);
  assert_eq!(a < b, false);
  assert_eq!(a == b, false);
  assert_eq!(a > b, true);

  // cmp mpz_r
  let a = &mpz_s::from(3);
  let b = &mpz_s::from(5);
  assert_eq!(a < b, true);
  assert_eq!(a == b, false);
  assert_eq!(a > b, false);

  // cmp mpz_r
  let a = &mut mpz_s::from(3);
  let b = &mut mpz_s::from(5);
  *b *= -1 as si_t;
  assert_eq!(a < b, false);
  assert_eq!(a == b, false);
  assert_eq!(a > b, true);

  // sub mpz_s - mpz_s
  let a = mpz_s::from(-2);
  let b = mpz_s::from(5);
  let c = a - b;
  assert_eq!(format!("{}", c), "-7");

  // sub mpz_r - mpz_r
  let a = &mpz_s::from(-2);
  let b = &mpz_s::from(5);
  let c = a - b;
  assert_eq!(format!("{}", c), "-7");

  // sub mpz_s - ui_t
  let a = mpz_s::from(3);
  let b = 5 as ui_t;
  let c = a - b;
  assert_eq!(format!("{}", c), "-2");

  // sub mpz_r - ui_t
  let a = &mpz_s::from(-3);
  let b = 5 as ui_t;
  let c = a - b;
  assert_eq!(format!("{}", c), "-8");

  // sub ui_t - mpz_s
  let a = 3 as ui_t;
  let b = mpz_s::from(5);
  let c = a - b;
  assert_eq!(format!("{}", c), "-2");

  // sub ui_t - mpz_r
  let a = 3 as ui_t;
  let b = &mpz_s::from(-5);
  let c = a - b;
  assert_eq!(format!("{}", c), "8");

  // sub assign mpz_s -= mpz_s
  let mut a = mpz_s::from(-3);
  let b = mpz_s::from(7);
  a -= b;
  assert_eq!(format!("{}", a), "-10");

  // sub assign mpz_s -= mpz_r
  let mut a = mpz_s::from(-2);
  let b = &mpz_s::from(5);
  a -= b;
  assert_eq!(format!("{}", a), "-7");

  // sub assign mpz_t -= mpz_s
  let a = &mut mpz_s::from(-3);
  let b = mpz_s::from(7);
  *a -= b;
  assert_eq!(format!("{}", a), "-10");

  // sub assign mpz_t -= mpz_r
  let a = &mut mpz_s::from(-2);
  let b = &mpz_s::from(5);
  *a -= b;
  assert_eq!(format!("{}", a), "-7");

  // sub assign mpz_s -= ui_t
  let mut a = mpz_s::from(-3);
  a -= 7 as ui_t;
  assert_eq!(format!("{}", a), "-10");

  // sub assign mpz_t -= ui_t (now same as mpz_s with *a)
  let a = &mut mpz_s::from(-3);
  *a -= 7 as ui_t;
  assert_eq!(format!("{}", a), "-10");

  // add mpz_s + mpz_s
  let a = mpz_s::from(-2);
  let b = mpz_s::from(5);
  let c = a + b;
  assert_eq!(format!("{}", c), "3");

  // add mpz_r + mpz_r
  let a = &mpz_s::from(-2);
  let b = &mpz_s::from(5);
  let c = a + b;
  assert_eq!(format!("{}", c), "3");

  // add mpz_s + ui_t
  let a = mpz_s::from(3);
  let b = 5 as ui_t;
  let c = a + b;
  assert_eq!(format!("{}", c), "8");

  // add mpz_r + ui_t
  let a = &mpz_s::from(-3);
  let b = 5 as ui_t;
  let c = a + b;
  assert_eq!(format!("{}", c), "2");

  // add ui_t + mpz_s
  let a = 3 as ui_t;
  let b = mpz_s::from(5);
  let c = a + b;
  assert_eq!(format!("{}", c), "8");

  // add ui_t + mpz_r
  let a = 3 as ui_t;
  let b = &mpz_s::from(-5);
  let c = a + b;
  assert_eq!(format!("{}", c), "-2");

  // add assign mpz_s += mpz_s
  let mut a = mpz_s::from(-3);
  let b = mpz_s::from(7);
  a += b;
  assert_eq!(format!("{}", a), "4");

  // add assign mpz_s += mpz_r
  let mut a = mpz_s::from(-2);
  let b = &mpz_s::from(5);
  a += b;
  assert_eq!(format!("{}", a), "3");

  // add assign mpz_t += mpz_s (now same as mpz_s with *a)
  let a = &mut mpz_s::from(-3);
  let b = mpz_s::from(7);
  *a += b;
  assert_eq!(format!("{}", a), "4");

  // add assign mpz_t += mpz_r (now same as mpz_s with *a)
  let a = &mut mpz_s::from(-2);
  let b = &mpz_s::from(5);
  *a += b;
  assert_eq!(format!("{}", a), "3");

  // add assign mpz_s += ui_t
  let mut a = mpz_s::from(-3);
  a += 7 as ui_t;
  assert_eq!(format!("{}", a), "4");

  // add assign mpz_t += ui_t (now same as mpz_s with *a)
  let a = &mut mpz_s::from(-3);
  *a += 7 as ui_t;
  assert_eq!(format!("{}", a), "4");

  // mul mpz_s * mpz_s
  let a = mpz_s::from(3);
  let b = mpz_s::from(5);
  let c = a * b;
  assert_eq!(format!("{}", c), "15");

  // mul mpz_r * mpz_r
  let a = &mpz_s::from(3);
  let b = &mpz_s::from(-5);
  let c = a * b;
  assert_eq!(format!("{}", c), "-15");

  // mul mpz_s * ui_t
  let a = mpz_s::from(3);
  let b = 5 as ui_t;
  let c = a * b;
  assert_eq!(format!("{}", c), "15");

  // mul mpz_r * ui_t
  let a = &mpz_s::from(-3);
  let b = 5 as ui_t;
  let c = a * b;
  assert_eq!(format!("{}", c), "-15");

  // mul ui_t * mpz_s
  let a = 3 as ui_t;
  let b = mpz_s::from(5);
  let c = a * b;
  assert_eq!(format!("{}", c), "15");

  // mul ui_t * mpz_r
  let a = 3 as ui_t;
  let b = &mpz_s::from(-5);
  let c = a * b;
  assert_eq!(format!("{}", c), "-15");

  // mul mpz_s * si_t
  let a = mpz_s::from(-3);
  let b = -5 as si_t;
  let c = a * b;
  assert_eq!(format!("{}", c), "15");

  // mul mpz_r * si_t
  let a = &mpz_s::from(3);
  let b = -5 as si_t;
  let c = a * b;
  assert_eq!(format!("{}", c), "-15");

  // mul si_t * mpz_s
  let a = -3 as si_t;
  let b = mpz_s::from(5);
  let c = a * b;
  assert_eq!(format!("{}", c), "-15");

  // mul si_t * mpz_r
  let a = -3 as si_t;
  let b = &mpz_s::from(-5);
  let c = a * b;
  assert_eq!(format!("{}", c), "15");

  // mul assign mpz_s *= mpz_s
  let mut a = mpz_s::from(-3);
  let b = mpz_s::from(7);
  a *= b;
  assert_eq!(format!("{}", a), "-21");

  // mul assign mpz_s *= mpz_r
  let mut a = mpz_s::from(-2);
  let b = &mpz_s::from(5);
  a *= b;
  assert_eq!(format!("{}", a), "-10");

  // mul assign mpz_t *= mpz_s
  let a = &mut mpz_s::from(-3);
  let b = mpz_s::from(7);
  *a *= b;
  assert_eq!(format!("{}", a), "-21");

  // mul assign mpz_t *= mpz_r
  let a = &mut mpz_s::from(-2);
  let b = &mpz_s::from(5);
  *a *= b;
  assert_eq!(format!("{}", a), "-10");

  // mul assign mpz_s *= ui_t
  let mut a = mpz_s::from(-3);
  a *= 7 as ui_t;
  assert_eq!(format!("{}", a), "-21");

  // mul assign mpz_t *= ui_t (now same as mpz_s with *a)
  let a = &mut mpz_s::from(-3);
  *a *= 7 as ui_t;
  assert_eq!(format!("{}", a), "-21");

  // mul assign mpz_s *= si_t
  let mut a = mpz_s::from(-3);
  a *= -7 as si_t;
  assert_eq!(format!("{}", a), "21");

  // mul assign mpz_t *= si_t (now same as mpz_s with *a)
  let a = &mut mpz_s::from(-3);
  *a *= -7 as si_t;
  assert_eq!(format!("{}", a), "21");

  // div mpz_s / mpz_s
  let a = mpz_s::from(13);
  let b = mpz_s::from(-5);
  let c = a / b;
  assert_eq!(format!("{}", c), "-2");

  // div mpz_r / mpz_r
  let a = &mpz_s::from(-17);
  let b = &mpz_s::from(5);
  let c = a / b;
  assert_eq!(format!("{}", c), "-3");

  // div mpz_s / ui_t
  let a = mpz_s::from(13);
  let c = a / 5 as ui_t;
  assert_eq!(format!("{}", c), "2");

  // div mpz_r / ui_t
  let a = &mpz_s::from(-17);
  let c = a / 5 as ui_t;
  assert_eq!(format!("{}", c), "-3");

  // div assign mpz_s /= mpz_s
  let mut a = mpz_s::from(13);
  let b = mpz_s::from(-5);
  a /= b;
  assert_eq!(format!("{}", a), "-2");

  // div assign mpz_s /= mpz_r
  let mut a = mpz_s::from(-17);
  let b = &mpz_s::from(5);
  a /= b;
  assert_eq!(format!("{}", a), "-3");

  // div assign mpz_t /= mpz_s
  let a = &mut mpz_s::from(13);
  let b = mpz_s::from(-5);
  *a /= b;
  assert_eq!(format!("{}", a), "-2");

  // div assign mpz_t /= mpz_r
  let a = &mut mpz_s::from(-17);
  let b = &mpz_s::from(-5);
  *a /= b;
  assert_eq!(format!("{}", a), "3");

  // div assign mpz_s /= ui_t
  let mut a = mpz_s::from(-17);
  a /= 3 as ui_t;
  assert_eq!(format!("{}", a), "-5");

  // div assign mpz_t /= ui_t (now same as mpz_s with *a)
  let a = &mut mpz_s::from(-17);
  *a /= 5 as ui_t;
  assert_eq!(format!("{}", a), "-3");

  // rem mpz_s % mpz_s
  let a = mpz_s::from(13);
  let b = mpz_s::from(-5);
  let c = a % b;
  assert_eq!(format!("{}", c), "3"); // *** -5 * -2 + 3

  // rem mpz_r % mpz_r
  let a = &mpz_s::from(-17);
  let b = &mpz_s::from(5);
  let c = a % b;
  assert_eq!(format!("{}", c), "-2"); // *** 5 * -3 - 2

  // rem mpz_s % ui_t
  let a = mpz_s::from(13);
  let c = a % 5 as ui_t;
  assert_eq!(format!("{}", c), "3");

  // rem mpz_r % ui_t
  let a = &mpz_s::from(-17);
  let c = a % 5 as ui_t;
  assert_eq!(format!("{}", c), "-2"); // *** 5 * -3 - 2

  // rem assign mpz_s %= mpz_s
  let mut a = mpz_s::from(13);
  let b = mpz_s::from(-5);
  a %= b;
  assert_eq!(format!("{}", a), "3"); // *** -5 * -2 + 3

  // rem assign mpz_s %= mpz_r
  let mut a = mpz_s::from(-17);
  let b = &mpz_s::from(5);
  a %= b;
  assert_eq!(format!("{}", a), "-2"); // *** 5 * -3 - 2

  // rem assign mpz_t %= mpz_s
  let a = &mut mpz_s::from(13);
  let b = mpz_s::from(-5);
  *a %= b;
  assert_eq!(format!("{}", a), "3"); // *** -5 * -2 + 3

  // rem assign mpz_t %= mpz_r
  let a = &mut mpz_s::from(-17);
  let b = &mpz_s::from(-5);
  *a %= b;
  assert_eq!(format!("{}", a), "-2"); // *** -5 * 3 - 2

  // rem assign mpz_s %= ui_t
  let mut a = mpz_s::from(-17);
  a %= 3 as ui_t;
  assert_eq!(format!("{}", a), "-2"); // *** 3 * -5 - 2

  // rem assign mpz_t %= ui_t (now same as mpz_s with *a)
  let a = &mut mpz_s::from(-17);
  *a %= 5 as ui_t;
  assert_eq!(format!("{}", a), "-2"); // *** 5 * -3 - 2

  // div and rem with mut
  let mut a = mpz_s::from(-15);
  let mut b = mpz_s::from(3);
  a -= 2;
  b += 2;
//  let q = &a / &b; // essentially needless &mut a to use binary operator
  let q = &mut a / &mut b; // It is allowed with onforward_ref_mut_binop macro
  let r = a % b; // a b are moved, use ref before here
  assert_eq!(format!("{}, {}", q, r), "-3, -2"); // *** tdiv_qr

  // div and rem with mut
  let a = &mut mpz_s::from(-15);
  let b = &mut mpz_s::from(3);
  *a -= 2;
  *b += 2;
  let q = &*a / &*b; // '&*a' means cast '&mut a' to '&a'
  let r = a % b; // a b are moved, use ref before here
  assert_eq!(format!("{}, {}", q, r), "-3, -2"); // *** tdiv_qr

  // div and rem for Rust primitive
  let a = -17i32;
  let b = 5i32;
  let q = a / b;
  let r = a % b;
  assert_eq!(format!("{}, {}", q, r), "-3, -2"); // *** tdiv_qr

  // div and rem for Rust primitive
  let a = -17i32;
  let b = -5i32;
  let q = a / b;
  let r = a % b;
  assert_eq!(format!("{}, {}", q, r), "3, -2"); // *** tdiv_qr

  // div and rem based on fdiv_qr
  let a = &mpz_s::from(-17);
  let b = &mpz_s::from(5);
  let q = a.fdiv_q(b);
  let r = a.fdiv_r(b);
  assert_eq!(format!("{}, {}", q, r), "-4, 3"); // *** fdiv_qr

  // bitand mpz_s &amp; mpz_s
  let a = &mpz_s::from(0);
  let b = &mpz_s::from(0);
  let c = !a & !b;
  assert_eq!(format!("{}", c), "-1");

  // bitand assign mpz_s &amp;= mpz_s
  let mut a = mpz_s::from(!0 as si_t);
  let b = &mpz_s::from(!0 as si_t);
  a &= b;
  assert_eq!(format!("{}", c), "-1");

  // bitor mpz_s | mpz_s
  let a = &mpz_s::from(0);
  let b = &mpz_s::from(0);
  let c = a | !b;
  assert_eq!(format!("{}", c), "-1");

  // bitor assign mpz_s |= mpz_s
  let mut a = mpz_s::from(!0 as si_t);
  let b = &mpz_s::from(!0 as si_t);
  a |= !b;
  assert_eq!(format!("{}", c), "-1");

  // bitxor mpz_s ^ mpz_s
  let a = &mpz_s::from(0);
  let b = &mpz_s::from(0);
  let c = a ^ !b;
  assert_eq!(format!("{}", c), "-1");

  // bitxor assign mpz_s ^= mpz_s
  let mut a = mpz_s::from(!0 as si_t);
  let b = &mpz_s::from(!0 as si_t);
  a ^= !b;
  assert_eq!(format!("{}", c), "-1");

  // to avoid size difference of unsigned long and unsigned long long
  let zinv64 = (!0 as i32) as si_t; // ffffffffffffffff
  let zinv32 = (!0 as u32) as ui_t; // 00000000ffffffff
  let a = mpz_s::from(zinv64);
  let b = mpz_s::from(zinv32);
  let c = a & !b;
//  let c = !(a & b); // same result as above
  assert_eq!(format!("{}", c.hexstr()), "-100000000"); // not ffffffff00000000
  let a = mpz_s::from(zinv64);
  let b = mpz_s::from(zinv32);
  let c = !a | !b;
  assert_eq!(format!("{}", c.hexstr()), "-100000000"); // not ffffffff00000000
  let a = mpz_s::from(zinv64);
  let b = mpz_s::from(zinv32);
  let c = a ^ b;
  assert_eq!(format!("{}", c.hexstr()), "-100000000"); // not ffffffff00000000

  // shl mpz_s
  let a = mpz_s::from(zinv32);
  let c = a << 16;
  assert_eq!(format!("{}", c.hexstr()), "ffffffff0000");

  // shl assign mpz_s
  let mut a = mpz_s::from(zinv32);
  a <<= 8;
  assert_eq!(format!("{}", a.hexstr()), "ffffffff00");

  // shr mpz_s
  let a = !mpz_s::from(zinv32);
  let c = a >> 16; // ***arithmetic***
  assert_eq!(format!("{}", c.hexstr()), "-10000"); // not ffffffffffff0000

  // shr assign mpz_s
  let mut a = !mpz_s::from(zinv32);
  a >>= 8; // ***arithmetic***
  assert_eq!(format!("{}", a.hexstr()), "-1000000"); // not ffffffffff000000

  // shr for Rust primitive
  let b = 12*16u8;
  assert_eq!(format!("{}", b), "192");
  assert_eq!(format!("{:2x}", b), "c0");
  assert_eq!(format!("{:2x}", b >> 2), "30"); // ***logic***
  let b = (12*16u8) as i8;
  assert_eq!(format!("{}", b), "-64");
  assert_eq!(format!("{:2x}", b), "c0");
  assert_eq!(format!("{:2x}", b >> 2), "f0"); // ***arithmetic***

  // neg mpz_s
  let a = mpz_s::from(-2);
  let c = -a;
  assert_eq!(format!("{}", c), "2");

  // neg mpz_r
  let a = &mpz_s::from(2);
  let c = -a;
  assert_eq!(format!("{}", c), "-2");

  // not mpz_s
  let a = mpz_s::from(-2);
  let c = !a;
  assert_eq!(format!("{}", c), "1");

  // not mpz_r
  let a = &mpz_s::from(1);
  let c = !a;
  assert_eq!(format!("{}", c), "-2");
}
