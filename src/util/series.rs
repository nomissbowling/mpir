//! series
//!

use std::collections::HashMap;
use std::convert::From;

use crate::prim::{typ::*, mpz::*, mpf::*};
use crate::util;

/// SumArctan
pub struct SumArctan {
  /// ax: Vec&lt;(sgn: ui_t, a: ui_t, x: ui_t)&gt;
  pub ax: Vec<(ui_t, ui_t, ui_t)>
}

/// impl From&lt;Vec&lt;(si_t, ui_t)&gt;&gt; for SumArctan
/// - from Vec&lt;(a: si_t, x: ui_t)&gt;
/// - to Vec&lt;(sgn: ui_t, a: ui_t, x: ui_t)&gt;
impl From<Vec<(si_t, ui_t)>> for SumArctan {
  /// SumArctan from Vec&lt;(si_t, ui_t)&gt;
  #[inline]
  fn from(ax: Vec<(si_t, ui_t)>) -> Self {
    let ax = ax.into_iter().map(|(a, x)|
      if a < 0 { (1, -a as ui_t, x) } else { (0, a as ui_t, x) } // care range
    ).collect();
    SumArctan{ax}
  }
}

/// impl SumArctan
impl SumArctan {
  /// calc pi/4 sum arctan Gregory
  /// - arctan x = sigma[n=0-&gt;inf]{(-1 ** n) * x**(2n+1) / (2n+1)}
  /// - sum_n = sigma[k=0-&gt;ax.len](a_k * arctan_n x_k)
  /// - result = sigma[n=0-&gt;m]sum_n
  /// - inner loop may be slow (should be outer mul a_k) to check stop iterator
  pub fn sum_arctan_gregory(&self, m: ui_t) -> mpf_s {
    let mut sa = mpf_s::from(0);
    let _s = (0..=m).try_fold(&mut sa, |mut sa, n| {
      let pre = &mpf_s::from(&*sa);
      let k = 2 * n + 1;
      let mut sn = mpf_s::from(0);
      let _a = self.ax.iter().fold(&mut sn, |mut sn, (sgn, a, x)| {
//      let s = a * (mpz_s::ui_pow_ui(*x, k) * k).inv_f(); // outer a to faster
        let s = a / mpf_s::from(&(mpz_s::ui_pow_ui(*x, k) * k));
        if 1 == ((n & 1) ^ sgn) { sn -= s; } else { sn += s; }
        sn
      });
      sa += sn;
      if sa.cmp(pre) == 0 { None } else { Some(sa) }
    });
    sa
  }
}

/// Sigma
pub struct Sigma {
  /// digits
  pub digits: mp_size_t
}

/// impl From&lt;mp_size_t&gt; for Sigma
impl From<mp_size_t> for Sigma {
  /// Sigma from mp_size_t
  #[inline]
  fn from(d: mp_size_t) -> Self {
    Sigma{digits: d}
  }
}

/// impl Sigma
impl Sigma {
  /// calc pi Takano-Kanada
  /// - 12arctan(1/49) + 32arctan(1/57) - 5arctan(1/239) + 12arctan(1/110443)
  pub fn calc_pi_takano(&self) -> mpf_s {
    let recursion = self.digits.ilog2().pow(4); // ***loss of digits when big***
    let ax = vec![(12, 49), (32, 57), (-5, 239), (12, 110443)];
    let pi = SumArctan::from(ax).sum_arctan_gregory(recursion);
    pi * 4
  }

  /// calc pi Machin
  /// - pi/4 = 4arctan(1/5) - arctan(1/239)
  pub fn calc_pi_machin(&self) -> mpf_s {
    let recursion = self.digits.ilog2().pow(4); // ***loss of digits when big***
    let ax = vec![(4, 5), (-1, 239)];
    let pi = SumArctan::from(ax).sum_arctan_gregory(recursion);
    pi * 4
  }

  /// calc pi Leibniz create new instance ***CAUTION too slow digits &gt;= 7***
  /// - pi/4 = arctan(1)
  pub fn calc_pi_leibniz(&self) -> mpf_s {
    let recursion = 10usize.pow(self.digits as u32) as ui_t; // ipow
    let ax = vec![(1, 1)];
    let pi = SumArctan::from(ax).sum_arctan_gregory(recursion);
    pi * 4
  }

  /// calc pi Euler create new instance ***CAUTION too slow digits &gt;= 9***
  pub fn calc_pi_euler(&self) -> mpf_s {
    let mut pi = mpf_s::from(1);
    let g = &mut mpf_s::from(0);
    let d = 10usize.pow(self.digits as u32);
    let mut ept = util::EraPrimeTableUI::new(d);
    let _p = (0..ept.nprimes()).fold(&mut pi, |pi: mpf_t, k| {
      let np = ept.nth_prime(k, 0); // must be in the table
      let pp = &mpz_s::pow_ui(np, 2);
      pi.mul(&mut g.set_z(pp).ui_div(1).ui_sub(1).ui_div(1));
      pi
    });
    pi.mul_ui(6).sqrt()
  }

  /// calc pi Gauss-Legendre create new instance
  pub fn calc_pi_gauss_legendre(&self) -> mpf_s {
    let recursion = self.digits.ilog2(); // or + 1
    let a = &mut mpf_s::from(1);
    let b = &mut mpf_s::sqrt_ui(2);
    let t = &mut mpf_s::from(4);
    let p = &mut mpf_s::from(1);
    let (a, b, t, _p) = (0..recursion).fold((a, b.ui_div(1), t.ui_div(1), p),
      |(a, b, t, p), _k| {
      let na = &mut mpf_s::from(&*a); // next a, keep a
      na.add(b).div_ui(2);
      let nb = &b.mul(a).sqrt(); // next b, b will be broken
      t.sub(mpf_s::pow_ui(a.sub(na), 2).mul(p)); // modify t, a will be broken
      (a.set(na), b.set(nb), t, p.mul_ui(2)) // modify p
    });
    let mut pi = mpf_s::pow_ui(a.add(b), 2);
    pi.div_ui(4).div(t);
    pi
  }

  /// calc Napier create new instance
  pub fn calc_napier(&self, x: mpf_r) -> mpf_s {
    // significant digits of calc napier by Stirling's approximation
    let significant_digits_of_calc_napier = |n: f64| -> f64 {
      let p = (2.0 * std::f64::consts::PI * n).sqrt().log10(); // .log(10.0)
      let q = n * (n / std::f64::consts::E).log10(); // use preset napier f64
      p + q - 1.0
    };

    let mut e = mpf_s::from(0);
//    e.set_str("2.71828182845904523536", 10); // when self.digits = 21
    let g = &mut mpf_s::from(0);
    let m = &mut HashMap::<ui_t, mpz_s>::new();
    let _s = (0..=self.digits as ui_t).try_fold(&mut e, |e: mpf_t, i| {
      let d = if i == 0 { 0 } // 0.log10() is NaN
        else { significant_digits_of_calc_napier(i as f64) as mp_size_t };
//      println!("i {}, d {}", i, d);
      let n = &mpz_s::fact_cached(i, m);
      let f = &mut mpf_s::pow_ui(x, i);
      e.add(f.div(g.set_z(n)));
/*
      if d >= self.digits || i == self.digits as ui_t {
        println!("i {} g {} f {} e {}", i,
         g.fmtstr(10, self.digits),
         f.fmtstr(10, self.digits),
         e.fmtstr(10, self.digits + 3));
      }
*/
      if d >= self.digits { None } else { Some(e) }
    }); // skip .ok_or_else(|| {...}) .expect(...) when break by None
    e
  }
}
