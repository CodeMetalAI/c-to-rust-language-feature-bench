static mut I: [i32] = [0];

fn main() {
  if unsafe { I[0] }!= 0 {
    std::process::exit(1);
  }
  unsafe { I[0] = 7; }
  if unsafe { I[0] }!= 7 {
    std::process::exit(2);
  }
  std::process::exit(0);
}