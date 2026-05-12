#[derive(Clone, Copy)]
struct Coord {
    row: i32,
    col: i32,
}

#[derive(Clone, Copy)]
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
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = (i + j) as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = unsafe {
                let x_ptr = x.as_ptr() as *const i32;
                *x_ptr.add(i * 5 + j)
            };
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = unsafe { x.as_ptr() as *const i32 };
    let p1 = unsafe { x.as_ptr().add(5) };
    if unsafe { (p1 as usize - p0 as usize) / std::mem::size_of::<i32>() } != 5 {
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

    let mut start = Coord {
        row: x[1][0],
        col: x[1][1],
    };
    let mut goal = Coord {
        row: x[2][0],
        col: x[2][1],
    };
    let cursor = &mut start;

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
        let mut q: i32 = 0;
        q += std::mem::size_of::<Coord>() as i32;
        q += std::mem::size_of::<Coord>() as i32;
        if q == 0 {
            std::process::exit(13);
        }
    }

    {
        let mut wp = Waypoint { cost: x[2][0] };
        let wp_ptr = &mut wp;
        if wp_ptr.cost != x[2][0] {
            std::process::exit(14);
        }
        if entry_cost(wp) != 7 {
            std::process::exit(15);
        }
    }

    println!("PASS");
    std::process::exit(0);
}