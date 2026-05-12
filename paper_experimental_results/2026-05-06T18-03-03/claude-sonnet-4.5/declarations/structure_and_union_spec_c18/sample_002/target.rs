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
    let tag: U32 = 0xA1B2C3D4;
    let n: U32 = N as U32;
    
    storage[p_offset..p_offset + 4].copy_from_slice(&tag.to_ne_bytes());
    storage[p_offset + 4..p_offset + 8].copy_from_slice(&n.to_ne_bytes());

    // Check offset of data field
    {
        let off = mem::size_of::<Packet>();
        let expected = p_offset + off;
        let got1 = p_offset + data_offset;
        let got2 = p_offset + data_offset;

        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }
    }

    // Fill data array
    {
        let mut i = 0;
        while i < n {
            let v = (i + 1) * 3 + 1;
            let idx = data_offset + (i as usize) * mem::size_of::<U32>();
            storage[idx..idx + 4].copy_from_slice(&v.to_ne_bytes());
            i += 1;
        }
    }

    // Compute sum
    let mut sum: U32 = 0;
    {
        let mut i = 0;
        while i < n {
            let idx = data_offset + (i as usize) * mem::size_of::<U32>();
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

    // Write sum
    storage[p_offset + 8..p_offset + 12].copy_from_slice(&sum.to_ne_bytes());

    if sum != compute_sum(N as U32) {
        return 3;
    }

    if mem::size_of::<Packet>() < mem::size_of::<Packet>() {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: [U8; mem::size_of::<Packet>()] = [0; mem::size_of::<Packet>()];

    let p_offset = 0;
    let data_offset = mem::size_of::<Packet>();

    // Write packet header
    let tag: U32 = 0;
    let n: U32 = 0;
    let sum: U32 = 0;

    storage[p_offset..p_offset + 4].copy_from_slice(&tag.to_ne_bytes());
    storage[p_offset + 4..p_offset + 8].copy_from_slice(&n.to_ne_bytes());
    storage[p_offset + 8..p_offset + 12].copy_from_slice(&sum.to_ne_bytes());

    // Check offset of data field
    {
        let off = mem::size_of::<Packet>();
        let expected = p_offset + off;
        let got1 = p_offset + data_offset;
        let got2 = p_offset + data_offset;

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