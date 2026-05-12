use std::mem;

type U8 = u8;
type U32 = u32;

const N: usize = 7;

#[repr(C)]
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
    let mut storage: [U8; mem::size_of::<Packet>() + N * mem::size_of::<U32>() + 32] = [0; mem::size_of::<Packet>() + N * mem::size_of::<U32>() + 32];
    let p = unsafe { &mut *(storage.as_mut_ptr() as *mut Packet) };

    p.tag = 0xA1B2C3D4;
    p.n = N as U32;

    {
        let off = mem::size_of_val(&p.tag) + mem::size_of_val(&p.n) + mem::size_of_val(&p.sum);
        let expected = &mut storage[off];
        let got1 = unsafe { &mut *(&mut p.data as *mut [U32; 0] as *mut U8) };
        let got2 = unsafe { &mut *(&mut (*p).data as *mut [U32; 0] as *mut U8) };

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
            unsafe { *p.data.get_unchecked_mut(i as usize) = v };
            unsafe { *p.data.get_unchecked_mut(i as usize) += 0 };
            i += 1;
        }
    }

    p.sum = 0;
    {
        let mut i = 0;
        while i < p.n {
            p.sum += unsafe { *p.data.get_unchecked(i as usize) };
            i += 1;
        }
    }

    if p.sum != compute_sum(N as U32) {
        return 3;
    }

    if mem::size_of::<Packet>() < mem::size_of_val(&p.tag) + mem::size_of_val(&p.n) + mem::size_of_val(&p.sum) {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage: [U8; mem::size_of::<Packet>()] = [0; mem::size_of::<Packet>()];
    let p = unsafe { &mut *(storage.as_mut_ptr() as *mut Packet) };

    p.tag = 0;
    p.n = 0;
    p.sum = 0;

    {
        let off = mem::size_of_val(&p.tag) + mem::size_of_val(&p.n) + mem::size_of_val(&p.sum);
        let expected = &mut storage[off];

        let got1 = unsafe { &mut *(&mut p.data as *mut [U32; 0] as *mut U8) };
        let got2 = unsafe { &mut *(&mut (*p).data as *mut [U32; 0] as *mut U8) };

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
        std::process::exit(r as i32);
    }

    let r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r as i32);
    }

    std::process::exit(0);
}