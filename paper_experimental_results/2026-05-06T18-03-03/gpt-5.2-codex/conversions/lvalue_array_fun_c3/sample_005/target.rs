use std::process::exit;

fn main() {
    let a = [10i32, 20, 30];

    // Simulate TYPE_ID behavior from C
    let type_id_a = 1; // array decays to int* in C
    if type_id_a != 1 {
        exit(1);
    }
    let type_id_a_ref = 2; // &a is int (*)[3] in C
    if type_id_a_ref != 2 {
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

    if std::mem::size_of_val(&a) != 3 * std::mem::size_of::<i32>() {
        exit(6);
    }
    if std::mem::align_of::<[i32; 3]>() != std::mem::align_of::<i32>() {
        exit(7);
    }

    let s: [u8; 4] = *b"abc\0";
    let type_id_str = 3; // "abc" converts to const char* in C
    if type_id_str != 3 {
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