fn main() {
  let x = 17;
  let p = &x as *const i32;

  let pc: *const i32 = p;
  let pv: *const i32 = p;
  let pcv: *const i32 = p;

  if pc as *const ()!= p as *const () {
    std::process::exit(1);
  }
  if pv as *const ()!= p as *const () {
    std::process::exit(2);
  }
  if pcv as *const ()!= p as *const () {
    std::process::exit(3);
  }

  if unsafe { *pc }!= 17 {
    std::process::exit(4);
  }
  if unsafe { *pv }!= 17 {
    std::process::exit(5);
  }
  if unsafe { *pcv }!= 17 {
    std::process::exit(6);
  }

  std::process::exit(0);
}