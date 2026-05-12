fn main() {
    struct W {
        k: i64,
        l: i64,
    }

    union Inner {
        anonymous: (i32, i32), // i, j
        w: std::mem::ManuallyDrop<W>,
    }

    struct V {
        inner: Inner,
        m: i32,
    }

    // Since we need to work with unions safely, we'll simulate the behavior
    // by tracking what was written. In the original C code:
    // - v1.i = 2 writes to the first int of the union
    // - v1.w.k = 5 writes to the first long of the union (overwrites i)
    // - v1.i != 2 checks the first int (which is now part of k=5)
    // - v1.w.k != 5 checks the first long
    
    // The C code has undefined behavior due to type punning, but let's trace
    // what actually happens on typical systems:
    // After v1.w.k = 5, reading v1.i would read the lower 32 bits of k
    // On little-endian 64-bit systems, v1.i would be 5 (lower 32 bits of 5)
    
    // However, looking at the test expectations (returns 0 means success),
    // the test expects v1.i == 2 to be false after v1.w.k = 5
    // Wait, let me re-read: if (v1.i != 2) return 1 - this returns 1 if i != 2
    
    // Actually in C with unions, writing to w.k overwrites the memory where i was.
    // So after v1.w.k = 5, v1.i would read as 5 (on systems where int fits in long).
    // So v1.i != 2 would be true, and it would return 1.
    
    // But the test seems to expect return 0 (success). Let me re-examine...
    // Oh wait, in C the anonymous struct and w share the same memory.
    // v1.i = 2 sets i to 2
    // v1.w.k = 5 sets k to 5, which OVERWRITES i (they share memory)
    // So v1.i would now be 5 (or part of 5), not 2
    // This means the first if would trigger and return 1
    
    // Unless... the test is checking that the code compiles and runs.
    // Let me just simulate the exact memory layout behavior.
    
    // For a faithful translation that produces the same result:
    // We need to simulate the union behavior where i and w.k share memory
    
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut w_k: i64 = 0;
    let mut w_l: i64 = 0;
    let mut m: i32 = 0;
    let _ = (j, w_l, m); // suppress unused warnings
    
    // v1.i = 2
    i = 2;
    // This also affects w_k since they share memory - lower 32 bits
    w_k = (w_k & !0xFFFFFFFF_i64) | (i as u32 as i64);
    
    // v1.w.k = 5
    w_k = 5;
    // This also affects i since they share memory
    i = w_k as i32;
    
    // if (v1.i != 2) return 1
    if i != 2 {
        std::process::exit(1);
    }
    
    // if (v1.w.k != 5) return 1
    if w_k != 5 {
        std::process::exit(1);
    }
    
    std::process::exit(0);
}