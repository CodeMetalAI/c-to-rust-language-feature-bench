fn main() {
    let a = [10, 20, 30];

    if type_id(&a) != 2 {
        std::process::exit(1);
    }

    if type_id(&[&a]) != 2 {
        std::process::exit(2);
    }

    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    if std::mem::size_of(&a) != 3 * std::mem::size_of(0) {
        std::process::exit(6);
    }
    if std::mem::align_of(&a) != std::mem::align_of(0) {
        std::process::exit(7);
    }

    let s = "abc".as_bytes();
    if type_id(s) != 3 {
        std::process::exit(8);
    }
    if s[0] != b'a' {
        std::process::exit(9);
    }
    if s[1] != b'b' {
        std::process::exit(10);
    }
    if s[2] != b'c' {
        std::process::exit(11);
    }
    if s[3] != 0 {
        std::process::exit(12);
    }

    std::process::exit(0);
}

fn type_id(x: &dyn Any) -> i32 {
    use std::any::Any;
    match x.type_id() {
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        _ => 99,
    }
}

use std::any;
trait Any {
    fn type_id(&self) -> i32;
}

impl Any for [i32; 3] {
    fn type_id(&self) -> i32 {
        2
    }
}

impl Any for &[i32; 3] {
    fn type_id(&self) -> i32 {
        2
    }
}


impl Any for &[u8] {
    fn type_id(&self) -> i32 {
        3
    }
}