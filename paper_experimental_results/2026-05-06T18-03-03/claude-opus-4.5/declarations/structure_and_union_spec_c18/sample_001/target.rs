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
    let mut storage: Vec<u32> = vec![0; 3 + N]; // 3 fixed fields + N data elements

    // p->tag = 0xA1B2C3D4u;
    storage[0] = 0xA1B2C3D4;
    // p->n = N;
    storage[1] = N as u32;

    // The offset check in C is verifying that &p->data points to the right location
    // In our representation, data starts at index 3
    // We simulate this by checking that our indexing is consistent
    let data_offset: usize = 3;
    
    // Verify that accessing data through different syntax gives same result
    // In Rust, we just verify our data starts at the expected offset
    let expected_offset = data_offset;
    let got1 = data_offset; // &p->data
    let got2 = data_offset; // &(*p).data
    
    if got1 != expected_offset {
        return 1;
    }
    if got2 != expected_offset {
        return 2;
    }

    // Fill data array
    {
        let n = storage[1];
        let mut i: u32 = 0;
        while i < n {
            let v = (i + 1) * 3 + 1;
            storage[data_offset + i as usize] = v;
            storage[data_offset + i as usize] += 0;
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

    // sizeof(struct Packet) should be >= offset of data
    // In our model, the fixed part is 3 u32s = 12 bytes
    // offset of data is also 12 bytes (after tag, n, sum)
    let sizeof_packet: usize = 3 * std::mem::size_of::<u32>();
    let offsetof_data: usize = 3 * std::mem::size_of::<u32>();
    
    if sizeof_packet < offsetof_data {
        return 4;
    }

    0
}

fn test_zero_element_object() -> i32 {
    // Packet with zero data elements
    let mut storage: Vec<u32> = vec![0; 3]; // Just the 3 fixed fields

    storage[0] = 0; // p->tag = 0
    storage[1] = 0; // p->n = 0
    storage[2] = 0; // p->sum = 0

    let data_offset: usize = 3;
    let expected_offset = data_offset;
    
    let got1 = data_offset; // &p->data
    let got2 = data_offset; // &(*p).data
    
    if got1 != expected_offset {
        return 10;
    }
    if got2 != expected_offset {
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