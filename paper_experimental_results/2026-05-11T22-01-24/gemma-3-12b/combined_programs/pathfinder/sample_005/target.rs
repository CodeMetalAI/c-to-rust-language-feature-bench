#[derive(Copy, Clone)]
struct Coord {
    row: i32,
    col: i32,
}

#[derive(Copy, Clone)]
struct Waypoint {
    cost: i32,
}

fn make_coord(row: i32, col: i32) -> Coord {
    Coord { row, col }
}

fn coord_sum(v: Coord) -> i32 {
    v.row + v.col + 1
}

fn coord_sum_s(v: Coord) -> i32 {
    v.row + v.col + 2
}

fn shift_row(p: &mut Coord) -> i32 {
    p.row += 100;
    p.row
}

fn padded(v: i32) -> i32 {
    v + 4
}

fn entry_cost(v: Waypoint) -> i32 {
    v.cost + 5
}

fn main() {
    /* [expr.postfix.subscript] 3x5 int array; fill via subscript, verify via pointer arithmetic */
    let mut x = [[0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i + j;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j];
            if a != b {
                return 1;
            }
        }
    }

    let p0 = &x[0][0];
    let p1 = &x[1][0];
    if (p1 as *const i32).wrapping_sub(p0 as *const i32) != 5 {
        return 2;
    }

    /* [expr.postfix.member] rvalue member access on returned struct, args from array */
    if make_coord(x[1][0], x[1][1]).row != x[1][0] {
        return 3;
    }
    if make_coord(x[1][0], x[1][1]).col != x[1][1] {
        return 4;
    }
    if make_coord(x[0][3], x[1][2]).row + make_coord(x[2][0], x[0][4]).col != 7 {
        return 5;
    }

    /* [decl.type_defs] local struct variables seeded from array;
       same variable passed to typedef-named and struct-tag-named params --
       requires Copy on Coord/struct coord in correct Rust translation */
    let mut start: Coord;
    let mut goal: Coord;
    let mut cursor: *mut Coord;

    start.row = x[1][0];
    start.col = x[1][1];
    goal.row = x[2][0];
    goal.col = x[2][1];

    if coord_sum(start) != 4 {
        return 6;
    }
    if coord_sum_s(start) != 5 {
        return 7;
    }
    if coord_sum(goal) != 6 {
        return 8;
    }
    if coord_sum_s(goal) != 7 {
        return 9;
    }

    cursor = &mut start as *mut Coord;
    if shift_row(&mut start) != 101 {
        return 10;
    }
    if start.row != 101 {
        return 11;
    }
    if padded(start.row) != 105 {
        return 12;
    }

    {
        let q = std::mem::size_of::<Coord>() + std::mem::size_of::<Coord>();
        if q == 0 {
            return 13;
        }
    }

    {
        let mut wp: Waypoint;
        let mut wp_ptr: *mut Waypoint;
        wp.cost = x[2][0];
        wp_ptr = &mut wp as *mut Waypoint;
        if unsafe { (*wp_ptr).cost } != x[2][0] {
            return 14;
        }
        if entry_cost(wp) != 7 {
            return 15;
        }
    }

    println!("PASS");
    return 0;
}