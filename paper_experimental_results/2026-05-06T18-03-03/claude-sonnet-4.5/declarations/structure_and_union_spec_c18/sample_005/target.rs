use std::mem;
use std::ptr;

type U8 = u8;
type U32 = u32;

#[repr(C)]
struct Packet {
    tag: U32,
    n: U32,
    sum: U32,
}

fn compute_sum(n: U32) -> U32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += (i + 1) * 3 + 1;
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: usize = 7;

    let mut storage: [U8; mem::size_of::<Packet>() + N * mem::size_of::<U32>() + 32] = [0; mem::size_of::<Packet>() + N * mem::size_of::<U32>() + 32];

    let p_offset = 0;
    let data_offset = mem::size_of::<Packet>();

    // Write packet header
    let tag_bytes = 0xA1B2C3D4u32.to_ne_bytes();
    storage[p_offset..p_offset + 4].copy_from_slice(&tag_bytes);
    
    let n_bytes = (N as U32).to_ne_bytes();
    storage[p_offset + 4..p_offset + 8].copy_from_slice(&n_bytes);

    // Check data offset
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
        let mut i = 0;
        while i < N {
            let v = ((i + 1) * 3 + 1) as U32;
            let v_bytes = v.to_ne_bytes();
            let offset = data_offset + i * mem::size_of::<U32>();
            storage[offset..offset + 4].copy_from_slice(&v_bytes);
            i += 1;
        }
    }

    // Compute sum
    let mut sum = 0u32;
    {
        let mut i = 0;
        while i < N {
            let offset = data_offset + i * mem::size_of::<U32>();
            let mut val_bytes = [0u8; 4];
            val_bytes.copy_from_slice(&storage[offset..offset + 4]);
            let val = U32::from_ne_bytes(val_bytes);
            sum += val;
            i += 1;
        }
    }

    // Write sum
    let sum_bytes = sum.to_ne_bytes();
    storage[p_offset + 8..p_offset + 12].copy_from_slice(&sum_bytes);

    if sum != compute_sum(N as U32) {
        return 3;
    }

    if mem::size_of::<Packet>() < data_offset {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: [U8; mem::size_of::<Packet>()] = [0; mem::size_of::<Packet>()];

    let p_offset = 0;
    let data_offset = mem::size_of::<Packet>();

    // Write packet header with zeros
    let tag_bytes = 0u32.to_ne_bytes();
    storage[p_offset..p_offset + 4].copy_from_slice(&tag_bytes);
    
    let n_bytes = 0u32.to_ne_bytes();
    storage[p_offset + 4..p_offset + 8].copy_from_slice(&n_bytes);
    
    let sum_bytes = 0u32.to_ne_bytes();
    storage[p_offset + 8..p_offset + 12].copy_from_slice(&sum_bytes);

    // Check data offset
    {
        let expected_offset = data_offset;
        let got_offset = data_offset;

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
        std::process::exit(r);
    }

    let r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }

    std::process::exit(0);
}