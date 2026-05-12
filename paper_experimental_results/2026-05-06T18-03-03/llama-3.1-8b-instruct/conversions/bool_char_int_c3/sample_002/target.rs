fn main() {
    let pc: i8 = -1;
    let sc: i8 = -1;
    let uc: u8 = 255;

    let ipc = pc as i32;
    let isc = sc as i32;
    let iuc = uc as i32;

    if TYPE_ID(pc + 0) != 1 {
        return 1;
    }
    if TYPE_ID(sc + 0) != 1 {
        return 2;
    }
    if TYPE_ID(uc + 0) != 1 {
        return 3;
    }

    if isc != -1 {
        return 4;
    }

    if -1 < 0 {
        if ipc != -1 {
            return 5;
        }
        if iuc != 255 {
            return 6;
        }
    } else {
        if ipc != 255 {
            return 7;
        }
        if iuc != 255 {
            return 8;
        }
    }

    return 0;
}

fn TYPE_ID<T>(x: T) -> i32
where
    T: Copy + Eq + std::ops::Add<Output = T> + std::convert::From<i32>,
{
    match x {
        _ if x == -1 => 1,
        _ if x == 255 => 2,
        _ => 99,
    }
}