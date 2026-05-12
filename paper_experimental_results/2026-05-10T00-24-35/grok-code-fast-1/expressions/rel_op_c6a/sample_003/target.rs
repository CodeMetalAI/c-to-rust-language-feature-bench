use std::process;

fn main() {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 2;

    let temp: bool = a < b;
    let temp_int: i32 = if temp { 1 } else { 0 };
    let chained: bool = temp_int < c;
    if !chained {
        process::exit(1);
    }

    let temp2: bool = a < b;
    let temp_int2: i32 = if temp2 { 1 } else { 0 };
    let chained2: bool = temp_int2 < c;
    if !chained2 {
        process::exit(2);
    }

    a = 3;
    b = 2;
    c = 2;

    let temp3: bool = a < b;
    let temp_int3: i32 = if temp3 { 1 } else { 0 };
    let chained3: bool = temp_int3 < c;
    if !chained3 {
        process::exit(3);
    }

    process::exit(0);
}