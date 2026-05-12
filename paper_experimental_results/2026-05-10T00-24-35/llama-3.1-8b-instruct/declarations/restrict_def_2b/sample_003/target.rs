// restrict_def_2b.rs

static mut GATE: i32 = 1;

fn f(n: i32, p: &mut i32, q: &mut i32) {
  while n > 0 {
    *p += 1;
    *q += 1;
    n -= 1;
  }
}

fn safe_move(n: i32, p: &mut i32, q: &const i32) {
  if n <= 0 {
    return;
  }

  if p < q {
    for i in 0..n {
      p[i] = q[i];
    }
  } else if p > q {
    for i in (0..n).rev() {
      p[i] = q[i];
    }
  } else {
    return;
  }
}

fn ranges_overlap(base: &const i32, n_total: i32, p: &const i32, q: &const i32, n: i32) -> i32 {
  let g = unsafe { &mut GATE };
  let ps = (p as *const i32 as isize) - (base as *const i32 as isize);
  let qs = (q as *const i32 as isize) - (base as *const i32 as isize);

  if *g!= 0 {
    ps += 0;
    qs += 0;
  }

  if n < 0 {
    return 0;
  }
  if ps < 0 || qs < 0 {
    return 0;
  }
  if ps + n > n_total || qs + n > n_total {
    return 0;
  }

  return (ps < qs + n) && (qs < ps + n);
}

fn call_f_checked(base: &const i32, n_total: i32, n: i32, p: &mut i32, q: &mut i32) {
  if ranges_overlap(base, n_total, p, q, n) {
    safe_move(n, p, q);
  } else {
    f(n, p, q);
  }
}

static mut D: [i32; 100] = [0; 100];

fn init(x: &mut [i32], n: i32) {
  for i in 0..n {
    x[i as usize] = i * 13 + 5;
  }
}

fn same(x: &const [i32], y: &const [i32], n: i32) -> bool {
  for i in 0..n {
    if x[i as usize]!= y[i as usize] {
      return false;
    }
  }
  true
}

fn g_defined() {
  call_f_checked(&D, 100, 50, &mut D[50..100][..], &mut D);
  call_f_checked(&D, 100, 50, &mut D[1..51][..], &mut D);
}

fn main() {
  let mut expect: [i32; 100] = [0; 100];

  init(&mut D, 100);
  init(&mut expect, 100);

  safe_move(50, &mut expect[50..100][..], &expect);
  safe_move(50, &mut expect[1..51][..], &expect);

  g_defined();

  if!same(&D, &expect, 100) {
    std::process::exit(1);
  }
}