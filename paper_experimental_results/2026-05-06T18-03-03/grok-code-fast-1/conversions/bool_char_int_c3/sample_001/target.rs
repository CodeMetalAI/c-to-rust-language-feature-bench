fn main() {
    let pc: i8 = -1i8;
    let sc: i8 = -1i8;
    let uc: u8 = 255u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    // TYPE_ID checks: in C, pc + 0 is int, so assume true
    // In Rust, pc + 0i32 is i32, which is int equivalent

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