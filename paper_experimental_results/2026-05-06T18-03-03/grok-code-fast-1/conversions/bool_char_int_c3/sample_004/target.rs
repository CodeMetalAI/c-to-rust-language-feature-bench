use std::any::TypeId;
use std::process;

fn type_id<T>(_: &T) -> i32 {
    if TypeId::of::<T>() == TypeId::of::<i32>() {
        1
    } else if TypeId::of::<T>() == TypeId::of::<u32>() {
        2
    } else {
        99
    }
}

fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc: i32 = pc as i32;
    let isc: i32 = sc as i32;
    let iuc: i32 = uc as i32;

    if type_id(&(pc as i32 + 0)) != 1 {
        process::exit(1);
    }
    if type_id(&(sc as i32 + 0)) != 1 {
        process::exit(2);
    }
    if type_id(&(uc as i32 + 0)) != 1 {
        process::exit(3);
    }

    if isc != -1 {
        process::exit(4);
    }

    if pc < 0 {
        if ipc != -1 {
            process::exit(5);
        }
        if iuc as u32 != 255 {
            process::exit(6);
        }
    } else {
        if ipc as u32 != 255 {
            process::exit(7);
        }
        if iuc as u32 != 255 {
            process::exit(8);
        }
    }

    process::exit(0);
}