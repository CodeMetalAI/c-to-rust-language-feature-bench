fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: u32 = uc as u32;

    if !(pc + 0).is_positive() && !(pc + 0).is_negative() {
        return;
    }

    if !(sc + 0).is_positive() && !(sc + 0).is_negative() {
        return;
    }

    if !(uc + 0).is_positive() && !(uc + 0).is_negative() {
        return;
    }

    if isc != -1 {
        std::process::exit(4);
    }

    if i8::MIN < 0 {
        if ipc != -1 {
            std::process::exit(5);
        }
        if iuc != 255 {
            std::process::exit(6);
        }
    } else {
        if ipc as u32 != 255 {
            std::process::exit(7);
        }
        if iuc != 255 {
            std::process::exit(8);
        }
    }
}