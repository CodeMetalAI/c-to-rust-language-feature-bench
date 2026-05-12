use std::mem::size_of;

#[derive(Debug, Copy, Clone)]
struct Coord {
    row: i32,
    col: i32,
}

type CoordPtr = *mut Coord;

#[derive(Debug, Copy, Clone)]
struct Waypoint {
    cost: i32,
}

type WaypointPtr = *mut Waypoint;

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
    unsafe {
        (*p).row += 100;
        (*p).row
    }
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
            x[i][j] = i as i32 + j as i32;
        }
    }
    // x[0]={0,1,2,3,4}, x[1]={1,2,3,4,5}, x[2]={2,3,4,5,6}

    for i in 0..3 {
        for j in 0..5 {
            let a = x[i][j];
            let b = unsafe { *(*(x.as_ptr().offset(i as isize) as *const [i32; 5])).as_ptr().offset(j as isize) };
            if a != b {
                std::process::exit(1);
            }
        }
    }

    let p0 = x[0].as_ptr();
    let p1 = x[1].as_ptr();
    if (p1 as isize - p0 as isize) != 5 {
        std::process::exit(2);
    }

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
    let mut start = Coord { row: x[1][0], col: x[1][1] }; // {1, 2}
    let mut goal = Coord { row: x[2][0], col: x[2][1] }; // {2, 3}

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

    let cursor = &mut start as *mut Coord;
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
        let mut q = 0;
        q += size_of::<Coord>() as i32;
        q += size_of::<Coord>() as i32;
        if q == 0 {
            std::process::exit(13);
        }
    }

    {
        let mut wp = Waypoint { cost: x[2][0] }; // = 2, seeded from array
        let wp_ptr = &mut wp as *mut Waypoint;
        if unsafe { (*wp_ptr).cost } != x[2][0] {
            std::process::exit(14);
        } // = 2
        if entry_cost(wp) != 7 {
            std::process::exit(15);
        } // 2+5=7
    }

    println!("PASS");
    std::process::exit(0);
}