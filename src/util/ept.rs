//! era prime table
//!

use std::collections::BTreeMap;

use crate::prim::{typ::*, mpz::*}; // *, mpf::*, mpq::*, randstate::*, gmp::*

/// era prime table ui
pub struct EraPrimeTableUI {
  /// maxera
  pub maxera: mp_size_t,
  /// era
  pub era: Vec<bool>,
  /// tbl
  pub tbl: BTreeMap<mp_size_t, mpz_s>
}

/// impl EraPrimeTableUI
impl EraPrimeTableUI {
  /// new
  pub fn new(maxera: mp_size_t) -> Self {
    let mut era = vec![false; maxera + 1]; // allows [maxera] but NC
    (0..=1).for_each(|i| { era[i] = true; });
    (2..=(maxera as f64).sqrt() as mp_size_t).for_each(|i| { // isqrt
      if era[i] { return; } // continue (faster by skip)
      (2..=maxera/i).for_each(|j| {
//        if era[j] { return; } // continue (do not skip here)
        if !era[i * j] { era[i * j] = true; }
      });
    });
    let mut tbl = BTreeMap::<mp_size_t, mpz_s>::new();
    let _t = (2..maxera).fold((&mut tbl, 0), |(tbl, cnt), i| {
      if era[i] { (tbl, cnt) }
      else { tbl.insert(cnt, mpz_s::init_set_ui(i as ui_t)); (tbl, cnt + 1) }
    });
    EraPrimeTableUI{maxera, era, tbl}
  }

  /// get_maxera
  pub fn get_maxera(&self) -> mp_size_t {
    self.maxera
  }

  /// is_prime
  pub fn is_prime(&self, n: mp_size_t) -> Option<bool> {
    if n >= self.maxera { None } else { Some(!self.era[n]) }
  }

  /// nprimes
  pub fn nprimes(&self) -> mp_size_t {
    self.tbl.len()
  }

  /// nth_prime
  /// - when not found ***generate too slow***
  /// - gen = 0: panic
  /// - gen = not 0: generate nextprime (1: probably, 2: exactly)
  pub fn nth_prime(&mut self, n: mp_size_t, gen: int_t) -> mpz_t {
    let mut m = self.tbl.len() - 1;
    if n <= m { self.tbl.get_mut(&n).expect("prime in the table") }
    else {
      match gen {
      0 => panic!("tbl does not have nth prime"),
      _ => {
        while m < n {
          let p = self.tbl.get_mut(&m).expect("generate next of last prime");
          let mut q = p.nextprime();
          if q.probab_prime_p(3) >= gen {
            m += 1;
            self.tbl.insert(m, q);
          }
        }
        self.tbl.get_mut(&m).expect("nth prime")
      }
      }
    }
  }
}
