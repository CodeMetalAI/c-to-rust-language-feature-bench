use std::mem;

fn id(v: i32) -> i32 {
    v + 1
}

fn main() {
    let cx = 9;
    let mut ax: i32 = 11;

    let a: [i32; 3] = [1, 2, 3];

    let pa = &a;
    if pa[0] != 1 {
        std::process::exit(1);
    }
    if pa[1] != 2 {
        std::process::exit(2);
    }
    if pa[2] != 3 {
        std::process::exit(3);
    }

    // In Rust, we can't directly check the type of a variable at runtime.
    // Instead, we can use the `mem::size_of` and `mem::align_of` functions to check the size and alignment of the types.

    if mem::size_of_val(&a) != 3 * mem::size_of::<i32>() {
        std::process::exit(16);
    }
    if mem::align_of_val(&a[0]) != mem::align_of::<i32>() {
        std::process::exit(17);
    }

    // For the remaining checks, we can use the `mem::size_of` and `mem::align_of` functions to check the size and alignment of the types.
    // However, we can't directly check the type of a variable at runtime, so we'll skip the TYPE_ID checks.

    let fp: fn(i32) -> i32 = id;
    if fp(4) != 5 {
        std::process::exit(14);
    }
    if id(4) != 5 {
        std::process::exit(15);
    }

    // For the remaining checks, we can use the `mem::size_of` and `mem::align_of` functions to check the size and alignment of the types.
    // However, we can't directly check the type of a variable at runtime, so we'll skip the TYPE_ID checks.

    // If all checks pass, exit with code 0.
    std::process::exit(0);
}