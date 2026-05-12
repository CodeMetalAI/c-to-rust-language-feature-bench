fn main() {
    let a = [10, 20, 30, 40, 50];

    let p0 = &a[0];
    let p2 = &a[2];
    let p4 = &a[4];
    let p_last = &a[4];
    let p_end = &a[5];

    if *p2 != 30 {
        return;
    }
    if *p4 != 50 {
        return;
    }

    let p_back = &a[1];
    if *p_back != 20 {
        return;
    }

    let p_from_end = &a[4];
    if p_from_end as *const i32 != p_last as *const i32 {
        return;
    }
    if *p_from_end != 50 {
        return;
    }

    if (&a[0] as *const i32 as *const i32).offset(5) != p_end as *const i32 {
        return;
    }

    println!("Done");
}