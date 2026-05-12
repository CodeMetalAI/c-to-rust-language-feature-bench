fn main() -> i32 {
    let pc: char = -1i8 as char;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

    if type_id(pc.wrapping_add(0 as char)) != 1 {
        return 1;
    }
    if type_id(sc.wrapping_add(0)) != 1 {
        return 2;
    }
    if type_id(uc.wrapping_add(0)) != 1 {
        return 3;
    }

    if isc != -1 {
        return 4;
    }

    if (-1i8 as char) < 0 as char {
        if ipc != -1 {
            return 5;
        }
        if iuc != 255 {
            return 6;
        }
    } else {
        if ipc as u32 != 255 {
            return 7;
        }
        if iuc as u32 != 255 {
            return 8;
        }
    }

    return 0;
}

fn type_id<T>(_: T) -> i32 {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
        1
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
        2
    } else {
        99
    }
}