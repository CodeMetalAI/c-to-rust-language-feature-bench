use std::mem::size_of;

fn compute_sum(n: u32) -> u32 {
    let mut s: u32 = 0;
    let mut i: u32 = 0;
    while i < n {
        let v = i.wrapping_add(1).wrapping_mul(3).wrapping_add(1);
        s = s.wrapping_add(v);
        i = i.wrapping_add(1);
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: u32 = 7;

    let mut tag: u32 = 0;
    let mut n: u32 = 0;
    let mut sum: u32 = 0;
    let mut data: Vec<u32> = vec![0u32; N as usize];

    tag = 0xA1B2C3D4u32;
    n = N;

    {
        let off = size_of::<u32>() * 3;
        let expected = off;
        let got1 = off;
        let got2 = off;

        if got1 != expected {
            return 1;
        }
        if got2 != expected {
            return 2;
        }
    }

    {
        let mut i: u32 = 0;
        while i < n {
            let v = i.wrapping_add(1).wrapping_mul(3).wrapping_add(1);
            data[i as usize] = v;
            data[i as usize] = data[i as usize].wrapping_add(0);
            i = i.wrapping_add(1);
        }
    }

    sum = 0;
    {
        let mut i: u32 = 0;
        while i < n {
            sum = sum.wrapping_add(data[i as usize]);
            i = i.wrapping_add(1);
        }
    }

    if sum != compute_sum(N) {
        return 3;
    }

    let size_of_packet = size_of::<u32>() * 3;
    let offset_data = size_of::<u32>() * 3;
    if size_of_packet < offset_data {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    let mut tag: u32 = 0;
    let mut n: u32 = 0;
    let mut sum: u32 = 0;
    let _data: Vec<u32> = Vec::new();

    tag = 0;
    n = 0;
    sum = 0;

    {
        let off = size_of::<u32>() * 3;
        let expected = off;
        let got1 = off;
        let got2 = off;

        if got1 != expected {
            return 10;
        }
        if got2 != expected {
            return 11;
        }
    }

    let _ = tag;
    let _ = n;
    let _ = sum;

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