static mut I: [i32] = [0; 1];

fn main() {
  unsafe {
    if I[0]!= 0 {
      std::process::exit(1);
    }
    I[0] = 7;
    if I[0]!= 7 {
      std::process::exit(2);
    }
  }
  std::process::exit(0);
}