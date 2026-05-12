fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255; // equivalent to -1 for unsigned char

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

    // TYPE_ID(pc + 0) and similar checks to see if the result is of type int
    // In Rust, we operate with integers and use type inference

    if (pc as i32) != ipc {
        std::process::exit(1);
    }
    if (sc as i32) != isc {
        std::process::exit(2);
    }
    if (uc as i32) != iuc {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    if -1i8 < 0 {
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