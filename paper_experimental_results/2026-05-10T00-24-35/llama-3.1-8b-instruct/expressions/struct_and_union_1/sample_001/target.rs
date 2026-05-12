// Define a struct S with two integer fields
struct S {
    x: i32,
    y: i32,
}

// Define a function f() that returns an instance of S
fn f() -> S {
    S { x: 7, y: 11 }
}

fn main() {
    // Call f() and capture its results in vx and vy
    let vx = f().x;
    let vy = f().y;

    // Check if vx is not equal to 7 and return 1
    if vx != 7 {
        return 1;
    }
    // Check if vy is not equal to 11 and return 2
    if vy != 11 {
        return 2;
    }

    // Check if the sum of vx and vy is not equal to 18 and return 3
    if (vx + vy) != 18 {
        return 3;
    }

    // If all checks pass, return 0 to indicate success
    0
}