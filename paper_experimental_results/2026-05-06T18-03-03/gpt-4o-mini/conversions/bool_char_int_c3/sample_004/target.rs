fn type_id<T>(_: T) -> u32 {
    std::any::TypeId::of::<T>().hash(&mut std::collections::hash_map::DefaultHasher::new());
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
        1
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u32>() {
        2
    } else {
        99
    }
}

fn main() -> i32 {
    let pc: i8 = -1;
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

    if (pc as i32) < 0 {
        if ipc != -1 {
            return 5;
        }
        if iuc as u32 != 255 {
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

    0
}