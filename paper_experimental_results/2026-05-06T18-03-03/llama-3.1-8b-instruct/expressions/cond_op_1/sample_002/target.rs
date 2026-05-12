fn f<T: Copy>(p: Option<&T>) {
    match p {
        Some(_) => (),
        None => (),
    }
}

fn f1<T: Copy>(p: Option<&T>) {
    match p {
        Some(_) => (),
        None => (),
    }
}

fn f2<T: Copy>(p: Option<&T>) {
    match p {
        Some(_) => (),
        None => (),
    }
}

fn f3<T: Copy>(p: Option<&T>) {
    match p {
        Some(_) => (),
        None => (),
    }
}

fn f4<T: Copy>(p: Option<&T>) {
    match p {
        Some(_) => (),
        None => (),
    }
}

fn main() {
    f(Some(&1));
    f(Some(&1));

    f2(Some(&1));
    f2(Some(&1));

    f3(Some(&1));
    f3(Some(&1));

    f(Some(&1));
    f(Some(&1));

    f4(Some(&1));
    f4(Some(&1));

    f1(Some(&1));
    f1(Some(&1));

    return 0;
}