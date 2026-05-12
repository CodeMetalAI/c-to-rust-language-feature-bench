use std::os::raw::c_char;
use std::process::exit;

fn main() {
    let pc: c_char = (-1i8) as c_char;
    let sc: i8 = -1i8;
    let uc: u8 = (-1i8) as u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    let type_id_pc = {
        let _ = (pc as i32) + 0;
        1
    };
    if type_id_pc != 1 {
        exit(1);
    }

    let type_id_sc = {
        let _ = (sc as i32) + 0;
        1
    };
    if type_id_sc != 1 {
        exit(2);
    }

    let type_id_uc = {
        let _ = (uc as i32) + 0;
        1
    };
    if type_id_uc != 1 {
        exit(3);
    }

    if isc != -1 {
        exit(4);
    }

    let char_minus_one: c_char = (-1i8) as c_char;
    if (char_minus_one as i32) < 0 {
        if ipc != -1 {
            exit(5);
        }
        if (iuc as u32) != 255u32 {
            exit(6);
        }
    } else {
        if (ipc as u32) != 255u32 {
            exit(7);
        }
        if (iuc as u32) != 255u32 {
            exit(8);
        }
    }

    exit(0);
}