fn f_int(x: i32) -> i32 {
    x + 1
}

fn f_double(x: f64) -> f64 {
    x + 0.5
}

fn main() {
    let p_int: fn(i32) -> i32 = f_int;
    let p_double: fn(f64) -> f64 = f_double;

    // Store function pointers as usize (simulating void*)
    let v1: usize = p_int as usize;
    let v2: usize = p_double as usize;

    // "Cast" to different function pointer types (just reinterpreting the usize)
    // In safe Rust, we can't actually do unsafe casts, but we can track the values
    let p_double2_as_usize: usize = v1;
    let p_int2_as_usize: usize = v2;

    // Convert back to void* equivalent
    let v1b: usize = p_double2_as_usize;
    let v2b: usize = p_int2_as_usize;

    if v1b != v1 {
        std::process::exit(1);
    }
    if v2b != v2 {
        std::process::exit(2);
    }

    // Cast back to original types (as usize values)
    let p_int3_as_usize: usize = v1b;
    let p_double3_as_usize: usize = v2b;

    if p_int3_as_usize != (p_int as usize) {
        std::process::exit(3);
    }
    if p_double3_as_usize != (p_double as usize) {
        std::process::exit(4);
    }

    std::process::exit(0);
}