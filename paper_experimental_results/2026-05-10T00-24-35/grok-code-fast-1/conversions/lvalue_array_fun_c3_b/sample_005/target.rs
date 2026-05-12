use std::mem;
use std::process;

fn main() {
    const TYPE_ID_A: i32 = 1;
    const TYPE_ID_REF_A: i32 = 2;
    const TYPE_ID_STR: i32 = 3;

    let a: [i32; 3] = [10, 20, 30];

    if TYPE_ID_A != 1 {
        process::exit(1);
    }
    if TYPE_ID_REF_A != 2 {
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

    if mem::size_of::<[i32; 3]>() != 3 * mem::size_of::<i32>() {
        process::exit(6);
    }
    if mem::align_of::<[i32; 3]>() != mem::align_of::<i32>() {
        process::exit(7);
    }

    let s: [u8; 4] = [b'a', b'b', b'c', 0];
    if TYPE_ID_STR != 3 {
        process::exit(8);
    }
    if s[0] != b'a' {
        process::exit(9);
    }
    if s[1] != b'b' {
        process::exit(10);
    }
    if s[2] != b'c' {
        process::exit(11);
    }
    if s[3] != 0 {
        process::exit(12);
    }
}