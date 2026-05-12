fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = u8::MAX;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if (pc + 0) as i32 != 1 {
        std::process::exit(1);
    }
    if (sc + 0) as i32 != 1 {
        std::process::exit(2);
    }
    if (uc as i32 + 0) as i32 != 1 {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    if -1 < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if iuc as u32 != 255u32 {
            std::process::exit(6);
        }
    } else {
        if ipc as u32 != 255u32 {
            std::process::exit(7);
        }
        if iuc as u32 != 255u32 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}