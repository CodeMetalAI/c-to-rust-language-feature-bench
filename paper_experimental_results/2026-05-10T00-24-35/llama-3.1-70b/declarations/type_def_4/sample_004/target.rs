fn main() {
    let mut hits = 0;

    fn handler_a(x: i32) { hits += x + 1; }
    fn handler_b(x: i32) { hits += x + 2; }

    fn signal(s: i32, h: fn(i32)) -> fn(i32) {
        if s & 1 == 1 {
            handler_a
        } else {
            h
        }
    }

    let handler_b = handler_b;

    if (signal(1, handler_b))(3)!= (3 + 1) {
        return 1;
    }

    hits = 0;
    if (signal(2, handler_b))(4)!= (4 + 2) {
        return 2;
    }

    hits = 0;
    if (signal(3, handler_b))(5)!= (5 + 1) {
        return 3;
    }

    println!("{}", hits);
}