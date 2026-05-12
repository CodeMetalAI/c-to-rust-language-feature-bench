struct Point {
    x: i32,
    y: i32,
}

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
    let a = Point { x: 1, y: 1 };
    let b = Point { x: 3, y: 4 };

    if foo_val(a, b) != 0 {
        std::process::exit(10);
    }

    let a = Point { x: 1, y: 1 };
    let b = Point { x: 3, y: 4 };
    if bar_ptr(&a, &b) != 0 {
        std::process::exit(11);
    }
}