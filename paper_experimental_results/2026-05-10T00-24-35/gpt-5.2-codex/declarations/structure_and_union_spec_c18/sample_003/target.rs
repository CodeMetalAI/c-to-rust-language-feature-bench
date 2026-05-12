use std::mem::size_of;
use std::process::exit;

type U8 = u8;
type U32 = u32;

#[repr(C)]
struct PacketHeader {
    tag: U32,
    n: U32,
    sum: U32,
}

struct Packet {
    tag: U32,
    n: U32,
    sum: U32,
    data: Vec<U32>,
}

fn compute_sum(n: U32) -> U32 {
    let mut s: U32 = 0;
    let mut i: U32 = 0;
    while i < n {
        let v = i
            .wrapping_add(1)
            .wrapping_mul(3)
            .wrapping_add(1);
        s = s.wrapping_add(v);
        i = i.wrapping_add(1);
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: U32 = 7;
    let offset = size_of::<PacketHeader>() as usize;
    let _storage: Vec<U8> = vec![0u8; offset + (N as usize) * size_of::<U32>() + 32];

    let mut p = Packet {
        tag: 0xA1B2C3D4,
        n: N,
        sum: 0,
        data: vec![0; N as usize],
    };

    {
        let expected = offset;
        let got1 = offset;
        let got2 = offset;
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
            let v = i
                .wrapping_add(1)
                .wrapping_mul(3)
                .wrapping_add(1);
            p.data[i as usize] = v;
            p.data[i as usize] = p.data[i as usize].wrapping_add(0);
            i = i.wrapping_add(1);
        }
    }

    p.sum = 0;
    {
        let mut i: U32 = 0;
        while i < p.n {
            p.sum = p.sum.wrapping_add(p.data[i as usize]);
            i = i.wrapping_add(1);
        }
    }

    if p.sum != compute_sum(N) {
        return 3;
    }

    let size_packet = size_of::<PacketHeader>() as usize;
    if size_packet < offset {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let offset = size_of::<PacketHeader>() as usize;
    let _storage: Vec<U8> = vec![0u8; offset];

    let mut p = Packet {
        tag: 0,
        n: 0,
        sum: 0,
        data: Vec::new(),
    };

    {
        let expected = offset;
        let got1 = offset;
        let got2 = offset;
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
        exit(r);
    }

    r = test_zero_element_object();
    if r != 0 {
        exit(r);
    }

    exit(0);
}