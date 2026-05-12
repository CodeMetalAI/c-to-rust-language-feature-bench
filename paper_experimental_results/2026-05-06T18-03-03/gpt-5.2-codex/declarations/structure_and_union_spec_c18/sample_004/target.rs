use std::mem::size_of;

type U8 = u8;
type U32 = u32;

#[repr(C)]
struct PacketHeader {
    tag: U32,
    n: U32,
    sum: U32,
}

fn offset_of_data() -> usize {
    size_of::<PacketHeader>()
}

fn compute_sum(n: U32) -> U32 {
    let mut s: U32 = 0;
    let mut i: U32 = 0;
    while i < n {
        let v = (i.wrapping_add(1)).wrapping_mul(3).wrapping_add(1);
        s = s.wrapping_add(v);
        i = i.wrapping_add(1);
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: U32 = 7;

    let _storage_size = size_of::<PacketHeader>() + (N as usize) * size_of::<U32>() + 32usize;
    let mut packet = PacketHeader { tag: 0, n: 0, sum: 0 };
    let mut data: Vec<U32> = vec![0; N as usize];

    packet.tag = 0xA1B2C3D4u32;
    packet.n = N;

    {
        let off = offset_of_data();
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
        while i < packet.n {
            let v = (i.wrapping_add(1)).wrapping_mul(3).wrapping_add(1);
            data[i as usize] = v;
            data[i as usize] = data[i as usize].wrapping_add(0);
            i = i.wrapping_add(1);
        }
    }

    packet.sum = 0;
    {
        let mut i: U32 = 0;
        while i < packet.n {
            packet.sum = packet.sum.wrapping_add(data[i as usize]);
            i = i.wrapping_add(1);
        }
    }

    if packet.sum != compute_sum(N) {
        return 3;
    }

    if size_of::<PacketHeader>() < offset_of_data() {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let _storage_size = size_of::<PacketHeader>();
    let mut packet = PacketHeader { tag: 0, n: 0, sum: 0 };
    let _data: Vec<U32> = Vec::new();

    packet.tag = 0u32;
    packet.n = 0u32;
    packet.sum = 0u32;

    {
        let off = offset_of_data();
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
}