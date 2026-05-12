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

    // "Cast" to different function pointer types (just reinterpret the usize)
    // In the original C, this is UB but we're preserving the pointer value
    let p_double2: usize = v1;
    let p_int2: usize = v2;

    // Cast back to void* equivalent
    let v1b: usize = p_double2;
    let v2b: usize = p_int2;

    if v1b != v1 {
        std::process::exit(1);
    }
    if v2b != v2 {
        std::process::exit(2);
    }

    // Cast again
    let p_int3: usize = v1b;
    let p_double3: usize = v2b;

    if p_int3 != p_int as usize {
        std::process::exit(3);
    }
    if p_double3 != p_double as usize {
        std::process::exit(4);
    }

    std::process::exit(0);
}