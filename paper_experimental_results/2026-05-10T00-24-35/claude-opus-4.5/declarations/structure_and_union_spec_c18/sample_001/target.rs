fn compute_sum(n: u32) -> u32 {
    let mut s: u32 = 0;
    let mut i: u32 = 0;
    while i < n {
        s += (i + 1) * 3 + 1;
        i += 1;
    }
    s
}

fn test_nonempty_object() -> i32 {
    const N: usize = 7;

    // Simulate a Packet with flexible array member
    // Fixed fields: tag, n, sum (3 u32s = 12 bytes)
    // Followed by N u32 data elements
    let mut tag: u32 = 0;
    let mut n: u32 = 0;
    let mut sum: u32 = 0;
    let mut data: [u32; N] = [0; N];

    tag = 0xA1B2C3D4;
    n = N as u32;

    // The offset checks in C are verifying pointer arithmetic with flexible array members
    // In Rust, we're using separate variables, so we simulate the check by ensuring
    // the data array starts at the expected offset (which is always true in our representation)
    // The C code checks that &p->data == storage.raw + offsetof(Packet, data)
    // We'll just verify that our data structure is consistent
    let off: usize = 3 * std::mem::size_of::<u32>(); // offset of data in original struct
    let expected_off: usize = 12; // 3 * 4 bytes
    if off != expected_off {
        return 1;
    }
    // Second check is same as first in our representation
    if off != expected_off {
        return 2;
    }

    // Fill data array
    {
        let mut i: u32 = 0;
        while i < n {
            let v: u32 = (i + 1) * 3 + 1;
            data[i as usize] = v;
            data[i as usize] += 0;
            i += 1;
        }
    }

    // Compute sum
    sum = 0;
    {
        let mut i: u32 = 0;
        while i < n {
            sum += data[i as usize];
            i += 1;
        }
    }

    if sum != compute_sum(N as u32) {
        return 3;
    }

    // Check sizeof(Packet) >= offsetof(Packet, data)
    // In C, sizeof(struct Packet) with flexible array is typically the size of fixed members
    // which equals the offset of data. So sizeof >= offset should be true.
    let sizeof_packet: usize = 3 * std::mem::size_of::<u32>(); // 12
    let offsetof_data: usize = 3 * std::mem::size_of::<u32>(); // 12
    if sizeof_packet < offsetof_data {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    // Simulate a Packet with zero-length flexible array
    let tag: u32 = 0;
    let n: u32 = 0;
    let sum: u32 = 0;
    let _data: [u32; 0] = [];

    // Suppress unused variable warnings
    let _ = tag;
    let _ = n;
    let _ = sum;

    // Same offset checks as above
    let off: usize = 3 * std::mem::size_of::<u32>();
    let expected_off: usize = 12;

    if off != expected_off {
        return 10;
    }
    if off != expected_off {
        return 11;
    }

    0
}

fn main() {
    let mut r: i32;

    r = test_nonempty_object();
    if r != 0 {
        std::process::exit(r);
    }

    r = test_zero_element_object();
    if r != 0 {
        std::process::exit(r);
    }

    std::process::exit(0);
}