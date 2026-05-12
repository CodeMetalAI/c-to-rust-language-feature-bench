static OFF: i32 = 9;

fn f(n: i32, p: &mut i32, q: &mut i32) {
  while n > 0 {
    *p = *q;
    p = p.offset(1);
    q = q.offset(1);
    n -= 1;
  }
}

fn check_range_eq(a: &[i32], b: &[i32], n: usize) -> bool {
  let mut i = 0;
  while i < n {
    if a[i] != b[i] {
      return false;
    }
    i += 1;
  }
  true
}

fn main() {
  let mut buf = [0; 20];
  let mut snapshot = [0; 20];

  for i in 0..20 {
    buf[i] = i as i32 * 11 + 3;
    snapshot[i] = buf[i];
  }

  {
    let off = OFF;
    let n = 8;

    let mut p = buf.as_mut_ptr() as *mut i32;
    let q = (buf.as_ptr() as *mut i32).offset(off as isize);

    f(n, p, q);

    if !check_range_eq(&buf[..n], &snapshot[off..(off + n)]) {
      panic!("first range not equal");
    }

    if !check_range_eq(&buf[off..(off + n)], &snapshot[off..(off + n)]) {
      panic!("second range not equal");
    }

    if buf[off as usize - 1] != snapshot[off as usize - 1] {
      panic!("off - 1 not equal");
    }

    if buf[off as usize + n as usize] != snapshot[off as usize + n as usize] {
      panic!("off + n not equal");
    }
  }

  {
    let off = OFF;
    let n = 8;

    let mut p = (buf.as_ptr() as *mut i32).offset(off as isize);
    let q = buf.as_mut_ptr() as *mut i32;

    f(n, p, q);

    if !check_range_eq(&buf[off..(off + n)], &buf[..n]) {
      panic!("range not equal");
    }
  }
}