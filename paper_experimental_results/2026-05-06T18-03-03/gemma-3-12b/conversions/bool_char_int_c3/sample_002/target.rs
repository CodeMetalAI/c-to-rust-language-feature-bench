fn main() {
    let pc: volatile char = std::mem::transmute(-1i8 as char);
    let sc: volatile signed char = std::mem::transmute(-1i8 as signed char);
    let uc: volatile unsigned char = std::mem::transmute(-1i8 as unsigned char);

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

    if TYPE_ID(pc + 0) != 1 {
        std::process::exit(1);
    }
    if TYPE_ID(sc + 0) != 1 {
        std::process::exit(2);
    }
    if TYPE_ID(uc + 0) != 1 {
        std::process::exit(3);
    }

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

#[allow(dead_code)]
fn TYPE_ID<T>(x: T) -> i32 {
    if std::mem::size_of::<T>() == std::mem::size_of::<i32>() {
        1
    } else if std::mem::size_of::<T>() == std::mem::size_of::<u32>() {
        2
    } else {
        99
    }
}