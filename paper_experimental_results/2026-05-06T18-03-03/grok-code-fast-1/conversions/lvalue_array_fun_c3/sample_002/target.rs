use std::any::TypeId;
use std::mem;
use std::process;

fn type_id<T: 'static>(_: &T) -> i32 {
    let tid = TypeId::of::<T>();
    if tid == TypeId::of::<&[i32; 3]>() {
        1
    } else if tid == TypeId::of::<&&[i32; 3]>() {
        2
    } else if tid == TypeId::of::<&str>() {
        3
    } else if tid == TypeId::of::<&[i8; 4]>() {
        4
    } else {
        99
    }
}

fn main() {
    let a: &[i32; 3] = &[10, 20, 30];

    if type_id(a) != 1 {
        process::exit(1);
    }
    if type_id(&a) != 2 {
        process::exit(2);
    }

    if a[0] != 10 {
        process::exit(3);
    }
    if a[1] != 20 {
        process::exit(4);
    }
    if a[2] != 30 {
        process::exit(5);
    }

    if mem::size_of_val(a) != 3 * mem::size_of::<i32>() {
        process::exit(6);
    }
    if mem::align_of::<[i32; 3]>() != mem::align_of::<i32>() {
        process::exit(7);
    }

    let s: [i8; 4] = [b'a' as i8, b'b' as i8, b'c' as i8, 0];
    if type_id(&"abc") != 3 {
        process::exit(8);
    }
    if s[0] != b'a' as i8 {
        process::exit(9);
    }
    if s[1] != b'b' as i8 {
        process::exit(10);
    }
    if s[2] != b'c' as i8 {
        process::exit(11);
    }
    if s[3] != 0 {
        process::exit(12);
    }
}