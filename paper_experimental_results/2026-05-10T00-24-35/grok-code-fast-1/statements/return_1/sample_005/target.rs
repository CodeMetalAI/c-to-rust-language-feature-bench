#[derive(Clone, Copy)]
struct S {
    i: f64,
}

static mut G: [u8; 12] = [0; 12];

fn read_f64(offset: usize) -> f64 {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&G[offset..offset + 8]);
    f64::from_le_bytes(buf)
}

fn write_f64(offset: usize, val: f64) {
    let buf = val.to_le_bytes();
    G[offset..offset + 8].copy_from_slice(&buf);
}

fn f() -> S {
    S { i: read_f64(4) }
}

fn foo() -> i32 {
    1
}

fn main() {
    write_f64(4, 1.0);
    write_f64(0, 1.0);
    let s1 = f();
    write_f64(4, s1.i);
    let s2 = f();
    write_f64(0, s2.i);
    let val = foo() as f64;
    let sum = read_f64(4) + read_f64(0) + val;
    std::process::exit(if sum == 3.0 { 0 } else { 1 });
}