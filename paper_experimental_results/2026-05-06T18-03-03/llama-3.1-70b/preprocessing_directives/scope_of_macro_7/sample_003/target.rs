fn main() {
    let x = 7;
    let y = 3;

    if debug("Flag")!= 1 {
        return 1;
    }
    if debug("X = %d\n", x)!= 2 {
        return 2;
    }

    if!streq("The first, second, and third items.", "The first, second, and third items.") {
        return 3;
    }

    if report(x > y, "x is {} but y is {}", x, y)!= 1 {
        return 4;
    }
    if report(x < y, "x is {} but y is {}", x, y)!= 3 {
        return 5;
    }

    return 0;
}

fn debug<T: std::fmt::Debug>(args: T) -> usize {
    std::mem::size_of_val(&args)
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}

fn report<T: std::fmt::Debug>(test: bool, args: T) -> usize {
    if test {
        1
    } else {
        std::mem::size_of_val(&args)
    }
}