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

    let mut storage: Vec<u32> = vec![0; (std::mem::size_of::<Packet>() + N * std::mem::size_of::<u32>() + 32) / 4];
    let p = unsafe { &mut *(storage.as_mut_ptr() as *mut Packet) };

    p.tag = 0xA1B2C3D4;
    p.n = N as u32;

    let off = std::mem::size_of::<u32>() * 3;
    let expected = &mut storage[off / 4] as *mut u32 as *mut u8;
    let got1 = &mut p.data as *mut [u32] as *mut u8;
    let got2 = &mut (*p).data as *mut [u32] as *mut u8;

    if got1 != expected {
        return 1;
    }
    if got2 != expected {
        return 2;
    }

    for i in 0..p.n as usize {
        let v = (i + 1) as u32 * 3 + 1;
        p.data[i] = v;
    }

    p.sum = 0;
    for i in 0..p.n as usize {
        p.sum += p.data[i];
    }

    if p.sum != compute_sum(N as u32) {
        return 3;
    }

    if std::mem::size_of::<Packet>() < std::mem::size_of::<u32>() * 3 {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: Vec<u32> = vec![0; std::mem::size_of::<Packet>() / 4];
    let p = unsafe { &mut *(storage.as_mut_ptr() as *mut Packet) };

    p.tag = 0;
    p.n = 0;
    p.sum = 0;

    let off = std::mem::size_of::<u32>() * 3;
    let expected = &mut storage[off / 4] as *mut u32 as *mut u8;
    let got1 = &mut p.data as *mut [u32] as *mut u8;
    let got2 = &mut (*p).data as *mut [u32] as *mut u8;

    if got1 != expected {
        return 10;
    }
    if got2 != expected {
        return 11;
    }

    0
}

#[repr(C)]
struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 0],
}