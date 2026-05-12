fn f_int(x: i32) -> i32 {
    x + 1
}

fn f_double(x: f64) -> f64 {
    x + 0.5
}

fn main() {
    let p_int: fn(i32) -> i32 = f_int;
    let p_double: fn(f64) -> f64 = f_double;

    let v1 = p_int as usize;
    let v2 = p_double as usize;

    let p_double2_addr = v1;
    let p_int2_addr = v2;

    let v1b = p_double2_addr;
    let v2b = p_int2_addr;

    if v1b != v1 {
        std::process::exit(1);
    }
    if v2b != v2 {
        std::process::exit(2);
    }

    let p_int3_addr = v1b;
    let p_double3_addr = v2b;

    if p_int3_addr != p_int as usize {
        std::process::exit(3);
    }
    if p_double3_addr != p_double as usize {
        std::process::exit(4);
    }
}