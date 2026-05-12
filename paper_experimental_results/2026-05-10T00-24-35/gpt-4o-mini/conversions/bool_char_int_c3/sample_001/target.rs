fn main() -> i32 {
    let pc: char = -1i8 as char;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if type_id(pc as i32) != 1 {
        return 1;
    }
    if type_id(sc as i32) != 1 {
        return 2;
    }
    if type_id(uc as i32) != 1 {
        return 3;
    }

    if isc != -1 {
        return 4;
    }

    if (-1i8 as char) < 0 {
        if ipc != -1 {
            return 5;
        }
        if iuc != 255 {
            return 6;
        }
    } else {
        if ipc != 255 {
            return 7;
        }
        if iuc != 255 {
            return 8;
        }
    }

    return 0;
}

fn type_id(x: i32) -> i32 {
    match x {
        x if x == std::i32::from(std::i32::MIN) => 1, // Simulating int
        _ => 99, // default case
    }
}