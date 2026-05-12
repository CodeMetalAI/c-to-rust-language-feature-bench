fn f_int(x: i32) -> i32 {
    x + 1
}

fn f_double(x: f64) -> f64 {
    x + 0.5
}

fn main() {
    let p_int: fn(i32) -> i32 = f_int;
    let p_double: fn(f64) -> f64 = f_double;

    let v1: usize = p_int as usize;
    let v2: usize = p_double as usize;

    let p_double2: usize = v1;
    let p_int2: usize = v2;

    let v1b: usize = p_double2;
    let v2b: usize = p_int2;

    if v1b != v1 {
        std::process::exit(1);
    }
    if v2b != v2 {
        std::process::exit(2);
    }

    let p_int3: usize = v1b;
    let p_double3: usize = v2b;

    let p_int_addr = p_int as usize;
    let p_double_addr = p_double as usize;

    if p_int3 != p_int_addr {
        std::process::exit(3);
    }
    if p_double3 != p_double_addr {
        std::process::exit(4);
    }

    std::process::exit(0);
}