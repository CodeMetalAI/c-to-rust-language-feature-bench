static mut gate4: i32 = 1;

fn copy_restrict(n: usize, p: &mut i32, q: &mut i32) {
  while n > 0 {
    *p = *q;
    p = p.add(1);
    q = q.add(1);
    n -= 1;
  }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
  for i in 0..n {
    p[i] = base + i * 5 + 1;
  }
}

fn same(a: &[i32], b: &[i32], n: usize) -> bool {
  for i in 0..n {
    if a[i] != b[i] {
      return false;
    }
  }
  true
}

fn sum(a: &[i32], n: usize) -> i32 {
  let mut s = 0;
  for i in 0..n {
    s += a[i];
  }
  s
}

fn main() {
  let mut x: [i32; 32] = [0; 32];
  let mut y: [i32; 32] = [0; 32];
  let mut y_snapshot: [i32; 32] = [0; 32];

  fill(&mut x, 32, 10);
  fill(&mut y, 32, 100);

  for i in 0..32 {
    y_snapshot[i] = y[i];
  }

  {
    let p1: *mut i32 = &mut x[0];
    let q1: *mut i32 = &mut y[0];

    if unsafe { gate4 } != 0 {
      let p2: *mut i32 = p1.add(8);
      let q2: *mut i32 = q1.add(4);

      copy_restrict(16, p2, q2);

      let mut expect: [i32; 16] = [0; 16];
      for i in 0..16 {
        expect[i] = y_snapshot[4 + i];
      }
      if !same(&x[8..], &expect, 16) {
        std::process::exit(1);
      }

      if !same(&y[..], &y_snapshot[..], 32) {
        std::process::exit(2);
      }
    }

    copy_restrict(8, p1, q1.add(24));

    let mut expect2: [i32; 8] = [0; 8];
    for i in 0..8 {
      expect2[i] = y_snapshot[24 + i];
    }
    if !same(&x[..], &expect2, 8) {
      std::process::exit(3);
    }
  }

  if !same(&y[..], &y_snapshot[..], 32) {
    std::process::exit(4);
  }

  if sum(&x[..], 32) == 0 {
    std::process::exit(5);
  }
}