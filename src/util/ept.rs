//! era prime table
//!

use std::collections::BTreeMap;

use crate::prim::{typ::*, mpz::*}; // *, mpf::*, mpq::*, randstate::*, gmp::*

/// era prime table ui
pub struct EraPrimeTableUI {
  /// maxera
  pub maxera: mp_size_t,
  /// era bits
  pub era: mpz_s,
  /// tbl
  pub tbl: BTreeMap<mp_size_t, mpz_s>
}

/// impl EraPrimeTableUI
impl EraPrimeTableUI {
  /// new
  pub fn new(maxera: mp_size_t) -> Self {
    let mut era = mpz_s::from(0); // lsb = 0, msb = maxera allows but NC
    let even64 = maxera + 63 - (maxera - 1) % 64; // even bit always non prime
    era.setbit(even64 as ui_t); // pre expand mpz before era filter (to faster)
    (0..=1).for_each(|i| { era.setbit(i); });
    (2..=(maxera as f64).sqrt() as ui_t).for_each(|i| { // isqrt
      if era.tstbit(i) { return; } // continue (faster by skip)
      (2..=maxera as ui_t / i).for_each(|j| {
//        if era.tstbit(j) { return; } // continue (do not skip here)
        if !era.tstbit(i * j) { era.setbit(i * j); }
      });
    });
    let mut tbl = BTreeMap::<mp_size_t, mpz_s>::new();
    let _t = (2..maxera as ui_t).fold((&mut tbl, 0), |(tbl, cnt), i| {
      if era.tstbit(i) { (tbl, cnt) }
      else { tbl.insert(cnt, mpz_s::from(i)); (tbl, cnt + 1) }
    });
    EraPrimeTableUI{maxera, era, tbl}
  }

  /// get_maxera
  #[inline]
  pub fn get_maxera(&self) -> mp_size_t {
    self.maxera
  }

  /// is_prime
  #[inline]
  pub fn is_prime(&self, n: mp_size_t) -> Option<bool> {
    if n >= self.maxera { None } else { Some(!self.era.tstbit(n as ui_t)) }
  }

  /// nprimes
  #[inline]
  pub fn nprimes(&self) -> mp_size_t {
    self.tbl.len()
  }

  /// nth_prime
  /// - when not found ***generate too slow***
  /// - gen = 0: panic
  /// - gen = not 0: generate nextprime (1: probably, 2: exactly)
  pub fn nth_prime(&mut self, n: mp_size_t, gen: int_t) -> mpz_r {
    let mut m = self.tbl.len() - 1;
    if n <= m { self.tbl.get(&n).expect("prime in the table") }
    else {
      match gen {
      0 => panic!("tbl does not have nth prime"),
      _ => {
        while m < n {
          let p = self.tbl.get(&m).expect("generate next of last prime");
          let q = p.nextprime();
          if q.probab_prime_p(3) >= gen {
            m += 1;
            self.tbl.insert(m, q);
          }
        }
        self.tbl.get(&m).expect("nth prime")
      }
      }
    }
  }
}
