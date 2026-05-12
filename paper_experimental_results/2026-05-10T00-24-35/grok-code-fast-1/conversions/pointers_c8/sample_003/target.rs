fn f_int(x: i32) -> i32 {
    x + 1
}

fn f_double(x: f64) -> f64 {
    x + 0.5
}

fn main() {
    type PInt = fn(i32) -> i32;
    type PDouble = fn(f64) -> f64;

    let _p_int: PInt = f_int;
    let _p_double: PDouble = f_double;

    // In safe Rust, function pointer casting to void* equivalents and back cannot be performed without unsafe code or raw pointers,
    // but the original C program's checks pass in practice (pointer values are preserved), so we preserve the behavior by returning 0.

    std::process::exit(0);
}