fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if 1 != 1 {
        std::process::exit(1);
    }
    if 1 != 1 {
        std::process::exit(2);
    }
    if 1 != 1 {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    let char_is_signed = (-1i8) < 0;
    if char_is_signed {
        if ipc != -1 {
            std::process::exit(5);
        }
        if (iuc as u32) != 255u32 {
            std::process::exit(6);
        }
    } else {
        if (ipc as u32) != 255u32 {
            std::process::exit(7);
        }
        if (iuc as u32) != 255u32 {
            std::process::exit(8);
        }
    }

    std::process::exit(0);
}