// structure_and_union_spec.rs

const N: u32 = 7;

struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 7],
}

fn compute_sum(n: u32) -> u32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += (i + 1u32) * 3u32 + 1u32;
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    let mut storage = [0u8; 144];

    let p = &mut Packet {
        tag: 0xA1B2C3D4u32,
        n: N,
        sum: 0,
        data: [0u32; 7],
    };

    let expected = &storage[(unsafe { std::mem::offset_of::<Packet>(p as *const _) }) as usize..];

    let got1 = unsafe { &p.data as *const _ as *const u8 };
    let got2 = unsafe { (&*p).data as *const _ as *const u8 };

    if got1 != expected {
        return 1;
    }
    if got2 != expected {
        return 2;
    }

    for i in 0..p.n {
        let v = (i + 1u32) * 3u32 + 1u32;
        p.data[i] = v;
        (*p).data[i] += 0;
    }

    p.sum = 0;
    for i in 0..p.n {
        p.sum += p.data[i];
    }

    if p.sum != compute_sum(N) {
        return 3;
    }

    if std::mem::size_of::<Packet>() < unsafe { std::mem::offset_of::<Packet>(p as *const _) } {
        return 4;
    }

    return 0;
}

fn test_zero_element_object() -> i32 {
    let mut storage = [0u8; 40];

    let p = &mut Packet {
        tag: 0u32,
        n: 0,
        sum: 0,
        data: [0u32; 7],
    };

    let expected = &storage[(unsafe { std::mem::offset_of::<Packet>(p as *const _) }) as usize..];

    let got1 = unsafe { &p.data as *const _ as *const u8 };
    let got2 = unsafe { (&*p).data as *const _ as *const u8 };

    if got1 != expected {
        return 10;
    }
    if got2 != expected {
        return 11;
    }

    return 0;
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
}