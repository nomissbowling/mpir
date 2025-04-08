mpir
====

partial Rust porting of mpir multiple precision library based on gmp mpfr


Sample
------

see also [sum_arctan_gregory() source](https://docs.rs/mpir/latest/mpir/prim/mpf/struct.__mpf_struct.html#method.sum_arctan_gregory)

```Rust
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
  let f = mpf_s::from(q);
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

  // mpz and mpf (prepare and reset)
  let a = &mut mpz_s::init();
  let f = &mut mpf_s::init();
  let g = &mut mpf_s::init();

  // mpz and mpf (to be operator) check about significant digits
  assert_eq!(format!("{}", a.set_str("987654321098765432109", 10)),
    "987654321098765432109"); // 21 digits
  assert_eq!(format!("{}", f.set_z(a).div(g.set_str("1.0e+11", 10))), // drift
    "0.98765432109876543211e+10"); // 20 digits by default formatter
  assert_eq!(f.fmtstr(10, 22), // check to 22 digits
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
  (0..=20).for_each(|n: usize| {
    let t = &mpz_s::fact(n as ui_t);
    assert_eq!(format!("{}! = {}", n, t), format!("{}! = {}", n, facts[n]));
    let u = &mpz_s::fac_ui(n as ui_t);
    assert_eq!(format!("{}! = {}", n, t), format!("{}! = {}", n, u));
  });

  // mpz fact (to be operator) cached
  let m = &mut HashMap::<ui_t, mpz_s>::new();
  (0..=20).for_each(|n: usize| {
    let t = &mpz_s::fact_cached(n as ui_t, m);
    assert_eq!(format!("{}! = {}", n, t), format!("{}! = {}", n, facts[n]));
  });

  // mpq (to be operator)
  let q = &mpq_s::from((2, 8 as ui_t));
  assert_eq!(format!("{}", q), "2/8");

  // mpf prec
//  assert_eq!(mpf_get_default_prec(), 64); // may be 64
  mpf_set_default_prec(100); // 100 set to 128 bits (step by 2**n)
//  assert_eq!(mpf_get_default_prec(), 128); // may be 128 (about 38 digits)
  let digits = mpf_s::calc_digits_from_bits(128);
  assert_eq!(digits, 38); // may be 38

  // mpf significant digits (to be operator) test loss of digits on display
  let disp_digits = digits + 3; // set disp_digits to over prec
  let f = &mut mpf_s::from("1.0e-19");
  let e = &mpf_s::from("1.0e-50");
  assert_eq!(e.fmtstr(10, disp_digits), "0.1e-49");
  assert_eq!(f.fmtstr(10, disp_digits), "0.1e-18");
  // f.add(e) as 0.99999999999999999999e-19 without mpf_set_default_prec(100)
  assert_eq!(f.add(e).fmtstr(10, disp_digits), // use disp_digits
    "0.1000000000000000000000000000000099999999e-18"); // disp over prec
  assert_eq!(f.fmtstr(10, digits), // use digits
    "0.10000000000000000000000000000001e-18"); // disp as match with prec

  // mpf calc napier (to be operator)
  let digits = 150;
  mpf_set_default_prec(mpf_s::calc_bits_from_digits(digits + 3));
  let e = &util::Sigma::from(digits).calc_napier(&mpf_s::from(1.0));
  assert_eq!(format!("{}", e),
    "0.27182818284590452354e+1");
  assert_eq!(e.fmtstr(10, digits),
    "0.271828182845904523536028747135266249775724709369995957496696762772407663035354759457138217852516642742746639193200305992181741359662904357290033429526e+1");
/*
  2.
  7182818284 5904523536 0287471352 6624977572 4709369995
  9574966967 6277240766 3035354759 4571382178 5251664274
  2746639193 2003059921 8174135966 2904357290 0334295260
  ...
*/
```


Misc
----

- [mpz ...](https://gmplib.org/manual/Integer-Functions)
- [mpz cmp ...](https://gmplib.org/manual/Integer-Comparisons)
- [mpz fac_ui fib_ui ...](https://gmplib.org/manual/Number-Theoretic-Functions)
- [mpz sqrt root ...](https://gmplib.org/manual/Integer-Roots)
- [mpz add sub mul addmul ...](https://gmplib.org/manual/Integer-Arithmetic)
- [mpz *div mod ...](https://gmplib.org/manual/Integer-Division)
- [mpz pow ...](https://gmplib.org/manual/Integer-Exponentiation)

- [mpz logic bit](https://gmplib.org/manual/Integer-Logic-and-Bit-Fiddling)
- [mpz misc](https://gmplib.org/manual/Miscellaneous-Integer-Functions)
- [mpz special](https://gmplib.org/manual/Integer-Special-Functions)
- [mpz random](https://gmplib.org/manual/Integer-Random-Numbers)

- [mpf ...](https://gmplib.org/manual/Rational-Number-Functions)
- [mpf cmp ...](https://gmplib.org/manual/Float-Comparison)
- [mpf sqrt add sub mul div pow ...](https://gmplib.org/manual/Float-Arithmetic)

- [mpf misc](https://gmplib.org/manual/Miscellaneous-Float-Functions)

- [mpq ...](https://gmplib.org/manual/Rational-Number-Functions)
- [mpq cmp ...](https://gmplib.org/manual/Comparing-Rationals)
- [mpq add sub mul div inv ...](https://gmplib.org/manual/Rational-Arithmetic)


Links
-----

- [https://crates.io/crates/mpir](https://crates.io/crates/mpir)
- [https://github.com/nomissbowling/mpir](https://github.com/nomissbowling/mpir)


Requirements
------------

- [gmp](https://gmplib.org/)
- [mpir](https://github.com/ChillMagic/MPIR-Binary)


License
-------

MIT License
