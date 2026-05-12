use std::mem::size_of;
use std::process::exit;

type U8 = u8;
type U32 = u32;

#[derive(Default)]
struct PacketHeader {
    tag: U32,
    n: U32,
    sum: U32,
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
    const N: usize = 7;

    let header_size = size_of::<PacketHeader>();
    let _storage: Vec<U8> = vec![0u8; header_size + N * size_of::<U32>() + 32];

    let mut header = PacketHeader::default();
    let mut data: Vec<U32> = vec![0; N];

    header.tag = 0xA1B2C3D4;
    header.n = N as U32;

    {
        let off = header_size;
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
        while i < header.n {
            let v = (i + 1) * 3 + 1;
            data[i as usize] = v;
            data[i as usize] = data[i as usize].wrapping_add(0);
            i += 1;
        }
    }

    header.sum = 0;
    {
        let mut i: U32 = 0;
        while i < header.n {
            header.sum = header.sum.wrapping_add(data[i as usize]);
            i += 1;
        }
    }

    if header.sum != compute_sum(N as U32) {
        return 3;
    }

    if size_of::<PacketHeader>() < header_size {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let header_size = size_of::<PacketHeader>();
    let _storage: Vec<U8> = vec![0u8; header_size];

    let mut header = PacketHeader::default();

    header.tag = 0;
    header.n = 0;
    header.sum = 0;

    {
        let off = header_size;
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