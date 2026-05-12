use std::any::TypeId;
use std::mem::{align_of, align_of_val, size_of, size_of_val};
use std::process::exit;

fn type_id<T: 'static>(_: &T) -> i32 {
    let tid = TypeId::of::<T>();
    if tid == TypeId::of::<i32>() {
        1
    } else if tid == TypeId::of::<[i32; 3]>() {
        2
    } else if tid == TypeId::of::<u8>() {
        3
    } else if tid == TypeId::of::<[u8; 4]>() {
        4
    } else {
        99
    }
}

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if type_id(&a[0]) != 1 {
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
    if type_id(&b"abc\0"[0]) != 3 {
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