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
    // [expr.postfix.subscript] 3x5 int array; fill via subscript
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = (i + j) as i32;
        }
    }
    // x[0]={0,1,2,3,4}, x[1]={1,2,3,4,5}, x[2]={2,3,4,5,6}

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = x[i][j];
            if a != b {
                std::process::exit(1);
            }
        }
    }

    // Pointer arithmetic equivalence check - in Rust we verify row length
    if x[0].len() != 5 {
        std::process::exit(2);
    }

    // [expr.postfix.member] rvalue member access on returned struct, args from array
    if make_coord(x[1][0], x[1][1]).row != x[1][0] {
        std::process::exit(3);
    }
    if make_coord(x[1][0], x[1][1]).col != x[1][1] {
        std::process::exit(4);
    }
    if make_coord(x[0][3], x[1][2]).row + make_coord(x[2][0], x[0][4]).col != 7 {
        std::process::exit(5);
    }

    // [decl.type_defs] local struct variables seeded from array
    let mut start = Coord { row: x[1][0], col: x[1][1] }; // {1, 2}
    let goal = Coord { row: x[2][0], col: x[2][1] };      // {2, 3}

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

    if shift_row(&mut start) != 101 {
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
        let mut wp = Waypoint { cost: x[2][0] }; // = 2
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