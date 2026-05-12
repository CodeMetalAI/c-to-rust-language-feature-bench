// structure_and_union_spec.rs
use std::mem;

const ZERO: u8 = 0;

#[repr(C)]
struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 7], // N is defined as 7 in the original C code
}

fn compute_sum(n: u32) -> u32 {
    let mut s = 0;
    let mut i = 0;
    while i < n {
        s += (i + 1u32) * 3u32 + 1u32;
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    let storage = [0u8; 40]; // sizeof(Packet) + N * sizeof(u32) + 32u
    let p: *mut Packet = storage.as_mut_ptr() as *mut Packet;

    p.as_mut().unwrap().tag = 0xA1B2C3D4u;
    p.as_mut().unwrap().n = 7;

    let off = std::mem::offset_of!(Packet, data);
    let expected = storage.as_ptr().add(off as usize);
    let got1 = (&p.as_mut().unwrap().data) as *const u8;
    let got2 = (&(*p).data) as *const u8;

    if got1!= expected {
        return 1;
    }
    if got2!= expected {
        return 2;
    }

    let mut i = 0;
    while i < p.as_mut().unwrap().n {
        let v = (i + 1u32) * 3u32 + 1u32;
        p.as_mut().unwrap().data[i] = v;
        (*p).data[i] += 0u32;
        i += 1;
    }

    p.as_mut().unwrap().sum = 0;
    i = 0;
    while i < p.as_mut().unwrap().n {
        p.as_mut().unwrap().sum += p.as_mut().unwrap().data[i];
        i += 1;
    }

    if p.as_mut().unwrap().sum!= compute_sum(7) {
        return 3;
    }

    if std::mem::size_of::<Packet>() < off as usize {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let storage = [0u8; 16]; // sizeof(Packet)
    let p: *mut Packet = storage.as_mut_ptr() as *mut Packet;

    p.as_mut().unwrap().tag = 0u;
    p.as_mut().unwrap().n = 0u;
    p.as_mut().unwrap().sum = 0u;

    let off = std::mem::offset_of!(Packet, data);
    let expected = storage.as_ptr().add(off as usize);

    let got1 = (&p.as_mut().unwrap().data) as *const u8;
    let got2 = (&(*p).data) as *const u8;

    if got1!= expected {
        return 10;
    }
    if got2!= expected {
        return 11;
    }

    0
}

fn main() {
    let r = test_nonempty_object();
    if r!= 0 {
        std::process::exit(r);
    }

    let r = test_zero_element_object();
    if r!= 0 {
        std::process::exit(r);
    }

    std::process::exit(0);
}