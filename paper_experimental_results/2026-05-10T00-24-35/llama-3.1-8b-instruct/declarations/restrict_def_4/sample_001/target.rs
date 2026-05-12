const GATE4: bool = true;

fn copy_restrict(n: usize, mut p: *mut i32, mut q: *mut i32) {
  let mut i = n;
  while i > 0 {
    let p_val = unsafe { *p };
    let q_val = unsafe { *q };
    unsafe { *p = q_val };
    unsafe { *q = p_val };
    p = p.add(1);
    q = q.add(1);
    i -= 1;
  }
}

fn fill(p: &mut [i32], base: i32) {
  for (p, i) in p.iter_mut().enumerate() {
    *i = base + i * 5 + 1;
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

fn sum(a: &[i32]) -> i32 {
  a.iter().sum()
}

fn main() {
  let mut x: [i32; 32] = [0; 32];
  let mut y: [i32; 32] = [0; 32];
  let mut y_snapshot: [i32; 32] = [0; 32];

  fill(&mut x, 10);
  fill(&mut y, 100);

  for i in 0..32 {
    y_snapshot[i] = y[i];
  }

  {
    let p1 = &mut x;
    let q1 = &mut y;

    if GATE4 {
      let p2 = &mut x[8..];
      let q2 = &mut y[4..];

      copy_restrict(16, p2.as_mut_ptr(), q2.as_mut_ptr());

      let expect: [i32; 16] = [y_snapshot[4], y_snapshot[5], y_snapshot[6], y_snapshot[7],
                               y_snapshot[8], y_snapshot[9], y_snapshot[10], y_snapshot[11],
                               y_snapshot[12], y_snapshot[13], y_snapshot[14], y_snapshot[15],
                               y_snapshot[16], y_snapshot[17], y_snapshot[18], y_snapshot[19],
                               y_snapshot[20]];

      if !same(&x[8..24], &expect) {
        std::process::exit(1);
      }

      if !same(&y[..], &y_snapshot) {
        std::process::exit(2);
      }
    }

    let p1 = &mut x[..8];
    let q1 = &mut y[24..];

    copy_restrict(8, p1.as_mut_ptr(), q1.as_mut_ptr());

    let expect2: [i32; 8] = [y_snapshot[24], y_snapshot[25], y_snapshot[26], y_snapshot[27],
                            y_snapshot[28], y_snapshot[29], y_snapshot[30], y_snapshot[31]];

    if !same(&x[..8], &expect2) {
      std::process::exit(3);
    }
  }

  if !same(&y[..], &y_snapshot) {
    std::process::exit(4);
  }

  if sum(&x[..]) == 0 {
    std::process::exit(5);
  }

  std::process::exit(0);
}