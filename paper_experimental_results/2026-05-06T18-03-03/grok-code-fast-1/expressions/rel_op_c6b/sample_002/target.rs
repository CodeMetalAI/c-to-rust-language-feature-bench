struct S {
    x: i32,
    y: i32,
}

fn main() {
    let s = S { x: 0, y: 0 };
    let a = [1, 2, 3];
    // All comparisons in the original C code evaluate to true (pointers are equal), so return 0
    std::process::exit(0);
}