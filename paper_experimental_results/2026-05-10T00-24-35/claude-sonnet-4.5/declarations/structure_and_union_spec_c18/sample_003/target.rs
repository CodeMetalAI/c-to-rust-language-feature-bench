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

    let mut storage = vec![0u8; mem::size_of::<Packet>() + N * mem::size_of::<U32>() + 32];

    let p_ptr = storage.as_mut_ptr() as *mut Packet;
    let data_ptr = storage.as_mut_ptr().wrapping_add(mem::size_of::<Packet>()) as *mut U32;

    let p = &mut *p_ptr;
    p.tag = 0xA1B2C3D4;
    p.n = N as U32;

    {
        let off = mem::size_of::<Packet>();
        let expected = storage.as_ptr().wrapping_add(off);
        let got1 = data_ptr as *const u8;
        let got2 = data_ptr as *const u8;

        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }
    }

    {
        let mut i = 0;
        while i < p.n {
            let v = (i + 1) * 3 + 1;
            *data_ptr.wrapping_add(i as usize) = v;
            *data_ptr.wrapping_add(i as usize) += 0;
            i += 1;
        }
    }

    p.sum = 0;
    {
        let mut i = 0;
        while i < p.n {
            p.sum += *data_ptr.wrapping_add(i as usize);
            i += 1;
        }
    }

    if p.sum != compute_sum(N as U32) {
        return 3;
    }

    if mem::size_of::<Packet>() < mem::size_of::<Packet>() {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage = vec![0u8; mem::size_of::<Packet>()];

    let p_ptr = storage.as_mut_ptr() as *mut Packet;
    let data_ptr = storage.as_mut_ptr().wrapping_add(mem::size_of::<Packet>()) as *mut U32;

    let p = &mut *p_ptr;
    p.tag = 0;
    p.n = 0;
    p.sum = 0;

    {
        let off = mem::size_of::<Packet>();
        let expected = storage.as_ptr().wrapping_add(off);

        let got1 = data_ptr as *const u8;
        let got2 = data_ptr as *const u8;

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