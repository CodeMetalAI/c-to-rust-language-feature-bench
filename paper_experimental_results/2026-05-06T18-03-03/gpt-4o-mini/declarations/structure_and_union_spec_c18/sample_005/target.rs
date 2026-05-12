struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: Vec<u32>,
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
    const N: u32 = 7;

    let mut storage: Vec<u8> = vec![0; std::mem::size_of::<Packet>() + (N as usize * std::mem::size_of::<u32>()) + 32];
    let p = unsafe {
        &mut *(storage.as_mut_ptr() as *mut Packet)
    };

    p.tag = 0xA1B2C3D4;
    p.n = N;
    p.data = vec![0; N as usize];

    let off = std::mem::size_of::<u32>() * 3; // offset of data
    let expected = storage.as_ptr().add(off) as *const u8;

    let got1 = &p.data as *const _ as *const u8;
    let got2 = &(*p).data as *const _ as *const u8;

    if got1 != expected {
        return 1;
    }
    if got2 != expected {
        return 2;
    }

    for i in 0..p.n {
        let v = (i + 1) * 3 + 1;
        p.data[i as usize] = v;
    }

    p.sum = 0;
    for i in 0..p.n {
        p.sum += p.data[i as usize];
    }

    if p.sum != compute_sum(N) {
        return 3;
    }

    if std::mem::size_of::<Packet>() < off as usize {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: Vec<u8> = vec![0; std::mem::size_of::<Packet>()];
    let p = unsafe {
        &mut *(storage.as_mut_ptr() as *mut Packet)
    };

    p.tag = 0;
    p.n = 0;
    p.sum = 0;

    let off = std::mem::size_of::<u32>() * 3; // offset of data
    let expected = storage.as_ptr().add(off) as *const u8;

    let got1 = &p.data as *const _ as *const u8;
    let got2 = &(*p).data as *const _ as *const u8;

    if got1 != expected {
        return 10;
    }
    if got2 != expected {
        return 11;
    }

    0
}

fn main() {
    let mut r;

    r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r);
    }

    r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }

    std::process::exit(0);
}