use std::mem::size_of;

fn main() {
    let s_size: usize = size_of::<i32>();
    let ss_size: usize = size_of::<i32>();

    if s_size < ss_size {
        std::process::exit(1);
    }

    let offset_d: usize = s_size;
    if offset_d != s_size {
        std::process::exit(1);
    }

    let mut s1: Vec<u8> = vec![0; s_size + size_of::<f64>() * 8];
    let mut s2: Vec<u8> = vec![0; s_size + size_of::<f64>() * 5];

    if s1.is_empty() || s2.is_empty() {
        std::process::exit(1);
    }

    let d0_offset: usize = s_size;
    let val1: [u8; 8] = 42.0f64.to_le_bytes();
    s1[d0_offset..d0_offset + 8].copy_from_slice(&val1);

    let val2: [u8; 8] = 24.0f64.to_le_bytes();
    s2[d0_offset..d0_offset + 8].copy_from_slice(&val2);

    let read_bytes1: [u8; 8] = s1[d0_offset..d0_offset + 8].try_into().unwrap();
    let read1: f64 = f64::from_le_bytes(read_bytes1);

    let read_bytes2: [u8; 8] = s2[d0_offset..d0_offset + 8].try_into().unwrap();
    let read2: f64 = f64::from_le_bytes(read_bytes2);

    if read1 != 42.0 || read2 != 24.0 {
        std::process::exit(1);
    }

    std::process::exit(0);
}