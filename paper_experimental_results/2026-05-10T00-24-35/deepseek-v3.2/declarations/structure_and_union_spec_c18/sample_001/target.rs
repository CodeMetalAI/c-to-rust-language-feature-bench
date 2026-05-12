use std::mem;
use std::ptr;

type u8 = std::ffi::c_uchar;
type u32 = std::ffi::c_uint;

#[repr(C)]
struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 0],
}

fn offset_of<T, U>(obj: &T, member: *const U) -> usize {
    unsafe { (member as *const u8).offset_from(obj as *const _ as *const u8) as usize }
}

fn compute_sum(n: u32) -> u32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s = s.wrapping_add((i.wrapping_add(1)).wrapping_mul(3).wrapping_add(1));
        i = i.wrapping_add(1);
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: usize = 7;
    
    #[repr(C, align(4))]
    union Storage {
        raw: [u8; mem::size_of::<Packet>() + N * mem::size_of::<u32>() + 32],
        align: u32,
    }
    
    let mut storage = Storage { raw: [0; mem::size_of::<Packet>() + N * mem::size_of::<u32>() + 32] };
    
    let p_ptr = storage.raw.as_mut_ptr() as *mut Packet;
    
    unsafe {
        (*p_ptr).tag = 0xA1B2C3D4;
        (*p_ptr).n = N as u32;
    }
    
    let off = offset_of(unsafe { &*p_ptr }, &unsafe { &*p_ptr }.data);
    let expected = unsafe { storage.raw.as_ptr().add(off) };
    
    let got1 = unsafe { &(*p_ptr).data } as *const _ as *const u8;
    let got2 = unsafe { &(*p_ptr).data } as *const _ as *const u8;
    
    if got1 != expected {
        return 1;
    }
    if got2 != expected {
        return 2;
    }
    
    unsafe {
        let mut i = 0;
        while i < (*p_ptr).n {
            let v = (i.wrapping_add(1)).wrapping_mul(3).wrapping_add(1);
            ptr::write((*p_ptr).data.as_ptr().add(i as usize) as *mut u32, v);
            ptr::write(
                (*p_ptr).data.as_ptr().add(i as usize) as *mut u32,
                ptr::read((*p_ptr).data.as_ptr().add(i as usize) as *const u32)
                    .wrapping_add(0),
            );
            i = i.wrapping_add(1);
        }
    }
    
    unsafe {
        (*p_ptr).sum = 0;
        let mut i = 0;
        while i < (*p_ptr).n {
            (*p_ptr).sum = (*p_ptr)
                .sum
                .wrapping_add(ptr::read((*p_ptr).data.as_ptr().add(i as usize) as *const u32));
            i = i.wrapping_add(1);
        }
    }
    
    if unsafe { (*p_ptr).sum } != compute_sum(N as u32) {
        return 3;
    }
    
    if mem::size_of::<Packet>() < off {
        return 4;
    }
    
    0
}

fn test_zero_element_object() -> i32 {
    #[repr(C, align(4))]
    union Storage {
        raw: [u8; mem::size_of::<Packet>()],
        align: u32,
    }
    
    let mut storage = Storage {
        raw: [0; mem::size_of::<Packet>()],
    };
    
    let p_ptr = storage.raw.as_mut_ptr() as *mut Packet;
    
    unsafe {
        (*p_ptr).tag = 0;
        (*p_ptr).n = 0;
        (*p_ptr).sum = 0;
    }
    
    let off = offset_of(unsafe { &*p_ptr }, &unsafe { &*p_ptr }.data);
    let expected = unsafe { storage.raw.as_ptr().add(off) };
    
    let got1 = unsafe { &(*p_ptr).data } as *const _ as *const u8;
    let got2 = unsafe { &(*p_ptr).data } as *const _ as *const u8;
    
    if got1 != expected {
        return 10;
    }
    if got2 != expected {
        return 11;
    }
    
    0
}

fn main() {
    let mut r;
    
    r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r);
    }
    
    r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }
    
    std::process::exit(0);
}