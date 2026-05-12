fn check_type(x: i8) -> u8 {
    match x {
        _ if x >= 0 => 1,
        _ if x == -1i8 => 99,
        _ => 2,
    }
}

fn main() {
    let pc = -1 as i8;
    let sc = -1 as i8;
    let uc = 255 as u8;

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

    if check_type(pc as i8 + 0) != 1 {
        println!("pc type check failed");
        return;
    }
    if check_type(sc as i8 + 0) != 1 {
        println!("sc type check failed");
        return;
    }
    if check_type(uc as i8 + 0) != 1 {
        println!("uc type check failed");
        return;
    }

    if isc != -1 {
        println!("isc type check failed");
        return;
    }

    if -1 < 0 {
        if ipc != -1 {
            println!("ipc type check failed");
            return;
        }
        if (uc as i32) != 255 {
            println!("uc type check failed");
            return;
        }
    } else {
        if (ipc as u32) != 255 {
            println!("ipc type check failed");
            return;
        }
        if (uc as u32) != 255 {
            println!("uc type check failed");
            return;
        }
    }

    println!("All type checks passed");
}