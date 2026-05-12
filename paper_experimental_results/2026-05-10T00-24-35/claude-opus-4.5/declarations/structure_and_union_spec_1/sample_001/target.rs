fn main() {
    union Inner {
        anon: AnonStruct1,
        w: AnonStruct2,
    }

    #[derive(Copy, Clone)]
    struct AnonStruct1 {
        i: i32,
        j: i32,
    }

    #[derive(Copy, Clone)]
    struct AnonStruct2 {
        k: i64,
        l: i64,
    }

    struct V {
        inner: Inner,
        m: i32,
    }

    let mut v1 = V {
        inner: Inner {
            anon: AnonStruct1 { i: 0, j: 0 },
        },
        m: 0,
    };

    // v1.i = 2;
    v1.inner.anon.i = 2;

    // v1.w.k = 5;
    v1.inner.w.k = 5;

    // In C, writing to v1.w.k overwrites the memory that v1.i occupied
    // Since we're using safe Rust, we need to read from the union field that was last written
    // The C code checks v1.i after writing to v1.w.k, which reads from potentially overwritten memory
    
    // To preserve the exact C behavior, we need to check what the C code actually does:
    // After v1.w.k = 5, the memory layout means v1.i would be part of v1.w.k's bytes
    // On a little-endian 64-bit system, v1.i (first 4 bytes) would be 5, and j would be 0
    
    // However, in safe Rust we can't do union type punning directly
    // The C code's behavior depends on the union semantics where both fields share memory
    
    // Reading v1.i after writing v1.w.k = 5 would give us the low 32 bits of 5 (which is 5 on little-endian)
    // But the check is v1.i != 2, which would be true (5 != 2), so it would return 1
    
    // Wait, let me re-read the C code - the check happens AFTER both assignments
    // So v1.i = 2 is overwritten by v1.w.k = 5 (they share the same memory)
    // Therefore v1.i would NOT be 2 anymore in C
    
    // But the program returns 0, meaning the checks pass in the original C
    // This suggests the struct layout might be different, or we need to simulate this properly
    
    // Actually, re-examining: in C with this union, i and k share the same starting address
    // After v1.w.k = 5, reading v1.i gives the low 32 bits of 5 = 5 (not 2)
    // So the check v1.i != 2 would be TRUE, returning 1
    
    // But wait - the expected behavior should return 0 based on the program structure
    // Let me simulate what happens: after both writes, read the appropriate values
    
    // To match C behavior exactly with the union semantics:
    let i_val: i32 = unsafe { v1.inner.anon.i };
    let k_val: i64 = unsafe { v1.inner.w.k };
    
    if i_val != 2 {
        std::process::exit(1);
    }

    if k_val != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}