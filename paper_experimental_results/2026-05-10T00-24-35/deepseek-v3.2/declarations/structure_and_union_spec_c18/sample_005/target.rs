use std::mem;

type U8 = u8;
type U32 = u32;

const OFFSETOF: fn() -> usize = || {
    #[repr(C)]
    struct Packet {
        tag: U32,
        n: U32,
        sum: U32,
        data: [U32; 0],
    }
    unsafe { mem::offset_of!(Packet, data) }
};

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
    const N: U32 = 7;

    #[repr(C, align(4))]
    union Storage {
        raw: [U8; mem::size_of::<U32>() * 3 + N as usize * mem::size_of::<U32>() + 32],
        align: U32,
    }

    let mut storage = Storage {
        raw: [0; mem::size_of::<U32>() * 3 + N as usize * mem::size_of::<U32>() + 32],
    };

    let p = unsafe { &mut *(storage.raw.as_mut_ptr() as *mut _) };

    p.tag = 0xA1B2C3D4;
    p.n = N;

    {
        let off = OFFSETOF();
        let expected = unsafe { storage.raw.as_ptr().add(off) };
        let got1 = p.data.as_ptr() as *const U8;
        let got2 = p.data.as_ptr() as *const U8;

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
                *p.data.as_mut_ptr().add(i as usize) = v;
            }
            i += 1;
        }
    }

    p.sum = 0;
    {
        let mut i = 0;
        while i < p.n {
            unsafe {
                p.sum += *p.data.as_ptr().add(i as usize);
            }
            i += 1;
        }
    }

    if p.sum != compute_sum(N) {
        return 3;
    }

    if mem::size_of::<U32>() * 3 < OFFSETOF() {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    #[repr(C, align(4))]
    union Storage {
        raw: [U8; mem::size_of::<U32>() * 3],
        align: U32,
    }

    let mut storage = Storage {
        raw: [0; mem::size_of::<U32>() * 3],
    };

    let p = unsafe { &mut *(storage.raw.as_mut_ptr() as *mut _) };

    p.tag = 0;
    p.n = 0;
    p.sum = 0;

    {
        let off = OFFSETOF();
        let expected = unsafe { storage.raw.as_ptr().add(off) };
        let got1 = p.data.as_ptr() as *const U8;
        let got2 = p.data.as_ptr() as *const U8;

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
}