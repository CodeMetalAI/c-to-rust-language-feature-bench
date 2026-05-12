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

    let p_double2 = p_int as usize as fn(f64) -> f64;
    let p_int2 = p_double as usize as fn(i32) -> i32;

    let v1b = p_double2 as usize;
    let v2b = p_int2 as usize;

    if v1b != v1 {
        std::process::exit(1);
    }
    if v2b != v2 {
        std::process::exit(2);
    }

    let p_int3 = v1b as fn(i32) -> i32;
    let p_double3 = v2b as fn(f64) -> f64;

    if p_int3 as usize != p_int as usize {
        std::process::exit(3);
    }
    if p_double3 as usize != p_double as usize {
        std::process::exit(4);
    }

    std::process::exit(0);
}