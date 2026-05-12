fn type_id<T>(_: T) -> u32 {
    match std::any::type_id_of::<T>() {
        std::any::TypeId::Int => 1,
        std::any::TypeId::Uint => 2,
        _ => 99,
    }
}

fn main() {
    let pc = std::mem::size_of::<i8>() as i8;
    let sc = std::mem::size_of::<i8>() as i8;
    let uc = std::mem::size_of::<u8>() as u8;

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

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

    if pc < 0 {
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