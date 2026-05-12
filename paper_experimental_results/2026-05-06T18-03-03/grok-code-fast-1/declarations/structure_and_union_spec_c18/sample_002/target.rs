use std::mem::{size_of, offset_of};

type u8 = u8;
type u32 = u32;

#[repr(C)]
struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
}

fn compute_sum(n: u32) -> u32 {
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
    let mut storage: [u8; size_of::<Packet>() + N * size_of::<u32>() + 32] = [0; size_of::<Packet>() + N * 4 + 32];
    let p = storage.as_mut_ptr() as *mut Packet;
    unsafe {
        (*p).tag = 0xA1B2C3D4;
        (*p).n = N as u32;
    }
    let off = offset_of!(Packet, sum) + size_of::<u32>();
    let expected = unsafe { storage.as_ptr().add(off) };
    let got1 = unsafe { (p as *mut u8).add(off) };
    if !std::ptr::eq(got1, expected) {
        return 1;
    }
    let got2 = unsafe { (p as *mut u8).add(off) };
    if !std::ptr::eq(got2, expected) {
        return 2;
    }
    let data_ptr = unsafe { (p as *mut u8).add(off) as *mut u32 };
    let mut i = 0;
    while i < unsafe { (*p).n } {
        let v = (i + 1) * 3 + 1;
        unsafe { *data_ptr.add(i as usize) = v; }
        unsafe { *data_ptr.add(i as usize) += 0; }
        i += 1;
    }
    unsafe { (*p).sum = 0; }
    i = 0;
    while i < unsafe { (*p).n } {
        unsafe { (*p).sum += *data_ptr.add(i as usize); }
        i += 1;
    }
    if unsafe { (*p).sum } != compute_sum(N as u32) {
        return 3;
    }
    if (size_of::<Packet>() as u64) < (off as u64) {
        return 4;
    }
    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: [u8; size_of::<Packet>() + 32] = [0; size_of::<Packet>() + 32];
    let p = storage.as_mut_ptr() as *mut Packet;
    unsafe {
        (*p).tag = 0;
        (*p).n = 0;
        (*p).sum = 0;
    }
    let off = offset_of!(Packet, sum) + size_of::<u32>();
    let expected = unsafe { storage.as_ptr().add(off) };
    let got1 = unsafe { (p as *mut u8).add(off) };
    if !std::ptr::eq(got1, expected) {
        return 10;
    }
    let got2 = unsafe { (p as *mut u8).add(off) };
    if !std::ptr::eq(got2, expected) {
        return 11;
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