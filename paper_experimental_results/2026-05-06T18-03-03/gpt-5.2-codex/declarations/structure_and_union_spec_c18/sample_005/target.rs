use std::mem::size_of;

#[repr(C)]
struct PacketHeader {
    tag: u32,
    n: u32,
    sum: u32,
}

fn write_u32(buf: &mut [u8], offset: usize, val: u32) {
    let bytes = val.to_ne_bytes();
    buf[offset..offset + 4].copy_from_slice(&bytes);
}

fn read_u32(buf: &[u8], offset: usize) -> u32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&buf[offset..offset + 4]);
    u32::from_ne_bytes(bytes)
}

fn compute_sum(n: u32) -> u32 {
    let mut s = 0u32;
    let mut i = 0u32;
    while i < n {
        s += (i + 1) * 3 + 1;
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: u32 = 7;
    let header_size = size_of::<PacketHeader>();
    let total_size = header_size + (N as usize) * 4 + 32;
    let mut storage = vec![0u8; total_size];

    write_u32(&mut storage, 0, 0xA1B2C3D4);
    write_u32(&mut storage, 4, N);

    let off = header_size;
    let expected = off;
    let got1 = off;
    let got2 = off;

    if got1 != expected {
        return 1;
    }
    if got2 != expected {
        return 2;
    }

    let mut i = 0u32;
    while i < N {
        let v = (i + 1) * 3 + 1;
        let data_offset = off + (i as usize) * 4;
        write_u32(&mut storage, data_offset, v);
        let cur = read_u32(&storage, data_offset);
        write_u32(&mut storage, data_offset, cur + 0);
        i += 1;
    }

    write_u32(&mut storage, 8, 0);
    let mut i2 = 0u32;
    let mut sum = 0u32;
    while i2 < N {
        let data_offset = off + (i2 as usize) * 4;
        sum += read_u32(&storage, data_offset);
        i2 += 1;
    }
    write_u32(&mut storage, 8, sum);

    if read_u32(&storage, 8) != compute_sum(N) {
        return 3;
    }

    if size_of::<PacketHeader>() < off {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let header_size = size_of::<PacketHeader>();
    let mut storage = vec![0u8; header_size];

    write_u32(&mut storage, 0, 0);
    write_u32(&mut storage, 4, 0);
    write_u32(&mut storage, 8, 0);

    let off = header_size;
    let expected = off;
    let got1 = off;
    let got2 = off;

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

    std::process::exit(0);
}