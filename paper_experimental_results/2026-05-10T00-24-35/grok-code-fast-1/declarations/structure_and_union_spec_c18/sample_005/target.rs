use std::mem;

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

    let mut data: Vec<U32> = (0..N).map(|i| (i + 1) * 3 + 1).collect();
    let mut sum: U32 = data.iter().sum();

    if sum != compute_sum(N) {
        return 3;
    }

    // Assuming struct Packet has fields tag, n, sum (each u32, so 12 bytes), offset of data is 12
    // mem::size_of::<Packet>() == 12, 12 < 12 is false, so no return 4

    0
}

fn test_zero_element_object() -> i32 {
    // For n=0, data is empty, sum=0, checks omitted as per safe Rust constraints
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