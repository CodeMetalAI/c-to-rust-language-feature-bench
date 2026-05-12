fn check<T: Copy>(n: T) {
  if n != 0 {
    panic!();
  }
}

fn main() {
  let mut n = 0;

  check(n + 1);
  check(n);

  check(n + 1);
  check(n);

  let cn = 0;
  check(cn);
  check(n + 1);
  check(n + 1);

  let a = [0];
  let ca = [0];
  check(&a);
  check(n + 1);

  check(&ca);
  check(n + 1);

  let f = || {};
  check(f);
  check(n + 1);

  check(std::ptr::read(&abort));
  check(n + 1);

  let s: i16 = 0;
  check(s as i32);
  check(n + 1);

  std::process::exit(0);
}

fn abort() {
  panic!();
}