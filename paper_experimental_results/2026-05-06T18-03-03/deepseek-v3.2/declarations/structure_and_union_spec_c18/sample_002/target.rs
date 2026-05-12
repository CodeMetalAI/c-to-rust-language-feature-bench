use std::mem;

type U8 = u8;
type U32 = u32;

const OFFSETOF: usize = mem::size_of::<U32>() * 3;

struct Packet {
    tag: U32,
    n: U32,
    sum: U32,
    data: [U32; 0],
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
    const N: U32 = 7;

    union Storage {
        raw: [U8; mem::size_of::<Packet>() + (N as usize) * mem::size_of::<U32>() + 32],
        align: U32,
    }

    let mut storage = Storage {
        raw: [0; mem::size_of::<Packet>() + (N as usize) * mem::size_of::<U32>() + 32],
    };

    let p: *mut Packet = storage.raw.as_mut_ptr() as *mut Packet;

    unsafe {
        (*p).tag = 0xA1B2C3D4;
        (*p).n = N;

        let expected = storage.raw.as_ptr().add(OFFSETOF);
        let got1 = (*p).data.as_ptr() as *const U8;
        let got2 = (*p).data.as_ptr() as *const U8;

        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }

        let data_ptr = (*p).data.as_ptr() as *mut U32;
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

        if (*p).sum != compute_sum(N) {
            return 3;
        }

        if mem::size_of::<Packet>() < OFFSETOF {
            return 4;
        }
    }

    0
}

fn test_zero_element_object() -> i32 {
    union Storage {
        raw: [U8; mem::size_of::<Packet>()],
        align: U32,
    }

    let mut storage = Storage {
        raw: [0; mem::size_of::<Packet>()],
    };

    let p: *mut Packet = storage.raw.as_mut_ptr() as *mut Packet;

    unsafe {
        (*p).tag = 0;
        (*p).n = 0;
        (*p).sum = 0;

        let expected = storage.raw.as_ptr().add(OFFSETOF);
        let got1 = (*p).data.as_ptr() as *const U8;
        let got2 = (*p).data.as_ptr() as *const U8;

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