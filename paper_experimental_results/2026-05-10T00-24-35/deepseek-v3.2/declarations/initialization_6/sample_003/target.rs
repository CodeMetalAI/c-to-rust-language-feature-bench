fn chk(v: i16, e: i16) -> bool {
    v == e
}

fn main() {
    // Rust requires explicit array dimensions and full type specification.
    // Initializer follows C's rule: unspecified elements are zero-initialized.
    let q: [[[i16; 2]; 3]; 4] = [
        [[1, 0], [0, 0], [0, 0]],
        [[2, 3], [0, 0], [0, 0]],
        [[4, 5], [6, 0], [0, 0]],
        [[0, 0], [0, 0], [0, 0]],
    ];

    if !chk(q[0][0][0], 1) {
        std::process::exit(1);
    }
    if !chk(q[0][0][1], 0) {
        std::process::exit(2);
    }
    if !chk(q[0][1][0], 0) {
        std::process::exit(3);
    }
    if !chk(q[0][2][1], 0) {
        std::process::exit(4);
    }

    if !chk(q[1][0][0], 2) {
        std::process::exit(5);
    }
    if !chk(q[1][0][1], 3) {
        std::process::exit(6);
    }
    if !chk(q[1][1][0], 0) {
        std::process::exit(7);
    }

    if !chk(q[2][0][0], 4) {
        std::process::exit(8);
    }
    if !chk(q[2][0][1], 5) {
        std::process::exit(9);
    }
    if !chk(q[2][1][0], 6) {
        std::process::exit(10);
    }
    if !chk(q[2][1][1], 0) {
        std::process::exit(11);
    }

    if !chk(q[3][0][0], 0) {
        std::process::exit(12);
    }
    if !chk(q[3][2][1], 0) {
        std::process::exit(13);
    }

    {
        // In Rust, we can safely get a slice view of the whole array as a flat slice.
        let p: &[i16] = &q[0][0][0..]; // Actually, we need to flatten properly.
        // Better: reinterpret the array as a flat slice using its memory layout.
        // Since arrays are stored contiguously, we can take a slice of the whole memory.
        let p: &[i16] = unsafe {
            // But safe Rust doesn't allow reinterpretation without unsafe.
            // Instead, we can compute indices manually, but let's simulate the pointer arithmetic.
            // We'll create a flat slice by referencing the first element and using slice::from_raw_parts.
            // However, that requires unsafe. Instead, we'll manually compute the linear index.
            // The C code uses p[0], p[6], etc. The array dimensions are [4][3][2].
            // Linear index = i*3*2 + j*2 + k.
            // We'll just compute each expected value directly.
        };
        // Instead of unsafe, we'll compute the linear indexing manually.
        // The C pointer p points to the first element, and the array is row-major.
        // So p[6] corresponds to q[1][0][0] because 6 = 1*3*2 + 0*2 + 0.
        // Let's verify the expected values using the same linear indexing.
        // We'll create a flat iterator over the array to simulate the linear memory.
        let flat: Vec<i16> = q.iter()
            .flat_map(|layer| layer.iter())
            .flat_map(|row| row.iter())
            .cloned()
            .collect();
        if !chk(flat[0], 1) {
            std::process::exit(14);
        }
        if !chk(flat[6], 2) {
            std::process::exit(15);
        }
        if !chk(flat[7], 3) {
            std::process::exit(16);
        }
        if !chk(flat[12], 4) {
            std::process::exit(17);
        }
        if !chk(flat[13], 5) {
            std::process::exit(18);
        }
        if !chk(flat[14], 6) {
            std::process::exit(19);
        }
    }

    std::process::exit(0);
}