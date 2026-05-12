use std::ptr;

#[derive(Copy, Clone)]
struct Coord {
    row: i32,
    col: i32,
}

type CoordPtr<'a> = &'a mut Coord;

#[derive(Copy, Clone)]
struct Waypoint {
    cost: i32,
}

type WaypointPtr<'a> = &'a mut Waypoint;

fn make_coord(row: i32, col: i32) -> Coord {
    Coord { row, col }
}

fn coord_sum(v: Coord) -> i32 {
    v.row + v.col + 1
}

fn coord_sum_s(v: Coord) -> i32 {
    v.row + v.col + 2
}

fn shift_row(p: CoordPtr) -> i32 {
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
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];
    
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = (i + j) as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j];
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    let diff = unsafe { p1.offset_from(p0) };
    if diff != 5 {
        std::process::exit(2);
    }

    if make_coord(x[1][0], x[1][1]).row != x[1][0] {
        std::process::exit(3);
    }
    if make_coord(x[1][0], x[1][1]).col != x[1][1] {
        std::process::exit(4);
    }
    if make_coord(x[0][3], x[1][2]).row + make_coord(x[2][0], x[0][4]).col != 7 {
        std::process::exit(5);
    }

    let mut start = Coord { row: 0, col: 0 };
    let mut goal = Coord { row: 0, col: 0 };
    
    start.row = x[1][0];
    start.col = x[1][1];
    goal.row = x[2][0];
    goal.col = x[2][1];

    if coord_sum(start) != 4 {
        std::process::exit(6);
    }
    if coord_sum_s(start) != 5 {
        std::process::exit(7);
    }
    if coord_sum(goal) != 6 {
        std::process::exit(8);
    }
    if coord_sum_s(goal) != 7 {
        std::process::exit(9);
    }

    let cursor: &mut Coord = &mut start;
    if shift_row(cursor) != 101 {
        std::process::exit(10);
    }
    if start.row != 101 {
        std::process::exit(11);
    }
    if padded(start.row) != 105 {
        std::process::exit(12);
    }

    {
        let mut q = 0;
        q += std::mem::size_of::<Coord>() as i32;
        q += std::mem::size_of::<Coord>() as i32;
        if q == 0 {
            std::process::exit(13);
        }
    }

    {
        let mut wp = Waypoint { cost: 0 };
        wp.cost = x[2][0];
        let wp_ptr: &mut Waypoint = &mut wp;
        if wp_ptr.cost != x[2][0] {
            std::process::exit(14);
        }
        if entry_cost(wp) != 7 {
            std::process::exit(15);
        }
    }

    println!("PASS");
}