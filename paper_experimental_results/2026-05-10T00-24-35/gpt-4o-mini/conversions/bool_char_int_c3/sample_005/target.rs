fn main() -> i32 {
    let pc: char = -1i8 as char;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

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

    if -1i8 < 0 {
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

fn type_id<T>(_: T) -> i32 {
    match std::any::TypeId::of::<T>() {
        id if id == std::any::TypeId::of::<i32>() => 1,
        id if id == std::any::TypeId::of::<u32>() => 2,
        _ => 99,
    }
}