// structure_and_union_spec.rs

const U8: u8 = 0;
const U32: u32 = 0;

macro_rules! offset_of {
    ($type:ty, $member:ident) => {
        std::mem::size_of::<$type>() as u32
    };
}

struct Packet {
    tag: u32,
    n: u32,
    sum: u32,
    data: [u32; 7],
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

    let mut storage = [0u8; 7 * std::mem::size_of::<u32>() + 32];
    let p = &mut Packet {
        tag: 0xA1B2C3D4,
        n: N,
        sum: 0,
        data: [0; 7],
    };

    let expected = storage.as_mut_ptr() as *mut u8;
    let got1 = (p as *const Packet).offset(offsetof(Packet, data)) as *mut u8;
    let got2 = (&*p).offset(offsetof(Packet, data)) as *mut u8;

    if got1 != expected {
        return 1;
    }
    if got2 != expected {
        return 2;
    }

    for i in 0..p.n {
        let v = (i + 1) * 3 + 1;
        p.data[i] = v;
        unsafe { (*p).data[i] += 0; }
    }

    p.sum = 0;
    for i in 0..p.n {
        p.sum += p.data[i];
    }

    if p.sum != compute_sum(N) {
        return 3;
    }

    if std::mem::size_of::<Packet>() < offset_of!(Packet, data) {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage = [0u8; std::mem::size_of::<Packet>()];
    let p = &mut Packet {
        tag: 0,
        n: 0,
        sum: 0,
        data: [0; 0],
    };

    let expected = storage.as_mut_ptr() as *mut u8;
    let got1 = (p as *const Packet).offset(offsetof(Packet, data)) as *mut u8;
    let got2 = (&*p).offset(offsetof(Packet, data)) as *mut u8;

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
        return;
    }

    let r = test_zero_element_object();
    if r != 0 {
        return;
    }

    std::process::exit(0);
}