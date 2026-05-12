use std::mem;
use std::process;

type U8 = u8;
type U32 = u32;

#[repr(C)]
struct Packet {
    tag: U32,
    n: U32,
    sum: U32,
}

fn compute_sum(n: U32) -> U32 {
    let mut s = 0u32;
    let mut i = 0u32;
    while i < n {
        s += (i + 1u32) * 3u32 + 1u32;
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: usize = 7;

    let mut storage = [0u8; mem::size_of::<Packet>() + N * mem::size_of::<U32>() + 32];

    let packet_size = mem::size_of::<Packet>();
    let data_offset = packet_size;

    storage[0..4].copy_from_slice(&0xA1B2C3D4u32.to_ne_bytes());
    storage[4..8].copy_from_slice(&(N as U32).to_ne_bytes());

    {
        let expected_offset = data_offset;
        let got_offset = packet_size;

        if got_offset != expected_offset {
            return 1;
        }
        if got_offset != expected_offset {
            return 2;
        }
    }

    {
        let mut i = 0u32;
        while i < N as U32 {
            let v = (i + 1u32) * 3u32 + 1u32;
            let idx = data_offset + (i as usize) * 4;
            storage[idx..idx + 4].copy_from_slice(&v.to_ne_bytes());
            i += 1;
        }
    }

    let mut sum = 0u32;
    {
        let mut i = 0u32;
        while i < N as U32 {
            let idx = data_offset + (i as usize) * 4;
            let v = U32::from_ne_bytes([
                storage[idx],
                storage[idx + 1],
                storage[idx + 2],
                storage[idx + 3],
            ]);
            sum += v;
            i += 1;
        }
    }

    storage[8..12].copy_from_slice(&sum.to_ne_bytes());

    if sum != compute_sum(N as U32) {
        return 3;
    }

    if mem::size_of::<Packet>() < data_offset {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage = [0u8; mem::size_of::<Packet>()];

    let packet_size = mem::size_of::<Packet>();
    let data_offset = packet_size;

    storage[0..4].copy_from_slice(&0u32.to_ne_bytes());
    storage[4..8].copy_from_slice(&0u32.to_ne_bytes());
    storage[8..12].copy_from_slice(&0u32.to_ne_bytes());

    {
        let expected_offset = data_offset;
        let got_offset = packet_size;

        if got_offset != expected_offset {
            return 10;
        }
        if got_offset != expected_offset {
            return 11;
        }
    }

    0
}

fn main() {
    let r = test_nonempty_object();
    if r != 0 {
        process::exit(r);
    }

    let r = test_zero_element_object();
    if r != 0 {
        process::exit(r);
    }

    process::exit(0);
}