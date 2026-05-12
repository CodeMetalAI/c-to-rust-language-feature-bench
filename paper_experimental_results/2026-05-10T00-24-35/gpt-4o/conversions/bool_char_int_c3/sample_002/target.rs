fn main() -> i32 {
    let pc = -1i8 as i8;
    let sc = -1i8 as i8;
    let uc = 255u8 as u8;

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

    // In Rust, all types are inferred correctly, so no need for TYPE_ID

    if !matches!(pc as i32 + 0, 1) {
        return 1;
    }
    if !matches!(sc as i32 + 0, 1) {
        return 2;
    }
    if !matches!(uc as i32 + 0, 1) {
        return 3;
    }

    if isc != -1 {
        return 4;
    }

    if (-1i8 as i8) < 0 {
        if ipc != -1 {
            return 5;
        }
        if iuc as u32 != 255u32 {
            return 6;
        }
    } else {
        if ipc as u32 != 255u32 {
            return 7;
        }
        if iuc as u32 != 255u32 {
            return 8;
        }
    }

    0
}