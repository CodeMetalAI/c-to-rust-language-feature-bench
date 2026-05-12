fn f(x: i32) -> i32 { x }

fn main() {
  let p0: *const i32 = std::ptr::null();
  let v0: *const () = std::ptr::null();
  let p1: *const i32 = v0 as *const i32;

  let fp0: fn(i32) -> i32 = std::ptr::null();
  let fp1: fn(i32) -> i32 = std::ptr::null();
  let fp2: fn(i32) -> i32 = v0 as fn(i32) -> i32;

  if p0 as usize!= p1 as usize {
    std::process::exit(1);
  }
  if p0 as usize!= 0 {
    std::process::exit(2);
  }

  if v0 as usize!= 0 {
    std::process::exit(3);
  }
  if v0 as usize!= p0 as usize {
    std::process::exit(4);
  }

  if fp0 as usize!= fp1 as usize {
    std::process::exit(5);
  }
  if fp1 as usize!= fp2 as usize {
    std::process::exit(6);
  }
  if fp0 as usize!= 0 {
    std::process::exit(7);
  }

  if p0 as usize!= fp0 as usize {
    std::process::exit(8);
  }

  std::process::exit(0);
}