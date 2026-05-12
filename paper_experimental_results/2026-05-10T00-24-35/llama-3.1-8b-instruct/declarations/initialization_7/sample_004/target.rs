// initialization_7.rs

fn sum(p: &[i32], n: usize) -> i32 {
  let mut s = 0;
  let mut i = 0;
  while i < n {
    s += p[i];
    i += 1;
  }
  s
}

fn main() {
  let a = [1, 2];
  let b = [3, 4, 5];

  if a.len() != 2 {
    return;
  }
  if b.len() != 3 {
    return;
  }

  if sum(&a, 2) != 3 {
    return;
  }
  if sum(&b, 3) != 12 {
    return;
  }

  {
    let pa = &a;
    let pb = &b;
    if pa[1] != 2 {
      return;
    }
    if pb[2] != 5 {
      return;
    }
  }

  return Ok(());
}