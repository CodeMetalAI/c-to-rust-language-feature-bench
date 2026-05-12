use std::mem;

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
    const N: usize = 7;

    let mut storage: [u8; mem::size_of::<Packet>() + N * mem::size_of::<u32>() + 32] = [0; mem::size_of::<Packet>() + N * mem::size_of::<u32>() + 32];

    let p = unsafe { &mut *(storage.as_mut_ptr() as *mut Packet) };

    p.tag = 0xA1B2C3D4;
    p.n = N as u32;

    {
        let off = mem::size_of_val(&p.tag) + mem::size_of_val(&p.n) + mem::size_of_val(&p.sum);
        let expected = unsafe { storage.as_ptr().add(off) };
        let got1 = p.data.as_ptr() as *const u8;
        let got2 = unsafe { &(*p).data }.as_ptr() as *const u8;

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
            p.data[i as usize] = v;
            unsafe { (*p).data[i as usize] += 0 };
            i += 1;
        }
    }

    p.sum = 0;
    {
        let mut i = 0;
        while i < p.n {
            p.sum += p.data[i as usize];
            i += 1;
        }
    }

    if p.sum != compute_sum(N as u32) {
        return 3;
    }

    if mem::size_of::<Packet>() < mem::size_of_val(&p.tag) + mem::size_of_val(&p.n) + mem::size_of_val(&p.sum) {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: [u8; mem::size_of::<Packet>()] = [0; mem::size_of::<Packet>()];

    let p = unsafe { &mut *(storage.as_mut_ptr() as *mut Packet) };

    p.tag = 0;
    p.n = 0;
    p.sum = 0;

    {
        let off = mem::size_of_val(&p.tag) + mem::size_of_val(&p.n) + mem::size_of_val(&p.sum);
        let expected = unsafe { storage.as_ptr().add(off) };
        let got1 = p.data.as_ptr() as *const u8;
        let got2 = unsafe { &(*p).data }.as_ptr() as *const u8;

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