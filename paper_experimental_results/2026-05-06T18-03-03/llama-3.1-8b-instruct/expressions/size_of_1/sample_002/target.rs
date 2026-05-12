fn alloc(n: usize) -> *mut f64 {
    Box::new(vec![0.0; n])
}

fn main() {
    let dp = alloc(1);
    if dp.is_null() {
        return 1;
    }

    *dp = 3.14;
    if **dp != 3.14 {
        drop(dp);
        return 2;
    }

    drop(dp);
    return 0;
}