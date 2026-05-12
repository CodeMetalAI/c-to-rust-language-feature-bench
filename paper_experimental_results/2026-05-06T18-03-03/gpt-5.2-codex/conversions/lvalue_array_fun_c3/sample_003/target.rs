use std::mem::{align_of, size_of};
use std::process::exit;

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    let type_id_a = 1;
    if type_id_a != 1 {
        exit(1);
    }
    let type_id_refa = 2;
    if type_id_refa != 2 {
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

    let s: [u8; 4] = *b"abc\0";
    let type_id_abc = 3;
    if type_id_abc != 3 {
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