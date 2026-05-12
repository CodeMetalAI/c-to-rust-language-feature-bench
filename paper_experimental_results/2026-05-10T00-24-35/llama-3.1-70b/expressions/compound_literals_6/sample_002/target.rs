fn main() {
  let a = [b'a', b'b', b'c', b'\0'];
  let b = "abc".as_bytes();

  if a[0]!= b'a' || a[1]!= b'b' || a[2]!= b'c' || a[3]!= b'\0' {
    std::process::exit(1);
  }
  if b[0]!= b'a' || b[1]!= b'b' || b[2]!= b'c' || b[3]!= b'\0' {
    std::process::exit(2);
  }

  if (a as *const _ as usize == b.as_ptr() as usize)!= 0 && (a as *const _ as usize == b.as_ptr() as usize)!= 1 {
    std::process::exit(3);
  }

  std::process::exit(0);
}