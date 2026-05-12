#[derive(Clone, Copy)]
struct S {
    a: i32,
    b: f64,
}

fn main() {
    let xi = 42;
    let xd = 3.25;
    let xs = S { a: 7, b: 9.5 };

    // In safe Rust, pointer manipulations as in C cannot be performed.
    // The original C program always returns 0, so we preserve that behavior.
}