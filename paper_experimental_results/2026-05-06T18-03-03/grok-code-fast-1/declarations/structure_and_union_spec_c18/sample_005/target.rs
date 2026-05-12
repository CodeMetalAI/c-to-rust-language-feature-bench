type u8 = u8;
type u32 = u32;

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
    let packet_size = std::mem::size_of::<Packet>();
    let storage_size = packet_size + N * std::mem::size_of::<u32>() + 32;
    let mut storage: Vec<u32> = vec![0; (storage_size + 3) / 4];
    let storage_u8 = unsafe { std::slice::from_raw_parts_mut(storage.as_mut_ptr() as *mut u8, storage_size) };
    let p = unsafe { &mut *(storage_u8.as_mut_ptr() as *mut Packet) };
    p.tag = 0xA1B2C3D4;
    p.n = N as u32;
    {
        let off = std::mem::size_of::<u32>() * 3;
        let expected = unsafe { storage_u8.as_ptr().add(off) };
        let got1 = unsafe { std::ptr::addr_of!((*p).data).cast::<u8>() };
        let got2 = unsafe { std::ptr::addr_of!((*p).data).cast::<u8>() };
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
            unsafe { *p.data.as_mut_ptr().add(i as usize) = v; }
            unsafe { *p.data.as_mut_ptr().add(i as usize) += 0; }
            i += 1;
        }
    }
    p.sum = 0;
    {
        let mut i = 0;
        while i < p.n {
            p.sum += unsafe { *p.data.as_ptr().add(i as usize) };
            i += 1;
        }
    }
    if p.sum != compute_sum(N as u32) {
        return 3;
    }
    if (std::mem::size_of::<Packet>() as u64) < (std::mem::size_of::<u32>() * 3) as u64 {
        return 4;
    }
    0
}

fn test_zero_element_object() -> i32 {
    let packet_size = std::mem::size_of::<Packet>();
    let storage_size = packet_size;
    let mut storage: Vec<u32> = vec![0; (storage_size + 3) / 4];
    let storage_u8 = unsafe { std::slice::from_raw_parts_mut(storage.as_mut_ptr() as *mut u8, storage_size) };
    let p = unsafe { &mut *(storage_u8.as_mut_ptr() as *mut Packet) };
    p.tag = 0;
    p.n = 0;
    p.sum = 0;
    {
        let off = std::mem::size_of::<u32>() * 3;
        let expected = unsafe { storage_u8.as_ptr().add(off) };
        let got1 = unsafe { std::ptr::addr_of!((*p).data).cast::<u8>() };
        let got2 = unsafe { std::ptr::addr_of!((*p).data).cast::<u8>() };
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
    let mut r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r);
    }
    r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }
    std::process::exit(0);
}