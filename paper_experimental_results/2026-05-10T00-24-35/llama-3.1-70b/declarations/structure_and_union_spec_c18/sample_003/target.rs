#[repr(C, packed)]
struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 0],
}

fn compute_sum(n: u32) -> u32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += (i + 1) * 3 + 1;
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: usize = 7;

    let mut storage: [u8; std::mem::size_of::<Packet>() + N * std::mem::size_of::<u32>() + 32] = [0; std::mem::size_of::<Packet>() + N * std::mem::size_of::<u32>() + 32];
    let p: *mut Packet = storage.as_mut_ptr() as *mut Packet;

    unsafe {
        (*p).tag = 0xA1B2C3D4;
        (*p).n = N as u32;

        let off = std::mem::offset_of!(Packet, data);
        let expected = &storage[off..];
        let got1 = &(*p).data as *const _ as *const u8;
        let got2 = &(*p).data as *const _ as *const u8;

        if got1 as *const _ as usize != expected.as_ptr() as usize {
            return 1;
        }
        if got2 as *const _ as usize != expected.as_ptr() as usize {
            return 2;
        }

        for i in 0..N {
            let v = (i as u32 + 1) * 3 + 1;
            (*p).data[i] = v;
            (*p).data[i] += 0;
        }

        (*p).sum = 0;
        for i in 0..N {
            (*p).sum += (*p).data[i];
        }

        if (*p).sum != compute_sum(N as u32) {
            return 3;
        }

        if std::mem::size_of::<Packet>() < off {
            return 4;
        }
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: [u8; std::mem::size_of::<Packet>()] = [0; std::mem::size_of::<Packet>()];
    let p: *mut Packet = storage.as_mut_ptr() as *mut Packet;

    unsafe {
        (*p).tag = 0;
        (*p).n = 0;
        (*p).sum = 0;

        let off = std::mem::offset_of!(Packet, data);
        let expected = &storage[off..];
        let got1 = &(*p).data as *const _ as *const u8;
        let got2 = &(*p).data as *const _ as *const u8;

        if got1 as *const _ as usize != expected.as_ptr() as usize {
            return 10;
        }
        if got2 as *const _ as usize != expected.as_ptr() as usize {
            return 11;
        }
    }

    0
}

fn main() {
    let mut r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r);
    }

    r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }
}