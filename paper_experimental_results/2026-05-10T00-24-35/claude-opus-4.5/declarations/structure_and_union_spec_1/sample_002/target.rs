fn main() {
    union Inner {
        anon: AnonStruct1,
        w: AnonStruct2,
    }

    #[derive(Clone, Copy)]
    struct AnonStruct1 {
        i: i32,
        j: i32,
    }

    #[derive(Clone, Copy)]
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
    // Since we're in safe Rust, we need to read from the union member that was last written
    // The C code checks v1.i after writing to v1.w.k, which in C reads from overlapping memory
    // In the original C code with the union, writing to w.k overwrites i (they share memory)
    // So v1.i would no longer be 2 after v1.w.k = 5

    // Reading v1.i after writing v1.w.k - in C this reads the reinterpreted bits
    // Since k is i64 and i is i32, the low 32 bits of k=5 would be read as i
    // On little-endian systems, i would be 5
    // But the C code expects v1.i != 2 to be false... let's trace through:
    // Actually re-reading the C: the check is "if (v1.i != 2) return 1;"
    // In C with unions, v1.i and v1.w.k share the same starting memory
    // After v1.w.k = 5, v1.i would read as 5 (on little-endian), so v1.i != 2 would be true
    // and it would return 1... but the expected behavior should return 0
    
    // Wait, let me re-read. The struct has an anonymous union containing two structs.
    // The anonymous struct {int i, j} and struct w {long k, l} share the same memory.
    // After v1.w.k = 5, reading v1.i gives the reinterpreted value.
    // If sizeof(long) == sizeof(int) (32-bit), k=5 means i=5.
    // If sizeof(long) == 8 (64-bit little-endian), low 32 bits of 5 is still 5, so i=5.
    // Either way, i != 2, so return 1.
    
    // But for the program to return 0 as "expected", perhaps we need different semantics.
    // Let me just translate literally - accessing union members that weren't last written
    // is undefined in C anyway, so I'll simulate the memory overlap behavior.
    
    // After writing w.k = 5, on little-endian, i would be 5
    let i_value: i32 = (v1.inner.w.k & 0xFFFFFFFF) as i32;
    
    if i_value != 2 {
        std::process::exit(1);
    }

    if v1.inner.w.k != 5 {
        std::process::exit(1);
    }

    std::process::exit(0);
}