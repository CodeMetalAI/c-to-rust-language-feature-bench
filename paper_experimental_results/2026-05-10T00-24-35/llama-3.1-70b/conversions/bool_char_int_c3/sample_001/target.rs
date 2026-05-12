fn main() {
    let pc: char = -1 as char;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if (pc as i32).signum() != 1 {
        return;
    }
    if (sc as i32).signum() != 1 {
        std::process::exit(2);
    }
    if (!uc.is_ascii()) {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    if pc.is_ascii() {
        if ipc != -1 {
            std::process::exit(5);
        }
        if iuc as u32 != 255 {
            std::process::exit(6);
        }
    } else {
        if ipc as u32 != 255 {
            std::process::exit(7);
        }
        if iuc as u32 != 255 {
            std::process::exit(8);
        }
    }
}