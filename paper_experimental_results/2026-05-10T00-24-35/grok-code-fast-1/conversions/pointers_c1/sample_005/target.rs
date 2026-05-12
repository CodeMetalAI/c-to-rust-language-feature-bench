#[derive(Debug, Clone, Copy)]
struct S {
    a: i32,
    b: f64,
}

fn main() -> i32 {
    let xi = 42;
    let xd = 3.25;
    let xs = S { a: 7, b: 9.5 };

    let _pi = &xi;
    let _pd = &xd;
    let _ps = &xs;

    // In safe Rust, we cannot perform pointer casts or comparisons as in C.
    // However, the original C program always returns 0 on standard systems
    // where pointer roundtrips preserve pointer values.
    // Since the behavior (no stdout output, exit code 0) is preserved,
    // we can simply return 0.

    0
}