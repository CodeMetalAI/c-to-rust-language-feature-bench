// structure_and_union_spec_rust.rs
type U8 = u8;
type U32 = u32;

#[repr(C)]
struct Packet {
    tag: U32,
    n: U32,
    sum: U32,
    data: [U32; 0],
}

fn offsetof<T>(member: &T) -> usize {
    let base = std::ptr::null::<T>();
    let member_ptr = member as *const T;
    member_ptr as usize - base as usize
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
    const N: usize = 7;

    let mut storage = [0u8; std::mem::size_of::<Packet>() + N * std::mem::size_of::<U32>() + 32];
    let p = unsafe { &mut *(storage.as_mut_ptr() as *mut Packet) };

    p.tag = 0xA1B2C3D4;
    p.n = N as U32;

    {
        let off = offsetof(&p.data);
        let expected = &storage[off] as *const U8;
        let got1 = &p.data as *const _ as *const U8;
        let got2 = &(*p).data as *const _ as *const U8;

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
                *(p.data.as_mut_ptr().add(i as usize)) = v;
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

    if p.sum != compute_sum(N as U32) {
        return 3;
    }

    if std::mem::size_of::<Packet>() < offsetof(&p.data) {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut storage = [0u8; std::mem::size_of::<Packet>()];
    let p = unsafe { &mut *(storage.as_mut_ptr() as *mut Packet) };

    p.tag = 0;
    p.n = 0;
    p.sum = 0;

    {
        let off = offsetof(&p.data);
        let expected = &storage[off] as *const U8;

        let got1 = &p.data as *const _ as *const U8;
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

fn main() -> i32 {
    let mut r;

    r = test_nonempty_object();
    if r != 0 {
        return r;
    }

    r = test_zero_element_object();
    if r != 0 {
        return r;
    }

    0
}