use std::mem;

type U8 = u8;
type U32 = u32;

#[repr(C)]
struct Packet {
    tag: U32,
    n: U32,
    sum: U32,
    data: [U32], // Using flexible array member equivalent in Rust
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
    let p: *mut Packet = storage.as_mut_ptr() as *mut Packet;

    unsafe {
        (*p).tag = 0xA1B2C3D4;
        (*p).n = N as U32;

        let off = mem::size_of::<U32>() * 3;
        let expected = storage.as_ptr().add(off);
        let got1 = &mut (*p).data.as_mut_ptr();
        let got2 = &mut (*p).data.as_mut_ptr();

        if got1 != expected as *mut U32 || got2 != expected as *mut U32 {
            return 1;
        }

        for i in 0..(*p).n as usize {
            let v = (i as U32 + 1) * 3 + 1;
            *(*p).data.as_mut_ptr().add(i) = v;
        }

        (*p).sum = 0;
        for i in 0..(*p).n as usize {
            (*p).sum += *(*p).data.as_mut_ptr().add(i);
        }

        if (*p).sum != compute_sum(N as U32) {
            return 3;
        }

        if mem::size_of::<Packet>() < off {
            return 4;
        }
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage = vec![0u8; mem::size_of::<Packet>()];
    let p: *mut Packet = storage.as_mut_ptr() as *mut Packet;

    unsafe {
        (*p).tag = 0;
        (*p).n = 0;
        (*p).sum = 0;

        let off = mem::size_of::<U32>() * 3;
        let expected = storage.as_ptr().add(off);
        let got1 = &mut (*p).data.as_mut_ptr();
        let got2 = &mut (*p).data.as_mut_ptr();

        if got1 != expected as *mut U32 || got2 != expected as *mut U32 {
            return 10;
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
}