fn main() {
  let x: i32 = 42;
  let p: *const i32 = &x;

  if p as *const _!= &x as *const _ {
    std::process::exit(1);
  }

  let p_end1: *const i32 = unsafe { p.offset(1) };
  let p_end2: *const i32 = unsafe { &x as *const _ as *const i32 }.offset(1);

  if p_end1 as *const _!= p_end2 as *const _ {
    std::process::exit(2);
  }

  if p_end1 as *const _ == p as *const _ {
    std::process::exit(3);
  }

  std::process::exit(0);
}