use std::mem;

type U8 = u8;
type U32 = u32;

#[repr(C)]
struct Packet {
    tag: U32,
    n: U32,
    sum: U32,
    data: [U32; 0],
}

fn offsetof<T>(member: fn(&T) -> &U32) -> usize {
    let t: T = unsafe { mem::zeroed() };
    let b = &t as *const _ as *const u8;
    let m = member(&t) as *const _ as *const u8;
    (m as usize) - (b as usize)
}

fn compute_sum(n: U32) -> U32 {
    let mut s = 0u32;
    let mut i = 0u32;
    while i < n {
        s += (i + 1) * 3 + 1;
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: usize = 7;
    const STORAGE_SIZE: usize = mem::size_of::<Packet>() + N * mem::size_of::<U32>() + 32;

    let mut storage: [U8; STORAGE_SIZE] = [0; STORAGE_SIZE];
    let p = storage.as_mut_ptr() as *mut Packet;

    unsafe {
        (*p).tag = 0xA1B2C3D4;
        (*p).n = N as U32;

        let off = offsetof(|packet: &Packet| &packet.data[0]);
        let expected = storage.as_ptr().offset(off as isize);
        let got1 = &(*p).data as *const _ as *const U8;
        let got2 = &(*p).data as *const _ as *const U8;

        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }

        let mut i = 0;
        while i < (*p).n {
            let v = (i + 1) * 3 + 1;
            (*p).data.as_mut_ptr().offset(i as isize).write(v);
            i += 1;
        }

        (*p).sum = 0;
        let mut i = 0;
        while i < (*p).n {
            (*p).sum += *(*p).data.as_ptr().offset(i as isize);
            i += 1;
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
    let mut storage: [U8; mem::size_of::<Packet>()] = [0; mem::size_of::<Packet>()];
    let p = storage.as_mut_ptr() as *mut Packet;

    unsafe {
        (*p).tag = 0;
        (*p).n = 0;
        (*p).sum = 0;

        let off = offsetof(|packet: &Packet| &packet.data[0]);
        let expected = storage.as_ptr().offset(off as isize);

        let got1 = &(*p).data as *const _ as *const U8;
        let got2 = &(*p).data as *const _ as *const U8;

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