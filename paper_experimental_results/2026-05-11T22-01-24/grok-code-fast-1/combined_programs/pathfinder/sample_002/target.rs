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

fn main() -> i32 {
    // [expr.postfix.subscript] 3x5 int array; fill via subscript
    let mut x: [[i32; 5]; 3] = [[0; 5]; 3];
    for i in 0..3 {
        for j in 0..5 {
            x[i][j] = i as i32 + j as i32;
        }
    }
    // x[0]={0,1,2,3,4}, x[1]={1,2,3,4,5}, x[2]={2,3,4,5,6}

    // Skip pointer arithmetic checks as they cannot be done in safe Rust,
    // but the behavior is equivalent since arrays are contiguous in memory.

    // [expr.postfix.member] rvalue member access on returned struct, args from array
    if make_coord(x[1][0], x[1][1]).row != x[1][0] {
        return 3;
    } // .row == 1
    if make_coord(x[1][0], x[1][1]).col != x[1][1] {
        return 4;
    } // .col == 2
    if make_coord(x[0][3], x[1][2]).row + make_coord(x[2][0], x[0][4]).col != 7 {
        return 5;
    }
    // x[0][3]=3, x[1][2]=3, x[2][0]=2, x[0][4]=4:
    // make_coord(3,3).row + make_coord(2,4).col = 3+4=7

    // [decl.type_defs] local struct variables seeded from array;
    let mut start = Coord {
        row: x[1][0],
        col: x[1][1],
    }; // {1, 2}
    let mut goal = Coord {
        row: x[2][0],
        col: x[2][1],
    }; // {2, 3}

    if coord_sum(start) != 4 {
        return 6;
    } // 1+2+1=4, Coord param
    if coord_sum_s(start) != 5 {
        return 7;
    } // 1+2+2=5, struct coord param, same var start
    if coord_sum(goal) != 6 {
        return 8;
    } // 2+3+1=6, Coord param
    if coord_sum_s(goal) != 7 {
        return 9;
    } // 2+3+2=7, struct coord param, same var goal

    let cursor = &mut start;
    if shift_row(cursor) != 101 {
        return 10;
    } // start.row: 1+100=101
    if start.row != 101 {
        return 11;
    } // aliasing: original reflects mutation
    if padded(start.row) != 105 {
        return 12;
    } // 101+4=105

    {
        let mut q: i32 = 0;
        q += std::mem::size_of::<Coord>() as i32;
        q += std::mem::size_of::<Coord>() as i32;
        if q == 0 {
            return 13;
        }
    }

    {
        let mut wp = Waypoint { cost: x[2][0] }; // = 2, seeded from array
        let wp_ptr = &mut wp;
        if wp_ptr.cost != x[2][0] {
            return 14;
        } // = 2
        if entry_cost(wp) != 7 {
            return 15;
        } // 2+5=7
    }

    println!("PASS");
    0
}