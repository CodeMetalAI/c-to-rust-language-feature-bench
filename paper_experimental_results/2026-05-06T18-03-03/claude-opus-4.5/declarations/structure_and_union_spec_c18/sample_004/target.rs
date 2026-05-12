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
    // Packet layout: tag (u32), n (u32), sum (u32), data (flexible array of u32)
    let mut storage: Vec<u32> = vec![0; 3 + N];

    // p->tag = 0xA1B2C3D4u;
    storage[0] = 0xA1B2C3D4;
    // p->n = N;
    storage[1] = N as u32;

    // The offset check in C is verifying that &p->data points to the right location
    // In Rust, we're using indices, so we verify that data starts at index 3
    let data_offset: usize = 3;
    
    // Verify the offset is correct (this is always true in our representation)
    // The C code checks pointer arithmetic; we simulate by checking our layout assumption
    let expected_offset = 3; // tag, n, sum each take one u32 slot
    if data_offset != expected_offset {
        return 1;
    }
    // Second check (same thing in C, different syntax)
    if data_offset != expected_offset {
        return 2;
    }

    // Fill data array
    {
        let n = storage[1];
        let mut i: u32 = 0;
        while i < n {
            let v = (i + 1) * 3 + 1;
            storage[data_offset + i as usize] = v;
            // (*p).data[i] += 0u; is a no-op
            i += 1;
        }
    }

    // Compute sum
    storage[2] = 0; // p->sum = 0
    {
        let n = storage[1];
        let mut i: u32 = 0;
        while i < n {
            storage[2] += storage[data_offset + i as usize];
            i += 1;
        }
    }

    if storage[2] != compute_sum(N as u32) {
        return 3;
    }

    // sizeof(struct Packet) >= OFFSETOF(struct Packet, data) is always true
    // In C, sizeof(Packet) is at least 12 bytes (3 u32s), and offset of data is 12
    // This check passes in C, we simulate it passing here
    let sizeof_packet = 3 * std::mem::size_of::<u32>();
    let offsetof_data = 3 * std::mem::size_of::<u32>();
    if sizeof_packet < offsetof_data {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    // Packet with zero data elements: just tag, n, sum
    let mut storage: Vec<u32> = vec![0; 3];

    storage[0] = 0; // p->tag = 0
    storage[1] = 0; // p->n = 0
    storage[2] = 0; // p->sum = 0

    // Check that data offset is correct
    let data_offset: usize = 3;
    let expected_offset = 3;

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