fn main() {
  let x: i32 = 17;
  let p: *const i32 = &x as *const i32;

  let pc: *const i32 = p;
  let pv: *mut i32 = p as *mut i32;
  let pcv: *const mut i32 = p as *const mut i32;

  if pc as *const _ != p as *const _ {
    return 1;
  }
  if pv as *const _ != p as *const _ {
    return 2;
  }
  if pcv as *const _ != p as *const _ {
    return 3;
  }

  if unsafe { *pc } != 17 {
    return 4;
  }
  if unsafe { *pv } != 17 {
    return 5;
  }
  if unsafe { *pcv } != 17 {
    return 6;
  }

  std::process::exit(0);
}