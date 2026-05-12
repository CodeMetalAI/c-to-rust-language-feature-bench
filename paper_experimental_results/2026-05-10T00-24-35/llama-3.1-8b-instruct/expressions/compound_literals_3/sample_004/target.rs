// Define a struct to represent a point
struct Point {
    x: i32,
    y: i32,
}

// Define a function to calculate the value based on the points
fn foo_val(a: Point, b: Point) -> i32 {
    if a.x != 1 {
        1
    } else if a.y != 1 {
        2
    } else if b.x != 3 {
        3
    } else if b.y != 4 {
        4
    } else {
        0
    }
}

// Define a function to calculate the value based on the points using references
fn bar_ptr(a: &Point, b: &Point) -> i32 {
    if a.x != 1 {
        5
    } else if a.y != 1 {
        6
    } else if b.x != 3 {
        7
    } else if b.y != 4 {
        8
    } else {
        0
    }
}

fn main() {
    // Create two points
    let point1 = Point { x: 1, y: 1 };
    let point2 = Point { x: 3, y: 4 };

    // Check the result of foo_val
    if foo_val(point1, point2) != 0 {
        return 10;
    }

    // Check the result of bar_ptr
    if bar_ptr(&point1, &point2) != 0 {
        return 11;
    }

    // If no errors, return 0
    return 0;
}