use std::any::TypeId;
use std::os::raw::c_char;
use std::process::exit;

fn type_id_val<T: 'static>(_: T) -> i32 {
    if TypeId::of::<T>() == TypeId::of::<i32>() {
        1
    } else if TypeId::of::<T>() == TypeId::of::<u32>() {
        2
    } else {
        99
    }
}

fn main() {
    let pc: c_char = (-1i8) as c_char;
    let sc: i8 = -1i8;
    let uc: u8 = (-1i8) as u8;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if type_id_val(pc as i32 + 0) != 1 {
        exit(1);
    }
    if type_id_val(sc as i32 + 0) != 1 {
        exit(2);
    }
    if type_id_val(uc as i32 + 0) != 1 {
        exit(3);
    }

    if isc != -1 {
        exit(4);
    }

    if (pc as i32) < 0 {
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