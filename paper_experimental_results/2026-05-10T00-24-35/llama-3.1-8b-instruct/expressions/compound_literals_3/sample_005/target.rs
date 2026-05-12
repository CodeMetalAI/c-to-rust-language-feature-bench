// Define a newtype struct Point
struct Point {
    x: i32,
    y: i32,
}

// Implement the main function
fn main() {
    // Define two points
    let point_a = Point { x: 1, y: 1 };
    let point_b = Point { x: 3, y: 4 };

    // Call foo_val
    if foo_val(point_a, point_b) != 0 {
        std::process::exit(10);
    }

    // Call bar_ptr
    if bar_ptr(&point_a, &point_b) != 0 {
        std::process::exit(11);
    }
}

// Define the function foo_val with the given logic
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

// Define the function bar_ptr with the given logic
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