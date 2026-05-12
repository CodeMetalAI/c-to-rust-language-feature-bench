fn main() {
    let pc: volatile char = std::mem::transmute((-1) as i8);
    let sc: volatile signed char = std::mem::transmute((-1) as i8);
    let uc: volatile unsigned char = std::mem::transmute((-1) as u8);

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if type_id(pc + 0) != 1 {
        std::process::exit(1);
    }
    if type_id(sc + 0) != 1 {
        std::process::exit(2);
    }
    if type_id(uc + 0) != 1 {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    if (pc as i8) < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if (iuc as u32) != 255 {
            std::process::exit(6);
        }
    } else {
        if (ipc as u32) != 255 {
            std::process::exit(7);
        }
        if (iuc as u32) != 255 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}

fn type_id<T>(x: T) -> i32 {
    if std::mem::size_of::<T>() == std::mem::size_of::<i32>() {
        1
    } else if std::mem::size_of::<T>() == std::mem::size_of::<u32>() {
        2
    } else {
        99
    }
}