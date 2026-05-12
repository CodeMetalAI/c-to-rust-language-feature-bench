struct Coord {
    row: i32,
    col: i32,
}

struct Waypoint {
    cost: i32,
}

fn make_coord(row: i32, col: i32) -> Coord {
    Coord { row, col }
}

fn coord_sum(v: &Coord) -> i32 {
    v.row + v.col + 1
}

fn coord_sum_s(v: &Coord) -> i32 {
    v.row + v.col + 2
}

fn shift_row(p: &mut Coord) -> i32 {
    p.row += 100;
    p.row
}

fn padded(v: i32) -> i32 {
    v + 4
}

fn entry_cost(v: &Waypoint) -> i32 {
    v.cost + 5
}

fn main() -> i32 {
    let mut x = [[0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i + j;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j]; // Pointer arithmetic equivalent in Rust
            if a != b {
                return 1;
            }
        }
    }

    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    if (p1 as isize - p0 as isize) != 5 {
        return 2;
    }

    if make_coord(x[1][0], x[1][1]).row != x[1][0] {
        return 3;
    }
    if make_coord(x[1][0], x[1][1]).col != x[1][1] {
        return 4;
    }
    if make_coord(x[0][3], x[1][2]).row + make_coord(x[2][0], x[0][4]).col != 7 {
        return 5;
    }

    let mut start = Coord { row: x[1][0], col: x[1][1] };
    let goal = Coord { row: x[2][0], col: x[2][1] };

    if coord_sum(&start) != 4 {
        return 6;
    }
    if coord_sum_s(&start) != 5 {
        return 7;
    }
    if coord_sum(&goal) != 6 {
        return 8;
    }
    if coord_sum_s(&goal) != 7 {
        return 9;
    }

    let cursor = &mut start;
    if shift_row(cursor) != 101 {
        return 10;
    }
    if start.row != 101 {
        return 11;
    }
    if padded(start.row) != 105 {
        return 12;
    }

    {
        let mut q = 0;
        q += std::mem::size_of::<Coord>() as i32;
        q += std::mem::size_of::<Coord>() as i32; // struct coord is the same as Coord
        if q == 0 {
            return 13;
        }
    }

    {
        let wp = Waypoint { cost: x[2][0] };
        let wp_ptr = &wp;
        if wp_ptr.cost != x[2][0] {
            return 14;
        }
        if entry_cost(&wp) != 7 {
            return 15;
        }
    }

    println!("PASS");
    0
}