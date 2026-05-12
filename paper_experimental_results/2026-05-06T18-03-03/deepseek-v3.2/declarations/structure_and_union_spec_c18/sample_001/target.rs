use std::mem;

type u8 = std::num::u8;
type u32 = std::num::u32;

fn offset_of<T, U>(_: &T, member: &U) -> usize {
    let base = 0 as *const T;
    let member_ptr = unsafe { &(*base).member as *const U };
    member_ptr as usize - base as usize
}

struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 0], // zero-length array to represent flexible array member
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

    union Storage {
        raw: [u8; mem::size_of::<Packet>() + N * mem::size_of::<u32>() + 32],
        align: u32,
    }

    let mut storage = Storage {
        raw: [0; mem::size_of::<Packet>() + N * mem::size_of::<u32>() + 32],
    };

    let p = unsafe { &mut *(storage.raw.as_mut_ptr() as *mut Packet) };

    p.tag = 0xA1B2C3D4;
    p.n = N as u32;

    {
        let off = offset_of(p, &p.data);
        let expected = storage.raw.as_ptr() as usize + off;
        let got1 = &p.data as *const [u32; 0] as usize;
        let got2 = &(*p).data as *const [u32; 0] as usize;

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
            unsafe {
                let data_ptr = p.data.as_ptr() as *mut u32;
                *data_ptr.add(i) = v;
                *data_ptr.add(i) += 0;
            }
            i += 1;
        }
    }

    p.sum = 0;
    {
        let mut i = 0;
        while i < p.n {
            unsafe {
                let data_ptr = p.data.as_ptr() as *const u32;
                p.sum += *data_ptr.add(i);
            }
            i += 1;
        }
    }

    if p.sum != compute_sum(N as u32) {
        return 3;
    }

    if mem::size_of::<Packet>() < offset_of(p, &p.data) {
        return 4;
    }

    return 0;
}

fn test_zero_element_object() -> i32 {
    union Storage {
        raw: [u8; mem::size_of::<Packet>()],
        align: u32,
    }

    let mut storage = Storage {
        raw: [0; mem::size_of::<Packet>()],
    };

    let p = unsafe { &mut *(storage.raw.as_mut_ptr() as *mut Packet) };

    p.tag = 0;
    p.n = 0;
    p.sum = 0;

    {
        let off = offset_of(p, &p.data);
        let expected = storage.raw.as_ptr() as usize + off;

        let got1 = &p.data as *const [u32; 0] as usize;
        let got2 = &(*p).data as *const [u32; 0] as usize;

        if got1 != expected {
            return 10;
        }
        if got2 != expected {
            return 11;
        }
    }

    return 0;
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