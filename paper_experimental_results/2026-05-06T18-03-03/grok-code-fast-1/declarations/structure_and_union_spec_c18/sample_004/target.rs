type U8 = u8;
type U32 = u32;

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
    const N: U32 = 7;
    let mut data: Vec<U32> = vec![0; N as usize];
    for i in 0..N {
        let v = (i + 1) * 3 + 1;
        data[i as usize] = v;
    }
    let mut sum = 0;
    for &v in &data {
        sum += v;
    }
    if sum != compute_sum(N) {
        return 3;
    }
    0
}

fn test_zero_element_object() -> i32 {
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