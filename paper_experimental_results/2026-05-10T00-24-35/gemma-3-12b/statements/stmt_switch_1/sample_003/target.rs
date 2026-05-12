fn main() {
    let mut expr = 0;

    switch_expr(expr);
}

fn switch_expr(expr: i32) -> i32 {
    let mut i = 4;

    match expr {
        0 => {
            i = 17;
            return if i == 17 { 0 } else { 1 };
        }
        _ => {}
    }
    0
}