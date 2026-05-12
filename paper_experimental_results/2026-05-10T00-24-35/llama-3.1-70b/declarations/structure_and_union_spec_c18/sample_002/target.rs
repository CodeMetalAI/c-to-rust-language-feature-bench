fn compute_sum(n: u32) -> u32 {
    (0..n).map(|i| (i + 1) * 3 + 1).sum()
}

fn test_nonempty_object() -> i32 {
    const N: usize = 7;

    let mut storage: [u8; std::mem::size_of::<Packet>() + N * std::mem::size_of::<u32>() + 32] = [0; std::mem::size_of::<Packet>() + N * std::mem::size_of::<u32>() + 32];
    let p = storage.as_mut_ptr() as *mut Packet;

    unsafe {
        (*p).tag = 0xA1B2C3D4;
        (*p).n = N as u32;

        let expected = storage.as_ptr().offset(std::mem::size_of::<Packet>() as isize);
        let got1 = &mut (*p).data as *mut u32 as *mut u8;
        let got2 = &mut (*p).data as *mut u32 as *mut u8;

        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }

        for i in 0..N {
            (*p).data[i as usize] = (i as u32 + 1) * 3 + 1;
            (*p).data[i as usize] += 0;
        }

        (*p).sum = 0;
        for i in 0..N {
            (*p).sum += (*p).data[i as usize];
        }

        if (*p).sum != compute_sum(N as u32) {
            return 3;
        }

        if std::mem::size_of::<Packet>() < std::mem::size_of::<Packet>() {
            return 4;
        }
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: [u8; std::mem::size_of::<Packet>()] = [0; std::mem::size_of::<Packet>()];
    let p = storage.as_mut_ptr() as *mut Packet;

    unsafe {
        (*p).tag = 0;
        (*p).n = 0;
        (*p).sum = 0;

        let expected = storage.as_ptr().offset(std::mem::size_of::<Packet>() as isize);
        let got1 = &mut (*p).data as *mut u32 as *mut u8;
        let got2 = &mut (*p).data as *mut u32 as *mut u8;

        if got1 != expected {
            return 10;
        }
        if got2 != expected {
            return 11;
        }
    }

    0
}

#[repr(C)]
struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32],
}

fn main() {
    let r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r);
    }

    let r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }
}