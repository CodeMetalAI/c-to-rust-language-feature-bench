use std::process;

trait TypeIdLike {
    fn type_id(self) -> i32;
}

impl<'a> TypeIdLike for &'a [i32] {
    fn type_id(self) -> i32 {
        1
    }
}

impl<'a> TypeIdLike for &'a [i32; 3] {
    fn type_id(self) -> i32 {
        2
    }
}

impl<'a> TypeIdLike for &'a str {
    fn type_id(self) -> i32 {
        3
    }
}

fn type_id<T: TypeIdLike>(x: T) -> i32 {
    x.type_id()
}

fn exit_with(code: i32) -> ! {
    process::exit(code);
}

fn main() {
    let a = [10i32, 20, 30];

    if type_id(a.as_slice()) != 1 {
        exit_with(1);
    }
    if type_id(&a) != 2 {
        exit_with(2);
    }

    if a[0] != 10 {
        exit_with(3);
    }
    if a[1] != 20 {
        exit_with(4);
    }
    if a[2] != 30 {
        exit_with(5);
    }

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        exit_with(6);
    }
    if std::mem::align_of_val(&a) != std::mem::align_of::<i32>() {
        exit_with(7);
    }

    let s: [u8; 4] = *b"abc\0";
    if type_id("abc") != 3 {
        exit_with(8);
    }
    if s[0] != b'a' {
        exit_with(9);
    }
    if s[1] != b'b' {
        exit_with(10);
    }
    if s[2] != b'c' {
        exit_with(11);
    }
    if s[3] != 0 {
        exit_with(12);
    }
}