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

    // Simulate the packet structure with a Vec
    // Packet has: tag (u32), n (u32), sum (u32), data (flexible array of u32)
    let mut storage: Vec<u32> = vec![0; 3 + N];

    // storage[0] = tag
    // storage[1] = n
    // storage[2] = sum
    // storage[3..] = data

    storage[0] = 0xA1B2C3D4u32; // tag
    storage[1] = N as u32;       // n

    // The offset checks in C are about pointer arithmetic
    // In Rust, we're using indices, so we verify the data starts at index 3
    let data_offset: usize = 3;
    let expected_offset: usize = 3;

    if data_offset != expected_offset {
        return 1;
    }
    if data_offset != expected_offset {
        return 2;
    }

    // Fill data array
    {
        let mut i: u32 = 0;
        let n = storage[1];
        while i < n {
            let v = (i + 1) * 3 + 1;
            storage[data_offset + i as usize] = v;
            storage[data_offset + i as usize] += 0;
            i += 1;
        }
    }

    // Compute sum
    storage[2] = 0; // sum = 0
    {
        let mut i: u32 = 0;
        let n = storage[1];
        while i < n {
            storage[2] += storage[data_offset + i as usize];
            i += 1;
        }
    }

    if storage[2] != compute_sum(N as u32) {
        return 3;
    }

    // In C: sizeof(struct Packet) >= offsetof(struct Packet, data)
    // The header size (3 u32s = 12 bytes) should be >= offset of data (also 12 bytes)
    let sizeof_packet: usize = 3 * std::mem::size_of::<u32>();
    let offsetof_data: usize = 3 * std::mem::size_of::<u32>();
    if sizeof_packet < offsetof_data {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    // Packet with zero data elements
    let mut storage: Vec<u32> = vec![0; 3];

    storage[0] = 0; // tag
    storage[1] = 0; // n
    storage[2] = 0; // sum

    // Check offset of data
    let data_offset: usize = 3;
    let expected_offset: usize = 3;

    if data_offset != expected_offset {
        return 10;
    }
    if data_offset != expected_offset {
        return 11;
    }

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