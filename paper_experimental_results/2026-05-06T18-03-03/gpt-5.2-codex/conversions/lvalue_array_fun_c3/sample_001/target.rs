use std::mem::{align_of, align_of_val, size_of, size_of_val};
use std::process::exit;

trait TypeId {
    fn id(&self) -> i32;
}

impl TypeId for [i32; 3] {
    fn id(&self) -> i32 {
        1
    }
}

impl<'a> TypeId for &'a [i32; 3] {
    fn id(&self) -> i32 {
        2
    }
}

impl<'a> TypeId for &'a str {
    fn id(&self) -> i32 {
        3
    }
}

fn type_id<T: TypeId>(x: T) -> i32 {
    x.id()
}

fn main() {
    let a = [10i32, 20, 30];

    if type_id(a) != 1 {
        exit(1);
    }
    if type_id(&a) != 2 {
        exit(2);
    }

    if a[0] != 10 {
        exit(3);
    }
    if a[1] != 20 {
        exit(4);
    }
    if a[2] != 30 {
        exit(5);
    }

    if size_of_val(&a) != 3 * size_of::<i32>() {
        exit(6);
    }
    if align_of_val(&a) != align_of::<i32>() {
        exit(7);
    }

    let s: [u8; 4] = *b"abc\0";
    if type_id("abc") != 3 {
        exit(8);
    }
    if s[0] != b'a' {
        exit(9);
    }
    if s[1] != b'b' {
        exit(10);
    }
    if s[2] != b'c' {
        exit(11);
    }
    if s[3] != 0 {
        exit(12);
    }

    exit(0);
}