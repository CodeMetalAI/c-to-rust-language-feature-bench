use std::mem::size_of;

#[derive(Debug, Copy, Clone)]
struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 0], // Flexible array member (FAM) - requires explicit size
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

    union {
        raw: [u8; size_of::<Packet>() + N * size_of::<u32>() + 32],
        align: u32,
    } storage;

    let p = unsafe { &mut *(storage.raw as *mut Packet) };

    p.tag = 0xA1B2C3D4;
    p.n = N;

    unsafe {
        let off = size_of::<Packet>() as u8;
        let expected = storage.raw.as_ptr() as *const u8.add(off);
        let got1 = (&p.data as *const [u32] as *const u8).add(0);
        let got2 = (&(*p).data as *const [u32] as *const u8).add(0);

        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }
    }

    unsafe {
        let mut i = 0;
        while i < p.n {
            let v = (i + 1) * 3 + 1;
            p.data[i as usize] = v;
            (*p).data[i as usize] += 0;
            i += 1;
        }
    }

    p.sum = 0;
    unsafe {
        let mut i = 0;
        while i < p.n {
            p.sum += p.data[i as usize];
            i += 1;
        }
    }

    if p.sum != compute_sum(N) {
        return 3;
    }

    if size_of::<Packet>() as u64 < size_of::<Packet>() as u64 + size_of::<u32>() as u64 {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    union {
        raw: [u8; size_of::<Packet>()],
        align: u32,
    } storage;

    let p = unsafe { &mut *(storage.raw as *mut Packet) };

    p.tag = 0;
    p.n = 0;
    p.sum = 0;

    unsafe {
        let off = size_of::<Packet>() as u8;
        let expected = storage.raw.as_ptr() as *const u8.add(off);

        let got1 = (&p.data as *const [u32] as *const u8).add(0);
        let got2 = (&(*p).data as *const [u32] as *const u8).add(0);

        if got1 != expected {
            return 10;
        }
        if got2 != expected {
            return 11;
        }
    }

    0
}

fn main() {
    let mut r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r as i32);
    }

    r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r as i32);
    }

    std::process::exit(0);
}