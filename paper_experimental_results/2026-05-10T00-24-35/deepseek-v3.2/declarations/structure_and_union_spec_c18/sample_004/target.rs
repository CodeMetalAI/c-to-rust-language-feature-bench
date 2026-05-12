use std::mem;

type u8 = u8;
type u32 = u32;

struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 0],
}

fn compute_sum(n: u32) -> u32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += (i + 1u32) * 3u32 + 1u32;
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: u32 =131072;

    // Allocate enough space for Packet header + N data elements + 32 bytes padding
    let total_size = mem::size_of::<Packet>() + N as usize * mem::size_of::<u32>() + 32;
    let mut storage: Vec<u8> = vec![0; total_size];
    
    // Ensure alignment for u32
    let storage_ptr = storage.as_mut_ptr() as usize;
    let aligned_ptr = (storage_ptr + 3) & !3;
    let offset = aligned_ptr - storage_ptr;
    
    // Adjust storage to be properly aligned
    let aligned_storage = &mut storage[offset..];
    let p = aligned_storage.as_mut_ptr() as *mut Packet;
    
    unsafe {
        (*p).tag = 0xA1B2C3D4u32;
        (*p).n = N;
    }
    
    // Calculate offset of data field
    let data_offset = {
        let dummy: Packet = Packet { tag: 0, n: 0, sum: 0, data: [] };
        let base_ptr = &dummy as *const Packet as usize;
        let data_ptr = &dummy.data as *const [u32; 0] as usize;
        data_ptr - base_ptr
    };
    
    // Verify pointer calculations
    {
        let expected = aligned_storage.as_ptr() as usize + data_offset;
        let got1 = unsafe { &(*p).data as *const [u32; 0] as usize };
        let got2 = unsafe { &(*p).data as *const [u32; 0] as usize };
        
        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }
    }
    
    // Fill data array
    unsafe {
        let data_ptr = (*p).data.as_ptr() as *mut u32;
        let mut i = 0;
        while i < (*p).n {
            let v = (i + 1u32) * 3u32 + 1u32;
            *data_ptr.add(i as usize) = v;
            i += 1;
        }
    }
    
    // Calculate sum
    unsafe {
        (*p).sum = 0;
        let data_ptr = (*p).data.as_ptr() as *const u32;
        let mut i = 0;
        while i < (*p).n {
            (*p).sum += *data_ptr.add(i as usize);
            i += 1;
        }
    }
    
    // Verify sum
    if unsafe { (*p).sum } != compute_sum(N) {
        return 3;
    }
    
    // Verify sizeof(Packet) >= offsetof(data)
    if mem::size_of::<Packet>() < data_offset {
        return 4;
    }
    
    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: Vec<u8> = vec![0; mem::size_of::<Packet>()];
    
    // Ensure alignment
    let storage_ptr = storage.as_mut_ptr() as usize;
    let aligned_ptr = (storage_ptr + 3) & !3;
    let offset = aligned_ptr - storage_ptr;
    
    let aligned_storage = &mut storage[offset..];
    let p = aligned_storage.as_mut_ptr() as *mut Packet;
    
    unsafe {
        (*p).tag = 0u32;
        (*p).n = 0u32;
        (*p).sum = 0u32;
    }
    
    // Calculate offset of data field
    let data_offset = {
        let dummy: Packet = Packet { tag: 0, n: 0, sum: 0, data: [] };
        let base_ptr = &dummy as *const Packet as usize;
        let data_ptr = &dummy.data as *const [u32; 0] as usize;
        data_ptr - base_ptr
    };
    
    // Verify pointer calculations
    {
        let expected = aligned_storage.as_ptr() as usize + data_offset;
        let got1 = unsafe { &(*p).data as *const [u32; 0] as usize };
        let got2 = unsafe { &(*p).data as *const [u32; 0] as usize };
        
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