mpir
====

partial Rust porting of mpir multiple precision library based on gmp mpfr


Sample
------

```Rust
  // mpz and mpf (prepare and reset)
  let a = &mut mpz_s::init();
  let f = &mut mpf_s::init();
  let g = &mut mpf_s::init();

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

  // mpq (to be operator)
  let q = &mut mpq_s::init();
  assert_eq!(format!("{}", q.set_ui(2, 8)), "2/8");
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

- [mpf ...](https://gmplib.org/manual/Rational-Number-Functions)
- [mpf cmp ...](https://gmplib.org/manual/Float-Comparison)
- [mpf sqrt add sub mul div pow ...](https://gmplib.org/manual/Float-Arithmetic)

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
