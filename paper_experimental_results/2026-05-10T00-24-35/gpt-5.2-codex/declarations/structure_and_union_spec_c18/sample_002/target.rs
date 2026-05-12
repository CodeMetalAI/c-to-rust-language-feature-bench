use std::mem::size_of;

#[derive(Clone, Copy)]
struct PacketHeader {
    tag: u32,
    n: u32,
    sum: u32,
}

fn offsetof_data() -> usize {
    size_of::<PacketHeader>()
}

fn compute_sum(n: u32) -> u32 {
    let mut s: u32 = 0;
    let mut i: u32 = 0;
    while i < n {
        s = s.wrapping_add((i + 1).wrapping_mul(3).wrapping_add(1));
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: u32 = 7;

    let _storage_size = size_of::<PacketHeader>() + (N as usize) * size_of::<u32>() + 32;

    let mut header = PacketHeader {
        tag: 0,
        n: 0,
        sum: 0,
    };
    let mut data: Vec<u32> = vec![0; N as usize];

    header.tag = 0xA1B2C3D4;
    header.n = N;

    {
        let off = offsetof_data();
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
        let mut i: u32 = 0;
        while i < header.n {
            let v = (i + 1).wrapping_mul(3).wrapping_add(1);
            data[i as usize] = v;
            data[i as usize] = data[i as usize].wrapping_add(0);
            i += 1;
        }
    }

    header.sum = 0;
    {
        let mut i: u32 = 0;
        while i < header.n {
            header.sum = header.sum.wrapping_add(data[i as usize]);
            i += 1;
        }
    }

    if header.sum != compute_sum(N) {
        return 3;
    }

    if size_of::<PacketHeader>() < offsetof_data() {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let _storage_size = size_of::<PacketHeader>();

    let mut header = PacketHeader {
        tag: 0,
        n: 0,
        sum: 0,
    };

    header.tag = 0;
    header.n = 0;
    header.sum = 0;

    {
        let off = offsetof_data();
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
        std::process::exit(r);
    }

    r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }

    std::process::exit(0);
}