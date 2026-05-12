// Define a struct S with fields x and y
struct S {
    x: i32,
    y: i32,
}

// Define a function f that returns a struct S
fn f() -> S {
    S { x: 7, y: 11 }
}

fn main() {
    // Call f and assign its return value to s
    let s = f();

    // Extract the x and y fields from s
    let vx = s.x;
    let vy = s.y;

    // Check if vx and vy are equal to 7 and 11 respectively
    if vx != 7 {
        return 1;
    }
    if vy != 11 {
        return 2;
    }

    // Check if the sum of x and y is equal to 18
    if (s.x + s.y) != 18 {
        return 3;
    }

    // If all checks pass, return 0
    return 0;
}