// pathfinder.rs
// Features threaded through a 3x5 int array whose values seed the struct
// operations:
//   expressions/array_subscripting  [expr.postfix.subscript]
//   expressions/struct_and_union_1  [expr.postfix.member]
//   declarations/type_def_2         [decl.type_defs]

fn make_coord(row: i32, col: i32) -> (i32, i32) {
    (row, col)
}

fn coord_sum(v: (i32, i32)) -> i32 {
    v.0 + v.1 + 1
}

fn coord_sum_s(v: struct coord) -> i32 {
    v.row + v.col + 2
}

fn shift_row(p: &mut (i32, i32)) -> i32 {
    p.0 += 100;
    p.0
}

fn padded(v: i32) -> i32 {
    v + 4
}

fn entry_cost(v: (i32, i32)) -> i32 {
    v.1 + 5
}

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
            let b = *(&x[i][0] + j);
            if a!= b {
                println!("Error: a!= b");
                return;
            }
        }
    }

    let p0 = &x[0][0];
    let p1 = &x[1][0];
    if (p1 as *const _ as usize - p0 as *const _ as usize)!= 5 {
        println!("Error: p1 - p0!= 5");
        return;
    }

    // [expr.postfix.member] rvalue member access on returned struct, args from array
    if make_coord(x[1][0], x[1][1]).0!= x[1][0] {
        println!("Error: make_coord(x[1][0], x[1][1]).0!= x[1][0]");
        return;
    }
    if make_coord(x[1][0], x[1][1]).1!= x[1][1] {
        println!("Error: make_coord(x[1][0], x[1][1]).1!= x[1][1]");
        return;
    }
    if make_coord(x[0][3], x[1][2]).0 + make_coord(x[2][0], x[0][4]).1!= 7 {
        println!("Error: make_coord(x[0][3], x[1][2]).0 + make_coord(x[2][0], x[0][4]).1!= 7");
        return;
    }

    // [decl.type_defs] local struct variables seeded from array;
    // same variable passed to typedef-named and struct-tag-named params --
    // requires Copy on Coord/struct coord in correct Rust translation
    let start = (x[1][0], x[1][1]);
    let goal = (x[2][0], x[2][1]);
    let mut cursor = start;

    if coord_sum(start)!= 4 {
        println!("Error: coord_sum(start)!= 4");
        return;
    }
    if coord_sum_s(start)!= 5 {
        println!("Error: coord_sum_s(start)!= 5");
        return;
    }
    if coord_sum(goal)!= 6 {
        println!("Error: coord_sum(goal)!= 6");
        return;
    }
    if coord_sum_s(goal)!= 7 {
        println!("Error: coord_sum_s(goal)!= 7");
        return;
    }

    shift_row(&mut cursor);
    if cursor.0!= 101 {
        println!("Error: cursor.0!= 101");
        return;
    }
    if start.0!= 101 {
        println!("Error: start.0!= 101");
        return;
    }
    if padded(start.0)!= 105 {
        println!("Error: padded(start.0)!= 105");
        return;
    }

    {
        let q = std::mem::size_of::<(i32, i32)>() + std::mem::size_of::<struct coord>();
        if q == 0 {
            println!("Error: q == 0");
            return;
        }
    }

    {
        let wp = (x[2][0], 0);
        let wp_ptr = &wp;
        if wp_ptr.0!= x[2][0] {
            println!("Error: wp_ptr.0!= x[2][0]");
            return;
        }
        if entry_cost(wp)!= 7 {
            println!("Error: entry_cost(wp)!= 7");
            return;
        }
    }

    println!("PASS");
    std::process::exit(0);
}

struct coord {
    row: i32,
    col: i32,
}