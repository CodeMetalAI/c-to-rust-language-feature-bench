fn main() {
  let x: i32 = 42;
  let p = &x as *const i32;

  if p as usize!= &x as *const i32 as usize {
    std::process::exit(1);
  }

  let p_end1 = p.offset(1);
  let p_end2 = &x as *const i32 as *const i32;

  if p_end1 as usize!= p_end2.offset(1) as usize {
    std::process::exit(2);
  }

  if p_end1 as usize == p as usize {
    std::process::exit(3);
  }

  std::process::exit(0);
}