// Define a struct for a point with x and y coordinates
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Define a function to calculate a value based on two points
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

// Define a function to calculate a value based on two pointers to points
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
    // Create two points
    let point_a = Point { x: 1, y: 1 };
    let point_b = Point { x: 3, y: 4 };

    // Test foo_val function
    if foo_val(point_a, point_b) != 0 {
        return;
    }

    // Create two pointers to points
    let point_a_ptr = &point_a;
    let point_b_ptr = &point_b;

    // Test bar_ptr function
    if bar_ptr(point_a_ptr, point_b_ptr) != 0 {
        return;
    }

    // If both tests pass, return 0
    std::process::exit(0);
}