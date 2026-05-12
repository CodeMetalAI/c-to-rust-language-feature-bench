use std::mem;
use std::process;

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    if a[0] != 10 {
        process::exit(3);
    }
    if a[1] != 20 {
        process::exit(4);
    }
    if a[2] != 30 {
        process::exit(5);
    }

    if mem::size_of_val(&a) != 3 * mem::size_of::<i32>() {
        process::exit(6);
    }
    if mem::align_of_val(&a) != mem::align_of::<i32>() {
        process::exit(7);
    }

    let s: [u8; 4] = [b'a', b'b', b'c', b'\0'];
    if s[0] != b'a' {
        process::exit(9);
    }
    if s[1] != b'b' {
        process::exit(10);
    }
    if s[2] != b'c' {
        process::exit(11);
    }
    if s[3] != b'\0' {
        process::exit(12);
    }

    process::exit(0);
}