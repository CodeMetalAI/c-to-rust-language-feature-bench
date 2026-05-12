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

    let mut storage: [U32; (mem::size_of::<Packet>() + N * mem::size_of::<U32>() + 32) / mem::size_of::<U32>() + 1] = [0; (mem::size_of::<Packet>() + N * mem::size_of::<U32>() + 32) / mem::size_of::<U32>() + 1];

    let raw_ptr = storage.as_mut_ptr() as *mut U8;
    let packet_size = mem::size_of::<Packet>();
    
    // Write packet header
    let tag_ptr = raw_ptr as *mut U32;
    let n_ptr = unsafe { tag_ptr.add(1) };
    let sum_ptr = unsafe { tag_ptr.add(2) };
    let data_ptr = unsafe { tag_ptr.add(3) };
    
    unsafe {
        *tag_ptr = 0xA1B2C3D4;
        *n_ptr = N as U32;
    }

    // Check offset of data field
    {
        let off = mem::size_of::<U32>() * 3;
        let expected = unsafe { raw_ptr.add(off) };
        let got1 = data_ptr as *mut U8;
        let got2 = data_ptr as *mut U8;

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
        let n_val = unsafe { *n_ptr };
        while i < n_val {
            let v = (i + 1) * 3 + 1;
            unsafe {
                *data_ptr.add(i as usize) = v;
                *data_ptr.add(i as usize) += 0;
            }
            i += 1;
        }
    }

    // Compute sum
    unsafe {
        *sum_ptr = 0;
    }
    {
        let mut i = 0;
        let n_val = unsafe { *n_ptr };
        while i < n_val {
            unsafe {
                *sum_ptr += *data_ptr.add(i as usize);
            }
            i += 1;
        }
    }

    let sum_val = unsafe { *sum_ptr };
    if sum_val != compute_sum(N as U32) {
        return 3;
    }

    let sizeof_packet = mem::size_of::<Packet>();
    let offsetof_data = mem::size_of::<U32>() * 3;
    if sizeof_packet < offsetof_data {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: [U32; (mem::size_of::<Packet>() / mem::size_of::<U32>()) + 1] = [0; (mem::size_of::<Packet>() / mem::size_of::<U32>()) + 1];

    let raw_ptr = storage.as_mut_ptr() as *mut U8;
    
    let tag_ptr = raw_ptr as *mut U32;
    let n_ptr = unsafe { tag_ptr.add(1) };
    let sum_ptr = unsafe { tag_ptr.add(2) };
    let data_ptr = unsafe { tag_ptr.add(3) };

    unsafe {
        *tag_ptr = 0;
        *n_ptr = 0;
        *sum_ptr = 0;
    }

    {
        let off = mem::size_of::<U32>() * 3;
        let expected = unsafe { raw_ptr.add(off) };

        let got1 = data_ptr as *mut U8;
        let got2 = data_ptr as *mut U8;

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