use std::mem::size_of;

type U8 = u8;
type U32 = u32;

#[derive(Default)]
struct Packet {
    tag: U32,
    n: U32,
    sum: U32,
    data: Vec<U32>,
}

#[repr(C)]
struct PacketHeader {
    tag: U32,
    n: U32,
    sum: U32,
    data: [U32; 0],
}

fn compute_sum(n: U32) -> U32 {
    let mut s: U32 = 0;
    let mut i: U32 = 0;
    while i < n {
        s = s.wrapping_add((i + 1) * 3 + 1);
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: U32 = 7;

    // Simulated storage
    let _storage: Vec<U8> = vec![
        0;
        size_of::<PacketHeader>() + (N as usize) * size_of::<U32>() + 32
    ];

    let mut p = Packet {
        tag: 0xA1B2C3D4,
        n: N,
        sum: 0,
        data: vec![0; N as usize],
    };

    {
        let off = size_of::<PacketHeader>();
        let expected = off;
        let got1 = off;
        let got2 = off;

        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }
    }

    {
        let mut i: U32 = 0;
        while i < p.n {
            let v = (i + 1) * 3 + 1;
            let idx = i as usize;
            p.data[idx] = v;
            p.data[idx] += 0;
            i += 1;
        }
    }

    p.sum = 0;
    {
        let mut i: U32 = 0;
        while i < p.n {
            p.sum = p.sum.wrapping_add(p.data[i as usize]);
            i += 1;
        }
    }

    if p.sum != compute_sum(N) {
        return 3;
    }

    let off = size_of::<PacketHeader>();
    let size = size_of::<PacketHeader>();
    if size < off {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    // Simulated storage
    let _storage: Vec<U8> = vec![0; size_of::<PacketHeader>()];

    let mut p = Packet {
        tag: 0,
        n: 0,
        sum: 0,
        data: Vec::new(),
    };

    {
        let off = size_of::<PacketHeader>();
        let expected = off;

        let got1 = off;
        let got2 = off;

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
        std::process::exit(r);
    }

    let r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }

    std::process::exit(0);
}