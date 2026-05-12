static mut SINK: i32 = 0;

fn copyt(n: i32) {
    let mut b: [i32; n as usize + 1] = [0; n as usize + 1];
    let mut a: [i32; (n + 1) as usize] = [0; (n + 1) as usize];
    let mut i = 0;
    while i < n {
        b[i as usize] = 1000 + i;
        i += 1;
    }
    i = 1;
    while i < n {
        a[(i - 1) as usize] = b[i as usize];
        i += 1;
    }
    if a[0] != 1001 {
        SINK = 3;
    }
    if a[(n - 2) as usize] != 1000 + (n - 1) {
        SINK = 4;
    }
    if (b.len() as i32) / b.len() != n {
        SINK = 2;
    }
    if (a.len() as i32) / a.len() != (n - 1) {
        SINK = 1;
    }
}

fn main() {
    unsafe {
        SINK = 0;
    }
    copyt(7);
    if SINK != 0 {
        panic!("Unexpected value of sink");
    }
}