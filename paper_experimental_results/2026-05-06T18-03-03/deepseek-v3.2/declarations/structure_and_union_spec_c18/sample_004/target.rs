use std::mem;

type u8 = std::num::Wrapping<u8>;
type u32 = std::num::Wrapping<u32>;

fn offset_of<T, U>(_: &T, member: &U) -> usize {
    let base = 0 as *const T;
    let member_ptr = unsafe { &(*base).member as *const U };
    member_ptr as usize - base as usize
}

struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 0],
}

fn compute_sum(n: u32) -> u32 {
    let mut s = u32(0);
    let mut i = u32(0);
    while i < n {
        s += (i + u32(1)) * u32(3) + u32(1);
        i += u32(1);
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: u32 = u32(7);

    union Storage {
        raw: [u8; mem::size_of::<Packet>() + (N.0 as usize) * mem::size_of::<u32>() + 32],
        align: u32,
    }

    let mut storage = Storage {
        raw: [u8(0); mem::size_of::<Packet>() + (N.0 as usize) * mem::size_of::<u32>() + 32],
    };

    let p: *mut Packet = unsafe { &mut storage.raw as *mut u8 as *mut Packet };

    unsafe {
        p.as_mut().unwrap().tag = u32(0xA1B2C3D4);
        p.as_mut().unwrap().n = N;
    }

    let off = offset_of(&Packet { tag: u32(0), n: u32(0), sum: u32(0), data: [] }, &Packet { tag: u32(0), n: u32(0), sum: u32(0), data: [] }.data);
    let expected = unsafe { storage.raw.as_ptr() as usize + off };
    let got1 = unsafe { &p.as_ref().unwrap().data as *const [u32; 0] as usize };
    let got2 = unsafe { &(*p).data as *const [u32; 0] as usize };

    if got1 != expected {
        return 1;
    }
    if got2 != expected {
        return 2;
    }

    unsafe {
        let mut i = u32(0);
        while i < p.as_ref().unwrap().n {
            let v = (i + u32(1)) * u32(3) + u32(1);
            p.as_mut().unwrap().data.as_mut_ptr().add(i.0 as usize).write(v);
            (*p).data.as_mut_ptr().add(i.0 as usize).write(v + u32(0));
            i += u32(1);
        }
    }

    unsafe {
        p.as_mut().unwrap().sum = u32(0);
        let mut i = u32(0);
        while i < p.as_ref().unwrap().n {
            p.as_mut().unwrap().sum += *p.as_ref().unwrap().data.as_ptr().add(i.0 as usize);
            i += u32(1);
        }
    }

    unsafe {
        if p.as_ref().unwrap().sum != compute_sum(N) {
            return 3;
        }
    }

    if mem::size_of::<Packet>() < off {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    union Storage {
        raw: [u8; mem::size_of::<Packet>()],
        align: u32,
    }

    let mut storage = Storage {
        raw: [u8(0); mem::size_of::<Packet>()],
    };

    let p: *mut Packet = unsafe { &mut storage.raw as *mut u8 as *mut Packet };

    unsafe {
        p.as_mut().unwrap().tag = u32(0);
        p.as_mut().unwrap().n = u32(0);
        p.as_mut().unwrap().sum = u32(0);
    }

    let off = offset_of(&Packet { tag: u32(0), n: u32(0), sum: u32(0), data: [] }, &Packet { tag: u32(0), n: u32(0), sum: u32(0), data: [] }.data);
    let expected = unsafe { storage.raw.as_ptr() as usize + off };
    let got1 = unsafe { &p.as_ref().unwrap().data as *const [u32; 0] as usize };
    let got2 = unsafe { &(*p).data as *const [u32; 0] as usize };

    if got1 != expected {
        return 10;
    }
    if got2 != expected {
        return 11;
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