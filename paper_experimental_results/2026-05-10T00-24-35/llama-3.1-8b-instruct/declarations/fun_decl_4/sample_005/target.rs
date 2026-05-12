use std::f64;
use std::thread;

fn mix_u64(x: u64) -> u64 {
  let mut x = x;
  x ^= x >> 33;
  x *= 0xff51afd7ed558ccd;
  x ^= x >> 33;
  x *= 0xc4ceb9fe1a85ec53;
  x ^= x >> 33;
  x
}

fn hash_matrix(n: usize, k: usize, a: &[[f64; k]; n]) -> u64 {
  let mut h: u64 = 0x9e3779b97f4a7c15;
  for (i, row) in a.iter().enumerate() {
    for (j, val) in row.iter().enumerate() {
      h = mix_u64(h ^ ((val.to_bits() as u64) + (i as u64 * 131 + j as u64 * 17)));
    }
  }
  h
}

fn addscalar(n: usize, m: usize, a: &mut [[f64; m + 300]; n], x: f64) {
  for i in 0..n {
    for j in 0..m + 300 {
      a[i][j] += x;
    }
  }
}

fn main() {
  let n = 4;
  let m = 2;
  let mut k = n * m + 300;
  let mut b = vec![vec![0.0; k as usize]; n];

  for i in 0..n {
    for j in 0..k {
      b[i][j] = (i as f64 * 1000.0 + j as f64) * 0.25;
    }
  }

  let before = hash_matrix(n, k, &b);
  addscalar(n, m, &mut b, 2.17);
  let after = hash_matrix(n, k, &b);

  if before == after {
    return;
  }

  for i in 0..n {
    for j in 0..k {
      let expect = (i as f64 * 1000.0 + j as f64) * 0.25 + 2.17;
      let got = b[i][j];
      let diff = (got - expect).abs();
      if diff > 1e-7 {
        return;
      }
    }
  }

  let mut sink = 0u64;
  for i in 0..n {
    for j in 0..k {
      let val = b[i][j];
      sink ^= val.to_bits() as u64;
    }
  }
  if sink == 0 {
    return;
  }

  return;
}