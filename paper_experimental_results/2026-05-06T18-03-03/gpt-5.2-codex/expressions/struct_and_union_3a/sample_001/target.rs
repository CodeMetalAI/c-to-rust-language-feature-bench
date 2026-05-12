use std::mem;

fn write_i32(buf: &mut [u8], offset: usize, value: i32) {
    let bytes = value.to_ne_bytes();
    buf[offset..offset + 4].copy_from_slice(&bytes);
}

fn read_i32(buf: &[u8], offset: usize) -> i32 {
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&buf[offset..offset + 4]);
    i32::from_ne_bytes(bytes)
}

fn write_f64(buf: &mut [u8], offset: usize, value: f64) {
    let bytes = value.to_ne_bytes();
    buf[offset..offset + 8].copy_from_slice(&bytes);
}

fn read_f64(buf: &[u8], offset: usize) -> f64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&buf[offset..offset + 8]);
    f64::from_ne_bytes(bytes)
}

fn main() {
    // Compute layout similar to C
    let int_size = mem::size_of::<i32>();
    let f64_align = mem::align_of::<f64>();
    let nf_double_offset = (int_size + f64_align - 1) / f64_align * f64_align;
    let union_size = nf_double_offset + mem::size_of::<f64>();
    let mut u = vec![0u8; union_size];

    // u.nf.type = 1;
    write_i32(&mut u, 0, 1);
    // u.nf.doublenode = 3.14;
    write_f64(&mut u, nf_double_offset, 3.14);

    // Checks
    if read_i32(&u, 0) != 1 {
        std::process::exit(1);
    }
    if read_i32(&u, 0) != 1 {
        std::process::exit(1);
    }
    if read_i32(&u, 0) != 1 {
        std::process::exit(2);
    }
    if read_f64(&u, nf_double_offset) != 3.14 {
        std::process::exit(3);
    }

    std::process::exit(0);
}