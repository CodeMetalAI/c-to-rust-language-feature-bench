fn f() -> i32 { -1 }

fn main() {
  let c: i8 = f();

  let eq = c == -1;

  let char_is_signed = (c as i8) < 0;

  if char_is_signed {
    if eq!= 1 {
      std::process::exit(1);
    }
  } else {
    if eq!= 0 {
      std::process::exit(2);
    }
  }
}