use std::mem::{self, offset_of};

type U8 = u8;
type U32 = u32;

const OFFSETOF: usize = offset_of!(PacketHeader, sum) + mem::size_of::<U32>();

#[repr(C)]
struct PacketHeader {
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
    const STORAGE_SIZE: usize = mem::size_of::<PacketHeader>() + N * mem::size_of::<U32>() + 32;

    let mut storage: [U8; STORAGE_SIZE] = [0; STORAGE_SIZE];
    let mut align: U32 = 0; // for alignment, but in Rust, the array is aligned

    let p: *mut PacketHeader = storage.as_mut_ptr() as *mut PacketHeader;

    unsafe {
        (*p).tag = 0xA1B2C3D4;
        (*p).n = N as U32;

        let off = OFFSETOF;
        let expected = storage.as_ptr().add(off);
        let got1 = (&mut (*p).sum as *mut U32 as *mut U8).add(mem::size_of::<U32>());
        let got2 = (&mut (*p).sum as *mut U32 as *mut U8).add(mem::size_of::<U32>());

        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }

        let data_ptr = (p as *mut U8).add(OFFSETOF) as *mut U32;
        let mut i = 0;
        while i < (*p).n {
            let v = (i + 1) * 3 + 1;
            *data_ptr.add(i as usize) = v;
            *data_ptr.add(i as usize) += 0;
            i += 1;
        }

        (*p).sum = 0;
        let mut i = 0;
        while i < (*p).n {
            (*p).sum += *data_ptr.add(i as usize);
            i += 1;
        }
    }

    if unsafe { (*p).sum } != compute_sum(N as U32) {
        return 3;
    }

    if mem::size_of::<PacketHeader>() < OFFSETOF {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    const STORAGE_SIZE: usize = mem::size_of::<PacketHeader>();

    let mut storage: [U8; STORAGE_SIZE] = [0; STORAGE_SIZE];

    let p: *mut PacketHeader = storage.as_mut_ptr() as *mut PacketHeader;

    unsafe {
        (*p).tag = 0;
        (*p).n = 0;
        (*p).sum = 0;

        let off = OFFSETOF;
        let expected = storage.as_ptr().add(off);
        let got1 = (&mut (*p).sum as *mut U32 as *mut U8).add(mem::size_of::<U32>());
        let got2 = (&mut (*p).sum as *mut U32 as *mut U8).add(mem::size_of::<U32>());

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