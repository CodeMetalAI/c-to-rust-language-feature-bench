struct Point {
    x: i32,
    y: i32,
}

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
    if foo_val(Point { x: 1, y: 1 }, Point { x: 3, y: 4 }) != 0 {
        std::process::exit(10);
    }

    if bar_ptr(&Point { x: 1, y: 1 }, &Point { x: 3, y: 4 }) != 0 {
        std::process::exit(11);
    }

    std::process::exit(0);
}