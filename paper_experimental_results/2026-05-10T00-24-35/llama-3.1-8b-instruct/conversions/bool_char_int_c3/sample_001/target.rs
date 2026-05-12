fn type_id(x: u8) -> i32 {
    match x {
        _ if x >= 0 as i8 => 2,
        _ => 1,
    }
}

fn main() {
    let mut pc = -1i8 as u8;
    let mut sc = -1i8 as i8;
    let mut uc = -1u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if type_id(pc + 0) != 1 {
        return 1;
    }
    if type_id(sc + 0) != 1 {
        return 2;
    }
    if type_id(uc + 0) != 1 {
        return 3;
    }

    if isc != -1 {
        return 4;
    }

    if -1 < 0 {
        if ipc != -1 {
            return 5;
        }
        if uc as i32 != 255 {
            return 6;
        }
    } else {
        if ipc as u32 != 255 {
            return 7;
        }
        if uc as i32 != 255 {
            return 8;
        }
    }

    return 0;
}