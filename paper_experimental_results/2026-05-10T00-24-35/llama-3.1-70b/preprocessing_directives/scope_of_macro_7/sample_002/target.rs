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
    let mut count = 0;
    std::fmt::Debug::fmt(&args, &mut std::fmt::Formatter::new())
       .expect("Failed to format debug output");
    count += 1;
    if let Some((_, rest)) = args.as_ref().debug_tuple_fields() {
        for _ in rest {
            count += 1;
        }
    }
    count
}

fn showlist<T: std::fmt::Display>(args: T) -> String {
    format!("{}", args)
}

fn report<T: std::fmt::Debug>(test: bool, args: T) -> usize {
    if test {
        1
    } else {
        let mut count = 0;
        std::fmt::Debug::fmt(&args, &mut std::fmt::Formatter::new())
           .expect("Failed to format debug output");
        count += 1;
        if let Some((_, rest)) = args.as_ref().debug_tuple_fields() {
            for _ in rest {
                count += 1;
            }
        }
        count
    }
}

fn streq(a: &str, b: &str) -> bool {
    a == b
}