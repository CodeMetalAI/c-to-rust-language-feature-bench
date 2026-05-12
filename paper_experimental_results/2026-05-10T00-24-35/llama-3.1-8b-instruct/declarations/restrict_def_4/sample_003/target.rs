// restrict_def_4.rs
static mut GATE4: i32 = 1;

fn copy_restrict(n: usize, p: &mut i32, q: &mut i32) {
  while n > 0 {
    *p = *q;
    p = p.offset(1);
    q = q.offset(1);
    n -= 1;
  }
}

fn fill(p: &mut [i32], n: usize, base: i32) {
  let mut i = 0;
  while i < n {
    p[i] = base + i * 5 + 1;
    i += 1;
  }
}

fn same(a: &[i32], b: &[i32], n: usize) -> bool {
  let mut i = 0;
  while i < n {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

fn sum(a: &[i32], n: usize) -> i32 {
  let mut s = 0;
  let mut i = 0;
  while i < n {
    s += a[i];
    i += 1;
  }
  s
}

fn main() {
  let mut x = [0; 32];
  let mut y = [0; 32];
  let mut y_snapshot = [0; 32];

  fill(&mut x, 32, 10);
  fill(&mut y, 32, 100);

  {
    let mut i = 0;
    while i < 32 {
      y_snapshot[i] = y[i];
      i += 1;
    }
  }

  {
    let mut p1 = &mut x;
    let mut q1 = &mut y;

    if GATE4 != 0 {
      let mut p2 = p1;
      let mut q2 = q1;

      copy_restrict(16, p2.offset(8), q2.offset(4));

      {
        let mut expect = [0; 16];
        let mut i = 0;
        while i < 16 {
          expect[i] = y_snapshot[4 + i];
          i += 1;
        }
        if !same(&x[8..24], &expect) {
          eprintln!("Error: unexpected value");
          std::process::exit(1);
        }
      }

      if !same(&y[..], &y_snapshot) {
        eprintln!("Error: unexpected value");
        std::process::exit(2);
      }
    }

    copy_restrict(8, p1, q1.offset(24));

    {
      let mut expect2 = [0; 8];
      let mut i = 0;
      while i < 8 {
        expect2[i] = y_snapshot[24 + i];
        i += 1;
      }
      if !same(&x[0..8], &expect2) {
        eprintln!("Error: unexpected value");
        std::process::exit(3);
      }
    }
  }

  if !same(&y[..], &y_snapshot) {
    eprintln!("Error: unexpected value");
    std::process::exit(4);
  }

  if sum(&x[..], 32) != 0 {
    eprintln!("Error: unexpected value");
    std::process::exit(5);
  }

  std::process::exit(0);
}