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

    let mut storage: [U32; (mem::size_of::<Packet>() + N * mem::size_of::<U32>() + 32) / mem::size_of::<U32>() + 1] = [0; 64];
    
    let packet_size = mem::size_of::<Packet>();
    let data_offset = packet_size;
    
    let raw_ptr = storage.as_mut_ptr() as *mut U8;
    
    let tag_ptr = raw_ptr as *mut U32;
    let n_ptr = raw_ptr.wrapping_add(4) as *mut U32;
    let sum_ptr = raw_ptr.wrapping_add(8) as *mut U32;
    let data_ptr = raw_ptr.wrapping_add(data_offset) as *mut U32;
    
    let tag = tag_ptr as usize;
    let n = n_ptr as usize;
    let sum = sum_ptr as usize;
    let data = data_ptr as usize;
    
    storage[(tag - storage.as_ptr() as usize) / 4] = 0xA1B2C3D4u32;
    storage[(n - storage.as_ptr() as usize) / 4] = N as U32;
    
    {
        let expected = raw_ptr.wrapping_add(data_offset);
        let got1 = data_ptr as *mut U8;
        let got2 = data_ptr as *mut U8;
        
        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }
    }
    
    {
        let mut i = 0u32;
        let n_val = storage[(n - storage.as_ptr() as usize) / 4];
        while i < n_val {
            let v = (i + 1u32) * 3u32 + 1u32;
            let idx = (data - storage.as_ptr() as usize) / 4 + i as usize;
            storage[idx] = v;
            storage[idx] += 0u32;
            i += 1;
        }
    }
    
    storage[(sum - storage.as_ptr() as usize) / 4] = 0;
    {
        let mut i = 0u32;
        let n_val = storage[(n - storage.as_ptr() as usize) / 4];
        while i < n_val {
            let data_idx = (data - storage.as_ptr() as usize) / 4 + i as usize;
            let sum_idx = (sum - storage.as_ptr() as usize) / 4;
            storage[sum_idx] += storage[data_idx];
            i += 1;
        }
    }
    
    let sum_val = storage[(sum - storage.as_ptr() as usize) / 4];
    if sum_val != compute_sum(N as U32) {
        return 3;
    }
    
    if mem::size_of::<Packet>() < data_offset {
        return 4;
    }
    
    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: [U32; (mem::size_of::<Packet>() + mem::size_of::<U32>() - 1) / mem::size_of::<U32>() + 1] = [0; 4];
    
    let packet_size = mem::size_of::<Packet>();
    let data_offset = packet_size;
    
    let raw_ptr = storage.as_mut_ptr() as *mut U8;
    
    let tag_ptr = raw_ptr as *mut U32;
    let n_ptr = raw_ptr.wrapping_add(4) as *mut U32;
    let sum_ptr = raw_ptr.wrapping_add(8) as *mut U32;
    let data_ptr = raw_ptr.wrapping_add(data_offset) as *mut U8;
    
    let tag = tag_ptr as usize;
    let n = n_ptr as usize;
    let sum = sum_ptr as usize;
    
    storage[(tag - storage.as_ptr() as usize) / 4] = 0u32;
    storage[(n - storage.as_ptr() as usize) / 4] = 0u32;
    storage[(sum - storage.as_ptr() as usize) / 4] = 0u32;
    
    {
        let expected = raw_ptr.wrapping_add(data_offset);
        let got1 = data_ptr;
        let got2 = data_ptr;
        
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
        process::exit(r);
    }
    
    let r = test_zero_element_object();
    if r != 0 {
        process::exit(r);
    }
    
    process::exit(0);
}