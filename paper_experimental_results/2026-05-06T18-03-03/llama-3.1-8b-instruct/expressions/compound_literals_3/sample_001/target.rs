// Define a struct to represent a point
struct Point {
    x: i32,
    y: i32,
}

// Define a function to calculate the value based on the points
fn foo_val(a: Point, b: Point) -> i32 {
    if a.x != 1 {
        return 1;
    }
    if a.y != 1 {
        return 2;
    }
    if b.x != 3 {
        return 3;
    }
    if b.y != 4 {
        return 4;
    }
    0
}

// Define a function to calculate the value based on the points using references
fn bar_ptr(a: &Point, b: &Point) -> i32 {
    if a.x != 1 {
        return 5;
    }
    if a.y != 1 {
        return 6;
    }
    if b.x != 3 {
        return 7;
    }
    if b.y != 4 {
        return 8;
    }
    0
}

fn main() {
    // Create two points and pass them to foo_val
    let a = Point { x: 1, y: 1 };
    let b = Point { x: 3, y: 4 };
    if foo_val(a, b) != 0 {
        return;
    }

    // Create two points and pass their references to bar_ptr
    let c = Point { x: 1, y: 1 };
    let d = Point { x: 3, y: 4 };
    if bar_ptr(&c, &d) != 0 {
        return;
    }

    // If no return statements are reached, the program exits with code 0
}