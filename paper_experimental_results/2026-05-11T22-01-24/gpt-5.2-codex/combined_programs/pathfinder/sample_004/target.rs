use std::mem::size_of;
use std::process::exit;

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
    let mut x = [[0i32; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i as i32 + j as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j];
            if a != b {
                exit(1);
            }
        }
    }

    let p0_index = 0isize;
    let p1_index = (1 * 5) as isize;
    if p1_index - p0_index != 5 {
        exit(2);
    }

    if make_coord(x[1][0], x[1][1]).row != x[1][0] {
        exit(3);
    }
    if make_coord(x[1][0], x[1][1]).col != x[1][1] {
        exit(4);
    }
    if make_coord(x[0][3], x[1][2]).row + make_coord(x[2][0], x[0][4]).col != 7 {
        exit(5);
    }

    let mut start = Coord { row: 0, col: 0 };
    let mut goal = Coord { row: 0, col: 0 };

    start.row = x[1][0];
    start.col = x[1][1];
    goal.row = x[2][0];
    goal.col = x[2][1];

    if coord_sum(start) != 4 {
        exit(6);
    }
    if coord_sum_s(start) != 5 {
        exit(7);
    }
    if coord_sum(goal) != 6 {
        exit(8);
    }
    if coord_sum_s(goal) != 7 {
        exit(9);
    }

    {
        let result = {
            let cursor = &mut start;
            shift_row(cursor)
        };
        if result != 101 {
            exit(10);
        }
    }
    if start.row != 101 {
        exit(11);
    }
    if padded(start.row) != 105 {
        exit(12);
    }

    {
        let mut q = 0i32;
        q += size_of::<Coord>() as i32;
        q += size_of::<Coord>() as i32;
        if q == 0 {
            exit(13);
        }
    }

    {
        let mut wp = Waypoint { cost: x[2][0] };
        let wp_ptr = &wp;
        if wp_ptr.cost != x[2][0] {
            exit(14);
        }
        if entry_cost(wp) != 7 {
            exit(15);
        }
    }

    println!("PASS");
    exit(0);
}