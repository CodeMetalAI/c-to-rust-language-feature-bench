fn f(x: i32) -> i32 {
    x
}

fn main() {
    let p0: Option<fn(i32) -> i32> = None;
    let mut v0: Option<Box<dyn Fn(i32) -> i32>> = None;

    let fp0: fn(i32) -> i32 = f;
    let p0 = Some(fp0);
    let v0 = Some(Box::new(fp0) as Box<dyn Fn(i32) -> i32>);

    let p1 = p0;
    let v1 = v0;

    let p2 = p1;
    let v2 = v1;

    if p0 != p1 {
        return 1;
    }

    if v0 != v1 {
        return 2;
    }

    if v0 != v2 {
        return 3;
    }

    if p0 != p2 {
        return 4;
    }

    if fp0 != fp0 {
        return 5;
    }

    if fp0 != fp0 {
        return 6;
    }

    if p0 != Some(fp0) {
        return 7;
    }

    if v0 != Some(Box::new(fp0) as Box<dyn Fn(i32) -> i32>) {
        return 8;
    }

    return 0;
}