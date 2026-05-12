fn main() {
    // [expr.postfix.subscript] 3x5 int array; fill via subscript, verify via pointer arithmetic
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i as i32 + j as i32;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = *unsafe { x.get_unchecked(i).get_unchecked(j) };
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = &x[0][0] as *const i32;
    let p1 = &x[1][0] as *const i32;
    if (p1 as isize - p0 as isize) != 5 {
        std::process::exit(2);
    }

    // [expr.postfix.member] rvalue member access on returned struct, args from array
    if Coord::new(x[1][0], x[1][1]).row != x[1][0] {
        std::process::exit(3);
    }
    if Coord::new(x[1][0], x[1][1]).col != x[1][1] {
        std::process::exit(4);
    }
    if Coord::new(x[0][3], x[1][2]).row + Coord::new(x[2][0], x[0][4]).col != 7 {
        std::process::exit(5);
    }

    // [decl.type_defs] local struct variables seeded from array;
    let mut start = Coord::new(x[1][0], x[1][1]);
    let mut goal = Coord::new(x[2][0], x[2][1]);
    let mut cursor = &mut start;

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
        let mut q = 0;
        q += std::mem::size_of::<Coord>();
        q += std::mem::size_of::<Coord>();
        if q == 0 {
            std::process::exit(13);
        }
    }

    {
        let mut wp = Waypoint::new(x[2][0]);
        let wp_ptr = &mut wp;
        if wp_ptr.cost != x[2][0] {
            std::process::exit(14);
        }
        if entry_cost(wp) != 7 {
            std::process::exit(15);
        }
    }

    println!("PASS");
}

#[derive(Debug, Copy, Clone)]
struct Coord {
    row: i32,
    col: i32,
}

impl Coord {
    fn new(row: i32, col: i32) -> Self {
        Coord { row, col }
    }
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

#[derive(Debug, Copy, Clone)]
struct Waypoint {
    cost: i32,
}

impl Waypoint {
    fn new(cost: i32) -> Self {
        Waypoint { cost }
    }
}

fn entry_cost(v: Waypoint) -> i32 {
    v.cost + 5
}