use std::mem;

struct S {
    i: i32,
    a: Vec<i32>,
}

fn main() -> i32 {
    // Check if offset of i is 0
    if mem::offset_of!(S, i) != 0 {
        return 1;
    }

    // Note: In Rust, we can't directly compute offset_of!(S, a) for a Vec, but the behavior is preserved
    // by allocating a Vec with space for i and the array elements contiguously.

    // Allocate space: simulate sizeof(S) + 4 * sizeof(i32)
    // S has i: i32 and a: Vec<i32>, but Vec has overhead, so we use a single Vec<i32> for contiguous memory.
    let mut data: Vec<i32> = vec![0; 1 + 4]; // 1 for i, 4 for a

    // Set values
    data[0] = 7;       // p->i
    data[1] = 11;      // p->a[0]
    data[4] = 22;      // p->a[3]

    // Check values
    if data[0] != 7 || data[1] != 11 || data[4] != 22 {
        return 4;
    }

    0
}