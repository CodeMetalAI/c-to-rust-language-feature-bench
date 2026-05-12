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

    // Initialize packet fields
    let tag_bytes = 0xA1B2C3D4u32.to_ne_bytes();
    storage[0..4].copy_from_slice(&tag_bytes);

    let n_bytes = (N as U32).to_ne_bytes();
    storage[4..8].copy_from_slice(&n_bytes);

    // Verify data offset
    {
        let expected_offset = data_offset;
        let got_offset = data_offset;

        if got_offset != expected_offset {
            return 1;
        }
        if got_offset != expected_offset {
            return 2;
        }
    }

    // Fill data array
    {
        let mut i = 0u32;
        while i < N as U32 {
            let v = (i + 1u32) * 3u32 + 1u32;
            let offset = data_offset + (i as usize) * mem::size_of::<U32>();
            let v_bytes = v.to_ne_bytes();
            storage[offset..offset + 4].copy_from_slice(&v_bytes);
            i += 1;
        }
    }

    // Compute sum
    let mut sum = 0u32;
    {
        let mut i = 0u32;
        while i < N as U32 {
            let offset = data_offset + (i as usize) * mem::size_of::<U32>();
            let mut bytes = [0u8; 4];
            bytes.copy_from_slice(&storage[offset..offset + 4]);
            let value = U32::from_ne_bytes(bytes);
            sum += value;
            i += 1;
        }
    }

    // Write sum
    let sum_bytes = sum.to_ne_bytes();
    storage[8..12].copy_from_slice(&sum_bytes);

    if sum != compute_sum(N as U32) {
        return 3;
    }

    if (mem::size_of::<Packet>() as u64) < (data_offset as u64) {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage = [0u8; mem::size_of::<Packet>()];

    let packet_size = mem::size_of::<Packet>();
    let data_offset = packet_size;

    // Initialize packet fields
    let tag_bytes = 0u32.to_ne_bytes();
    storage[0..4].copy_from_slice(&tag_bytes);

    let n_bytes = 0u32.to_ne_bytes();
    storage[4..8].copy_from_slice(&n_bytes);

    let sum_bytes = 0u32.to_ne_bytes();
    storage[8..12].copy_from_slice(&sum_bytes);

    // Verify data offset
    {
        let expected_offset = data_offset;
        let got1_offset = data_offset;
        let got2_offset = data_offset;

        if got1_offset != expected_offset {
            return 10;
        }
        if got2_offset != expected_offset {
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