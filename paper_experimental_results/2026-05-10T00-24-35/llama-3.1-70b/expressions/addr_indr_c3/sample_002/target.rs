fn main() {
  let x = 7;
  let p = &x;

  if std::ptr::addr_of!(x) as *const _!= p as *const _ {
    std::process::exit(1);
  }

  if std::ptr::addr_of!(*p) as *const _!= p as *const _ {
    std::process::exit(2);
  }

  let a = [10, 20, 30];

  if std::ptr::addr_of!(a[0]) as *const _!= a.as_ptr().offset(0) {
    std::process::exit(3);
  }
  if std::ptr::addr_of!(a[1]) as *const _!= a.as_ptr().offset(1) {
    std::process::exit(4);
  }
  if std::ptr::addr_of!(a[2]) as *const _!= a.as_ptr().offset(2) {
    std::process::exit(5);
  }

  if unsafe { *a.as_ptr().offset(1) }!= 20 {
    std::process::exit(6);
  }

  std::process::exit(0);
}