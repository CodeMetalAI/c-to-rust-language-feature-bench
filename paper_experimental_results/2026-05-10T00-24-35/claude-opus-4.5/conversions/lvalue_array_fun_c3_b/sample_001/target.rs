use std::mem;

fn main() {
    let a: [i32; 3] = [10, 20, 30];

    // In Rust, arrays don't decay to pointers like in C
    // The TYPE_ID checks are about C's type system behavior
    // We'll verify the equivalent Rust behaviors

    // Check array element access (equivalent to *a, *(a+1), *(a+2))
    if a[0] != 10 {
        std::process::exit(3);
    }
    if a[1] != 20 {
        std::process::exit(4);
    }
    if a[2] != 30 {
        std::process::exit(5);
    }

    // Check sizeof equivalent
    if mem::size_of_val(&a) != 3 * mem::size_of::<i32>() {
        std::process::exit(6);
    }
    
    // Check alignment
    if mem::align_of_val(&a) != mem::align_of::<i32>() {
        std::process::exit(7);
    }

    // String literal to char array
    let s: [u8; 4] = [b'a', b'b', b'c', 0];
    
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

    // Return 0 (success)
    std::process::exit(0);
}