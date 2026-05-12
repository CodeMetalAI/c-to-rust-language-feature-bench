type U8 = u8;
type U32 = u32;

#[repr(C)]
struct Packet {
    tag: U32,
    n: U32,
    sum: U32,
    data: [U32; 0], // Flexible array member in C, emulate with zero-length array in Rust
}

fn offsetof<T>(member_offset: usize) -> usize {
    member_offset
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

    union Storage {
        raw: [U8; std::mem::size_of::<Packet>() + N * std::mem::size_of::<U32>() + 32],
        align: U32,
    }

    let mut storage = Storage { raw: [0; std::mem::size_of::<Packet>() + N * std::mem::size_of::<U32>() + 32] };
    
    let packet_size = std::mem::size_of::<Packet>();
    let packet_data_offset = offsetof(packet_size);

    let p = unsafe { &mut *(storage.raw.as_mut_ptr() as *mut Packet) };

    p.tag = 0xA1B2C3D4;
    p.n = N as U32;

    let expected = unsafe { storage.raw.as_ptr().add(packet_data_offset) };
    let got1 = unsafe { &p.data as *const _ as *const U8 };
    let got2 = unsafe { &(*p).data as *const _ as *const U8 };

    if got1 != expected {
        return 1;
    }
    if got2 != expected {
        return 2;
    }

    for i in 0..p.n as usize {
        let v = (i as U32 + 1) * 3 + 1;
        unsafe {
            *(storage.raw.as_mut_ptr().add(packet_data_offset + i * std::mem::size_of::<U32>()) as *mut U32) = v;
        }
    }

    p.sum = 0;
    for i in 0..p.n as usize {
        unsafe {
            p.sum += *(storage.raw.as_ptr().add(packet_data_offset + i * std::mem::size_of::<U32>()) as *const U32);
        }
    }

    if p.sum != compute_sum(N as U32) {
        return 3;
    }

    if std::mem::size_of::<Packet>() < packet_data_offset {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    union Storage {
        raw: [U8; std::mem::size_of::<Packet>()],
        align: U32,
    }

    let mut storage = Storage { raw: [0; std::mem::size_of::<Packet>()] };

    let packet_data_offset = offsetof(std::mem::size_of::<Packet>());

    let p = unsafe { &mut *(storage.raw.as_mut_ptr() as *mut Packet) };

    p.tag = 0;
    p.n = 0;
    p.sum = 0;

    let expected = unsafe { storage.raw.as_ptr().add(packet_data_offset) };
    let got1 = unsafe { &p.data as *const _ as *const U8 };
    let got2 = unsafe { &(*p).data as *const _ as *const U8 };

    if got1 != expected {
        return 10;
    }
    if got2 != expected {
        return 11;
    }

    0
}

fn main() -> i32 {
    let mut r = test_nonempty_object();
    if r != 0 {
        return r;
    }

    r = test_zero_element_object();
    if r != 0 {
        return r;
    }

    0
}