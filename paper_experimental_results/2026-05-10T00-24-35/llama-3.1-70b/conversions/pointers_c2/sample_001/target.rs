fn main() {
  let x: i32 = 17;
  let p: &i32 = &x;

  let pc: &i32 = p;
  let pv: &i32 = p;
  let pcv: &i32 = p;

  if (pc as *const _) as *const () != p as *const _ as *const () {
    return;
  }
  if (pv as *const _) as *const () != p as *const _ as *const () {
    return;
  }
  if (pcv as *const _) as *const () != p as *const _ as *const () {
    return;
  }

  if *pc != 17 {
    return;
  }
  if *pv != 17 {
    return;
  }
  if *pcv != 17 {
    return;
  }

  println!(""); // In Rust, the process exit code cannot be set directly.
               // Instead, the program can terminate normally by reaching the end of `main`.
}