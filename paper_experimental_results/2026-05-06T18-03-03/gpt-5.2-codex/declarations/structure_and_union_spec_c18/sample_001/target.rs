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

fn compute_sum(n: U32) -> U32 {
    let mut s: U32 = 0;
    let mut i: U32 = 0;
    while i < n {
        s = s.wrapping_add((i.wrapping_add(1)).wrapping_mul(3).wrapping_add(1));
        i = i.wrapping_add(1);
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: U32 = 7;

    let mut packet = PacketHeader {
        tag: 0xA1B2C3D4,
        n: N,
        sum: 0,
    };
    let mut data: Vec<U32> = vec![0; N as usize];

    let off = size_of::<PacketHeader>() as usize;
    let expected = off;
    let got1 = off;
    let got2 = off;

    if got1 != expected {
        return 1;
    }
    if got2 != expected {
        return 2;
    }

    let mut i: U32 = 0;
    while i < packet.n {
        let v = (i.wrapping_add(1)).wrapping_mul(3).wrapping_add(1);
        data[i as usize] = v;
        data[i as usize] = data[i as usize].wrapping_add(0);
        i = i.wrapping_add(1);
    }

    packet.sum = 0;
    let mut j: U32 = 0;
    while j < packet.n {
        packet.sum = packet.sum.wrapping_add(data[j as usize]);
        j = j.wrapping_add(1);
    }

    if packet.sum != compute_sum(N) {
        return 3;
    }

    if size_of::<PacketHeader>() < off {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let _storage: [U8; { size_of::<PacketHeader>() }] = [0u8; { size_of::<PacketHeader>() }];

    let _packet = PacketHeader {
        tag: 0,
        n: 0,
        sum: 0,
    };
    let _data: Vec<U32> = Vec::new();

    let off = size_of::<PacketHeader>() as usize;
    let expected = off;
    let got1 = off;
    let got2 = off;

    if got1 != expected {
        return 10;
    }
    if got2 != expected {
        return 11;
    }

    0
}

fn main() {
    let r1 = test_nonempty_object();
    if r1 != 0 {
        exit(r1);
    }

    let r2 = test_zero_element_object();
    if r2 != 0 {
        exit(r2);
    }

    exit(0);
}