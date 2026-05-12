fn main() {
    let x = 7;
    let y = 3;

    if count_args("Flag")!= 1 {
        return 1;
    }
    if count_args("X = %d\n", x)!= 2 {
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

fn count_args<T, Args>(_t: T, _args: Args) -> usize
where
    Args: std::iter::FromIterator<T>,
{
    std::iter::once(_t).chain(_args.into_iter()).count()
}

fn streq(a: &str, b: &str) -> bool {
    a.chars().zip(b.chars()).all(|(a, b)| a == b) && a.len() == b.len()
}

fn report<T, Args>(test: bool, _t: T, _args: Args) -> usize
where
    Args: std::iter::FromIterator<T>,
{
    if test {
        1
    } else {
        std::iter::once(_t).chain(_args.into_iter()).count()
    }
}