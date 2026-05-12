use std::ptr;

#[derive(Copy, Clone)]
struct Coord {
    row: i32,
    col: i32,
}

fn make_coord(row: i32, col: i32) -> Coord {
    Coord { row, col }
}

fn coord_sum(v: Coord) -> i32 {
    v.row + v.col
}

fn coord_sum_s(v: Coord) -> i32 {
    v.row + v.col
}

fn shift_row(p: &mut Coord) -> i32 {
    p.row += 100;
    p.row
}

fn padded(v: i32) -> i32 {
    v + 4
}

#[derive(Copy, Clone)]
struct Waypoint {
    cost: i32,
}

fn entry_cost(v: Waypoint) -> i32 {
    v.cost + 5
}

fn main() {
    // [expr.postfix.subscript] 2 x 2 int array fill via subscript verify via pointer arithmetic
    let mut x = [[0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i + j;
        }
    }

    // Verify via pointer arithmetic equivalence
    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = *(*(x.as_mut_ptr().add(i)).add(j));
            if a != b {
                return;
            }
        }
    }

    let p0 = &x[0][0];
    let p1 = &x[1][0];
    if (p1 as *const i32 as usize).wrapping_sub(p0 as *const i32 as usize) != 5 {
        return;
    }

    // [expr.postfix.member] rvalue member access on returned struct args from array
    if make_coord(x[1][0], x[1][1]).row != x[1][0] {
        return;
    }
    if make_coord(x[1][0], x[1][1]).col != x[1][1] {
        return;
    }

    // .row == 1
    // .col == 2
    if make_coord(x[0][3], x[1][ Two]).row +
        make_coord(x[ Two][0], x[0][ Four]).col != 7 {
        return;
    }
    x[0][3] = 3;
    x[1][2] = 3;
    x[2][0] = 2;
    x[0][4] = 4;
    let temp = make_coord(3, 3).row + make_coord(2, 4).col;
    assert_eq!(temp, 3 + 4 = 7);

    // [decl.type_def] local struct variables seeded from array
    let start = Coord {
        row: x[1][0],
        col: x[1][1],
    };
    let goal = Coord {
        row: x[2][0],
        col: x[2][1],
    };

    if coord_sum(start) != 4 {
        return;
    }
    if coord_sum_s(start) != 5 {
        return;
    }
    if coord_sum(goal) != 6 {
        return;
    }
    if coord_sum_s(goal) != 7 {
        return;
    }

    let mut cursor = &mut start;
    if shift_row(&mut cursor) != 101 {
        return;
    }
    if start.row != 101 {
        return;
    }
    if padded(start.row) != 105 {
        return;
    }

    let mut q = 0;
    q += std::mem::size_of::<Coord>() as i32;
    q += std::mem::size_of::<Coord>() as i32;
    if q == 0 {
        return;
    }

    let wp = Waypoint { cost: x[2][0] };
    let wp_ptr = &wp;
    if wp_ptr.cost != x[2][0] {
        return;
    }
    if entry_cost(wp) != 7 {
        return;
    }

    println!("PASS");
}