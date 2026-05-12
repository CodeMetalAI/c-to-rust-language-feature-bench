use std::os::raw::c_char;

fn type_id_int(_x: i32) -> i32 {
    1
}

fn main() {
    let pc: c_char = (-1i8) as c_char;
    let sc: i8 = -1i8;
    let uc: u8 = 255u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if type_id_int((pc as i32) + 0) != 1 {
        std::process::exit(1);
    }
    if type_id_int((sc as i32) + 0) != 1 {
        std::process::exit(2);
    }
    if type_id_int((uc as i32) + 0) != 1 {
        std::process::exit(3);
    }

    if isc != -1 {
        std::process::exit(4);
    }

    let char_minus_one: c_char = (-1i8) as c_char;
    if (char_minus_one as i32) < 0 {
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