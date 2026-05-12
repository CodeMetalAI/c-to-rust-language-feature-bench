// Define a struct S with two fields x and y
struct S {
    x: i32,
    y: i32,
}

// Define a function f that returns a value of type S
fn f() -> S {
    S { x: 7, y: 11 }
}

fn main() {
    // Call f and store the result in vx and vy
    let res = f();
    let vx = res.x;
    let vy = res.y;

    // Check if vx and vy match the expected values
    if vx != 7 {
        return 1;
    }
    if vy != 11 {
        return 2;
    }

    // Check if the sum of vx and vy matches the expected value
    if (vx + vy) != 18 {
        return 3;
    }

    // If all checks pass, return 0
    return 0;
}