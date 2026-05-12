// type_def_3.rs
enum Tag {
  T(u32),
  Const(i32),
  R(i32),
}

struct Callback<T> {
  cb: fn(T) -> T,
}

impl Callback<i32> {
  fn call(&self, x: i32) -> i32 {
    (self.cb)(x)
  }
}

fn id(x: i32) -> i32 {
  x
}

fn call_cb<T>(cb: Callback<T>, x: T) -> T {
  cb.call(x)
}

fn use_bits(s: Tag) -> i32 {
  match s {
    Tag::T(a) => a as i32,
    Tag::Const(_) => 0,
    Tag::R(b) => b as i32,
  } * 100 + match s {
    Tag::T(_) => 0,
    Tag::Const(_) => 0,
    Tag::R(b) => b as i32,
  }
}

fn f(cb: impl Fn(i32) -> i32) -> i32 {
  call_cb(Callback { cb }, 9)
}

fn main() {
  let s = Tag::T(15);
  s.r = 31;

  if use_bits(s) != 15 * 100 + 31 {
    println!("{}", 1);
    std::process::exit(1);
  }

  {
    let t: i32 = 1234;

    if t != 1234 {
      println!("{}", 2);
      std::process::exit(2);
    }

    if call_cb(Callback { cb: id }, 7) != 7 {
      println!("{}", 3);
      std::process::exit(3);
    }

    if f(id) != 9 {
      println!("{}", 4);
      std::process::exit(4);
    }
  }

  println!("{}", 0);
}