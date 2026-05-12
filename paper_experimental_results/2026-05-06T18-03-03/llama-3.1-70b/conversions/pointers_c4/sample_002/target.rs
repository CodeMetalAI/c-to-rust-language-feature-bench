fn f(x: i32) -> i32 { x }

fn main() {
  let p0: *const i32 = std::ptr::null();
  let v0: *const () = std::ptr::null();
  let p1: *const i32 = v0 as *const i32;

  let fp0: fn(i32) -> i32 = std::ptr::null();
  let fp1: fn(i32) -> i32 = std::ptr::null();
  let fp2: fn(i32) -> i32 = v0 as fn(i32) -> i32;

  if p0 as *const ()!= p1 as *const () {
    return;
  }
  if p0 as *const ()!= std::ptr::null() {
    return;
  }

  if v0 as *const ()!= std::ptr::null() {
    return;
  }
  if v0 as *const ()!= p0 as *const () {
    return;
  }

  if fp0 as *const ()!= fp1 as *const () {
    return;
  }
  if fp1 as *const ()!= fp2 as *const () {
    return;
  }
  if fp0 as *const ()!= std::ptr::null() {
    return;
  }

  if p0 as *const ()!= fp0 as *const () {
    return;
  }
}