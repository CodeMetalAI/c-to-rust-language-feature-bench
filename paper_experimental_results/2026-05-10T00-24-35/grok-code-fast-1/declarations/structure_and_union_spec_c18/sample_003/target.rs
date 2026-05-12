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
    let sum_computed = compute_sum(N);
    let mut data = [0u32; 7];
    let mut i = 0;
    while i < N {
        let v = (i + 1) * 3 + 1;
        data[i as usize] = v;
        i += 1;
    }
    let mut sum = 0;
    i = 0;
    while i < N {
        sum += data[i as usize];
        i += 1;
    }
    if sum != sum_computed {
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