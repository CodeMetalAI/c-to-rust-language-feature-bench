fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

    if !matches!(pc + 0, 0..=i32::MAX) {
        std::process::exit(1);
    }
    if !matches!(sc + 0, 0..=i32::MAX) {
        std::process::exit(2);
    }
    if !matches!(uc + 0, 0..=i32::MAX) {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    if pc < 0 {
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