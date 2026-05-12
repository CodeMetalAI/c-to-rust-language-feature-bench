#[derive(Clone, Copy)]
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
    let exit_code = if foo_val(Point { x: 1, y: 1 }, Point { x: 3, y: 4 }) != 0 {
        10
    } else if bar_ptr(&Point { x: 1, y: 1 }, &Point { x: 3, y: 4 }) != 0 {
        11
    } else {
        0
    };
    std::process::exit(exit_code);
}