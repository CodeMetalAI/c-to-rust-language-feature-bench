use std::mem::{align_of, size_of};
use std::process::exit;

fn type_id_int_ptr(_: *const i32) -> i32 {
    1
}
fn type_id_int_array_ptr(_: *const [i32; 3]) -> i32 {
    2
}
fn type_id_const_char_ptr(_: *const u8) -> i32 {
    3
}

fn main() {
    let a = [10i32, 20, 30];

    if type_id_int_ptr(a.as_ptr()) != 1 {
        exit(1);
    }
    if type_id_int_array_ptr(&a as *const [i32; 3]) != 2 {
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

    if size_of::<[i32; 3]>() != 3 * size_of::<i32>() {
        exit(6);
    }
    if align_of::<[i32; 3]>() != align_of::<i32>() {
        exit(7);
    }

    let s: [u8; 4] = [b'a', b'b', b'c', 0];

    if type_id_const_char_ptr(b"abc".as_ptr()) != 3 {
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
}