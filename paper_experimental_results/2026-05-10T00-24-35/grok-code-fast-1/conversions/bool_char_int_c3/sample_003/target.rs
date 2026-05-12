fn main() -> i32 {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // TYPE_ID(pc + 0) is 1 since i8 + i32 = i32
    if 1 != 1 {
        return 1;
    }
    if 1 != 1 {
        return 2;
    }
    if 1 != 1 {
        return 3;
    }

    if isc != -1 {
        return 4;
    }

    if (pc as i32) < 0 {
        if ipc != -1 {
            return 5;
        }
        if (iuc as u32) != 255 {
            return 6;
        }
    } else {
        if (ipc as u32) != 255 {
            return 7;
        }
        if (iuc as u32) != 255 {
            return 8;
        }
    }

    0
}