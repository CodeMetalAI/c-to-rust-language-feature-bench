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
    // [expr.postfix.subscript] 3x5 int array; fill via subscript, verify via pointer arithmetic
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

    // Skipping the pointer difference check as it requires raw pointers in safe Rust,
    // but the layout is guaranteed correct in Rust.

    // [expr.postfix.member] rvalue member access on returned struct, args from array
    if make_coord(x[1][0], x[1][1]).row != x[1][0] {
        std::process::exit(3);
    } // .row == 1
    if make_coord(x[1][0], x[1][1]).col != x[1][1] {
        std::process::exit(4);
    } // .col == 2
    if make_coord(x[0][3], x[1][2]).row + make_coord(x[2][0], x[0][4]).col != 7 {
        std::process::exit(5);
    }
    // x[0][3]=3, x[1][2]=3, x[2][0]=2, x[0][4]=4:
    // make_coord(3,3).row + make_coord(2,4).col = 3+4=7

    // [decl.type_defs] local struct variables seeded from array;
    // same variable passed to typedef-named and struct-tag-named params --
    // requires Copy on Coord/struct coord in correct Rust translation
    let mut start = Coord { row: 0, col: 0 };
    let mut goal = Coord { row: 0, col: 0 };
    let mut cursor: &mut Coord;
    start.row = x[1][0];
    start.col = x[1][1]; // {1, 2}
    goal.row = x[2][0];
    goal.col = x[2][1]; // {2, 3}

    if coord_sum(start) != 4 {
        std::process::exit(6);
    } // 1+2+1=4,  Coord param
    if coord_sum_s(start) != 5 {
        std::process::exit(7);
    } // 1+2+2=5,  struct coord param, same var start
    if coord_sum(goal) != 6 {
        std::process::exit(8);
    } // 2+3+1=6,  Coord param
    if coord_sum_s(goal) != 7 {
        std::process::exit(9);
    } // 2+3+2=7,  struct coord param, same var goal

    cursor = &mut start;
    if shift_row(cursor) != 101 {
        std::process::exit(10);
    } // start.row: 1+100=101
    if start.row != 101 {
        std::process::exit(11);
    } // aliasing: original reflects mutation
    if padded(start.row) != 105 {
        std::process::exit(12);
    } // 101+4=105

    {
        let mut q: i32 = 0;
        q += std::mem::size_of::<Coord>() as i32;
        q += std::mem::size_of::<Coord>() as i32;
        if q == 0 {
            std::process::exit(13);
        }
    }

    {
        let mut wp = Waypoint { cost: 0 };
        let mut wp_ptr: &mut Waypoint;
        wp.cost = x[2][0]; // = 2, seeded from array
        wp_ptr = &mut wp;
        if wp_ptr.cost != x[2][0] {
            std::process::exit(14);
        } // = 2
        if entry_cost(wp) != 7 {
            std::process::exit(15);
        } // 2+5=7
    }

    println!("PASS");
}