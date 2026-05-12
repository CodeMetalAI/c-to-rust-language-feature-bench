// structure_and_union_spec.rs

#[derive(Debug)]
struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 7], // N = 7
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
    let storage = [0u8; 7 * 4 + 4 + 32]; // sizeof(Packet) + N * sizeof(u32) + 32
    let p = &mut storage[..] as *mut Packet;

    p.tag = 0xA1B2C3D4;
    p.n = 7;

    {
        let off = std::mem::size_of::<Packet>() as usize + std::mem::size_of::<u32>() * 7;
        let expected = &storage[off..];
        let got1 = &p.data as *const _ as *const u8;
        let got2 = &(*p).data as *const _ as *const u8;

        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }
    }

    for i in 0..p.n {
        let v = (i + 1) * 3 + 1;
        p.data[i] = v;
        (*p).data[i] += 0;
    }

    p.sum = 0;
    for i in 0..p.n {
        p.sum += p.data[i];
    }

    if p.sum != compute_sum(p.n) {
        return 3;
    }

    if std::mem::size_of::<Packet>() < std::mem::size_of::<u32>() * 7 {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let storage = [0u8; std::mem::size_of::<Packet>()];
    let p = &mut storage[..] as *mut Packet;

    p.tag = 0;
    p.n = 0;
    p.sum = 0;

    {
        let off = std::mem::size_of::<Packet>() as usize;
        let expected = &storage[off..];

        let got1 = &p.data as *const _ as *const u8;
        let got2 = &(*p).data as *const _ as *const u8;

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
    let r = test_nonempty_object();
    if r != 0 {
        return;
    }

    let r = test_zero_element_object();
    if r != 0 {
        return;
    }

    println!("Success");
}

fn main() {
    let r = test_nonempty_object();
    if r != 0 {
        return;
    }

    let r = test_zero_element_object();
    if r != 0 {
        return;
    }

    println!("Success");
}