fn main() {
  let x = 7;
  let p = &x;

  if &x as *const _!= p as *const _ {
    return;
  }

  if &(*p) as *const _!= p as *const _ {
    return;
  }

  let a = [10, 20, 30];

  if &a[0] as *const _!= a.as_ptr() {
    return;
  }
  if &a[1] as *const _!= a.as_ptr().offset(1) {
    return;
  }
  if &a[2] as *const _!= a.as_ptr().offset(2) {
    return;
  }

  if unsafe { *a.as_ptr().offset(1) }!= 20 {
    return;
  }

  std::process::exit(0);
}