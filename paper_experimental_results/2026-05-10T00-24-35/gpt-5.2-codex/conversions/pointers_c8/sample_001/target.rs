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

    let p_double2: fn(f64) -> f64 = v1 as fn(f64) -> f64;
    let p_int2: fn(i32) -> i32 = v2 as fn(i32) -> i32;

    let v1b: usize = p_double2 as usize;
    let v2b: usize = p_int2 as usize;

    let mut code = 0;
    if v1b != v1 {
        code = 1;
    } else if v2b != v2 {
        code = 2;
    } else {
        let p_int3: fn(i32) -> i32 = v1b as fn(i32) -> i32;
        let p_double3: fn(f64) -> f64 = v2b as fn(f64) -> f64;

        if (p_int3 as usize) != (p_int as usize) {
            code = 3;
        } else if (p_double3 as usize) != (p_double as usize) {
            code = 4;
        }
    }

    std::process::exit(code);
}