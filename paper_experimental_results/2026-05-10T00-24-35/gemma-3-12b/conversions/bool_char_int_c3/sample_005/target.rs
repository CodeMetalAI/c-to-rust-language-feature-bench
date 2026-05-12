fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

    if type_id(pc) != 1 {
        std::process::exit(1);
    }
    if type_id(sc) != 1 {
        std::process::exit(2);
    }
    if type_id(uc) != 2 {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    if pc as i32 < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if (uc as u32) != 255 {
            std::process::exit(6);
        }
    } else {
        if (pc as u32) != 255 {
            std::process::exit(7);
        }
        if (uc as u32) != 255 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}

fn type_id<T>(x: T) -> i32 {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i8>() {
        1
    } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<u8>() {
        2
    } else {
        99
    }
}