use std::mem::size_of;

type U8 = u8;
type U32 = u32;

#[repr(C)]
struct Packet {
    tag: U32,
    n: U32,
    sum: U32,
    // flexible array member in C
}

fn write_u32(buf: &mut [u8], offset: usize, value: U32) {
    let bytes = value.to_ne_bytes();
    buf[offset..offset + 4].copy_from_slice(&bytes);
}

fn read_u32(buf: &[u8], offset: usize) -> U32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&buf[offset..offset + 4]);
    U32::from_ne_bytes(bytes)
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
    let packet_size = size_of::<Packet>();
    let data_offset = packet_size;

    let mut storage: Vec<U8> = vec![0u8; packet_size + (N as usize) * 4 + 32];

    write_u32(&mut storage, 0, 0xA1B2C3D4);
    write_u32(&mut storage, 4, N);

    {
        let off = data_offset;
        let expected = off;
        let got1 = data_offset;
        let got2 = data_offset;

        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }
    }

    {
        let n = read_u32(&storage, 4);
        let mut i: U32 = 0;
        while i < n {
            let v = (i + 1) * 3 + 1;
            let offset = data_offset + (i as usize) * 4;
            write_u32(&mut storage, offset, v);
            let val = read_u32(&storage, offset).wrapping_add(0);
            write_u32(&mut storage, offset, val);
            i += 1;
        }
    }

    write_u32(&mut storage, 8, 0);
    {
        let n = read_u32(&storage, 4);
        let mut i: U32 = 0;
        let mut sum: U32 = 0;
        while i < n {
            let offset = data_offset + (i as usize) * 4;
            sum = sum.wrapping_add(read_u32(&storage, offset));
            i += 1;
        }
        write_u32(&mut storage, 8, sum);
    }

    let sum = read_u32(&storage, 8);
    if sum != compute_sum(N) {
        return 3;
    }

    if size_of::<Packet>() < data_offset {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let packet_size = size_of::<Packet>();
    let data_offset = packet_size;

    let mut storage: Vec<U8> = vec![0u8; packet_size];

    write_u32(&mut storage, 0, 0);
    write_u32(&mut storage, 4, 0);
    write_u32(&mut storage, 8, 0);

    {
        let off = data_offset;
        let expected = off;
        let got1 = data_offset;
        let got2 = data_offset;

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