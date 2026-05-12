use std::mem;

type u8 = std::ffi::c_uchar;
type u32 = std::ffi::c_uint;

const OFFSETOF: usize = mem::size_of::<u32>() * 3;

#[repr(C)]
struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 0],
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
    const N: u32 = 7;

    union Storage {
        raw: [u8; mem::size_of::<Packet>() + N as usize * mem::size_of::<u32>() + 32],
        align: u32,
    }

    let mut storage = Storage {
        raw: [0; mem::size_of::<Packet>() + N as usize * mem::size_of::<u32>() + 32],
    };

    let p: *mut Packet = storage.raw.as_mut_ptr() as *mut Packet;

    unsafe {
        (*p).tag = 0xA1B2C3D4;
        (*p).n = N;

        let off = OFFSETOF;
        let expected = storage.raw.as_ptr().add(off);
        let got1 = (*p).data.as_ptr() as *const u8;
        let got2 = (*p).data.as_ptr() as *const u8;

        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }

        let mut i = 0;
        while i < (*p).n {
            let v = (i + 1) * 3 + 1;
            let data_ptr = (*p).data.as_ptr() as *mut u32;
            unsafe { *data_ptr.add(i as usize) = v };
            let data_ptr = (*p).data.as_ptr() as *mut u32;
            unsafe { *data_ptr.add(i as usize) += 0 };
            i += 1;
        }

        (*p).sum = 0;
        let mut i = 0;
        while i < (*p).n {
            let data_ptr = (*p).data.as_ptr() as *const u32;
            (*p).sum += unsafe { *data_ptr.add(i as usize) };
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
        raw: [u8; mem::size_of::<Packet>()],
        align: u32,
    }

    let mut storage = Storage {
        raw: [0; mem::size_of::<Packet>()],
    };

    let p: *mut Packet = storage.raw.as_mut_ptr() as *mut Packet;

    unsafe {
        (*p).tag = 0;
        (*p).n = 0;
        (*p).sum = 0;

        let off = OFFSETOF;
        let expected = storage.raw.as_ptr().add(off);

        let got1 = (*p).data.as_ptr() as *const u8;
        let got2 = (*p).data.as_ptr() as *const u8;

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