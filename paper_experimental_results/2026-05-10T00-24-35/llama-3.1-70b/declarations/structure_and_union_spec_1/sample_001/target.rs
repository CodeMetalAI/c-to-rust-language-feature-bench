fn main() {
  #[repr(C)]
  struct V {
    i: i32,
    w_k: i64,
    _padding: [u8; 4],
    m: i32,
  }

  let mut v1 = V { i: 2, w_k: 5, _padding: [0; 4], m: 0 };

  if v1.i!= 2 {
    std::process::exit(1);
  }

  if v1.w_k!= 5 {
    std::process::exit(1);
  }

  std::process::exit(0);
}