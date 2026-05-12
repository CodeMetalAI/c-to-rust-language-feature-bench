fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = u8::MAX;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: u32 = uc as u32;

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

    if -1i8 < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if iuc != 255u32 {
            std::process::exit(6);
        }
    } else {
        if ipc as u32 != 255u32 {
            std::process::exit(7);
        }
        if iuc != 255u32 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}