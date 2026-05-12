use std::cmp::max;
use std::mem::size_of;
use std::process::exit;

fn write_int(buf: &mut [u8], val: i32) {
    let bytes = val.to_ne_bytes();
    buf[..bytes.len()].copy_from_slice(&bytes);
}

fn write_long(buf: &mut [u8], val: i64) {
    let bytes = val.to_ne_bytes();
    buf[..bytes.len()].copy_from_slice(&bytes);
}

fn read_int(buf: &[u8]) -> i32 {
    let mut arr = [0u8; 4];
    arr.copy_from_slice(&buf[..arr.len()]);
    i32::from_ne_bytes(arr)
}

fn read_long(buf: &[u8]) -> i64 {
    let mut arr = [0u8; 8];
    arr.copy_from_slice(&buf[..arr.len()]);
    i64::from_ne_bytes(arr)
}

fn main() {
    let union_size = max(2 * size_of::<i32>(), 2 * size_of::<i64>());
    let mut bytes = vec![0u8; union_size];

    // v1.i = 2;
    write_int(&mut bytes, 2);
    // v1.w.k = 5;
    write_long(&mut bytes, 5);

    if read_int(&bytes) != 2 {
        exit(1);
    }

    if read_long(&bytes) != 5 {
        exit(1);
    }

    exit(0);
}