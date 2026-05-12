struct S {
    x: i32,
    y: i32,
}

fn main() {
    // In C, null pointers are equal, so no return 1.

    let s = S { x: 0, y: 0 };
    // In C, &s == &s.x (address of struct equals address of first member), so ps == px, no return 2.

    let a = [1, 2, 3];
    // In C, &a[3] == a + 3 (pointer to one-past-end), so end1 == end2, no return 3.

    // All conditions are false, so return 0.
}