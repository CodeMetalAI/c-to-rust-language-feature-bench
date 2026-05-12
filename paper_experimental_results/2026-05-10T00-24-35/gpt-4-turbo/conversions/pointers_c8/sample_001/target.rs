fn f_int(x: i32) -> i32 {
    x + 1
}

fn f_double(x: f64) -> f64 {
    x + 0.5
}

fn main() {
    // Function pointers in Rust
    let p_int: fn(i32) -> i32 = f_int;
    let p_double: fn(f64) -> f64 = f_double;

    // Casting function pointers to void pointers, without using `void` which doesn't exist in Rust
    let v1 = p_int as *const ();
    let v2 = p_double as *const ();

    // Recasting back to function pointers
    let p_double2 = unsafe { std::mem::transmute::<*const (), fn(f64) -> f64>(v1) };
    let p_int2 = unsafe { std::mem::transmute::<*const (), fn(i32) -> i32>(v2) };

    // Recasting to void pointers again
    let v1b = p_double2 as *const ();
    let v2b = p_int2 as *const ();

    // Checking the pointer equality
    if v1b != v1 {
        std::process::exit(1);
    }
    if v2b != v2 {
        std::process::exit(2);
    }

    // Recasting back to function pointers from void pointers
    let p_int3 = unsafe { std::mem::transmute::<*const (), fn(i32) -> i32>(v1b) };
    let p_double3 = unsafe { std::mem::transmute::<*const (), fn(f64) -> f64>(v2b) };

    // Final check of pointer equality
    if p_int3 as *const () != p_int as *const () {
        std::process::exit(3);
    }
    if p_double3 as *const () != p_double as *const () {
        std::process::exit(4);
    }
}