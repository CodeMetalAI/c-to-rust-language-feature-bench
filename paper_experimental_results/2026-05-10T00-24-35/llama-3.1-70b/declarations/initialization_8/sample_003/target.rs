fn main() {
  let mut s = [0; 4];
  let s_ptr = s.as_mut_ptr() as *mut u8;
  unsafe { std::ptr::copy("abc\0".as_ptr(), s_ptr, 4) };

  let t = [b'a', b'b', b'c'];

  if s[0]!= b'a' || s[1]!= b'b' || s[2]!= b'c' || s[3]!= b'\0' {
    return;
  }

  if t[0]!= b'a' || t[1]!= b'b' || t[2]!= b'c' {
    std::process::exit(2);
  }

  unsafe { *s_ptr = b'x'; }
  t[2] = b'y';

  if s[0]!= b'x' {
    std::process::exit(3);
  }
  if t[2]!= b'y' {
    std::process::exit(4);
  }

  let p = b"abc";
  if p[0]!= b'a' || p[1]!= b'b' || p[2]!= b'c' || p[3]!= b'\0' {
    std::process::exit(5);
  }

  std::process::exit(0);
}