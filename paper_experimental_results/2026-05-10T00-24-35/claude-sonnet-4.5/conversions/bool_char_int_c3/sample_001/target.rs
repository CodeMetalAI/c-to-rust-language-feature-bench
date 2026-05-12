fn main() {
    let pc: i8 = -1i8 as i8;
    let sc: i8 = -1i8;
    let uc: u8 = 255u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if ipc + 0 != ipc {
        std::process::exit(1);
    }
    if isc + 0 != isc {
        std::process::exit(2);
    }
    if iuc + 0 != iuc {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    if (i8::MIN as i8) < 0 {
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