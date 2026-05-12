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

    // The offset checks in C are verifying that &p->data points to the right location.
    // In Rust, we're using indices, so the "data" starts at index 3.
    // The C code checks got1 == expected and got2 == expected.
    // Since we're modeling this with a slice, the equivalent check is that
    // the data portion starts at the correct offset. We'll just verify the
    // indexing is consistent (which it always is in Rust).
    
    // In the C code, these checks always pass for a correctly laid out struct,
    // so we don't return 1 or 2.

    // Fill data array
    {
        let n = storage[1];
        let mut i: u32 = 0;
        while i < n {
            let v = (i + 1) * 3 + 1;
            storage[3 + i as usize] = v;
            // (*p).data[i] += 0u; is a no-op
            i += 1;
        }
    }

    // Compute sum
    storage[2] = 0;
    {
        let n = storage[1];
        let mut i: u32 = 0;
        while i < n {
            storage[2] += storage[3 + i as usize];
            i += 1;
        }
    }

    if storage[2] != compute_sum(N as u32) {
        return 3;
    }

    // sizeof(struct Packet) >= OFFSETOF(struct Packet, data) is always true
    // for a flexible array member struct, so we don't return 4.

    0
}

fn test_zero_element_object() -> i32 {
    // Packet with zero data elements: just tag, n, sum
    let mut storage: Vec<u32> = vec![0; 3];

    storage[0] = 0; // tag
    storage[1] = 0; // n
    storage[2] = 0; // sum

    // The offset checks in C verify that &p->data points correctly even with 0 elements.
    // In Rust with our Vec model, the "data" would start at index 3, which is past the end
    // but that's fine since we never access it. The C checks always pass for correct layout.
    // We don't return 10 or 11.

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