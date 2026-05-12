fn main() {
    let np1: Option<&i32> = None;
    let np2: Option<&i32> = None;
    if np1 != np2 {
        return;
    }

    struct S {
        x: i32,
        y: i32,
    }
    let s = S { x: 0, y: 0 };
    let ps: &S = &s;
    let px: &i32 = &s.x;
    let ps_ptr = ps as *const S;
    let px_ptr = px as *const i32;
    if ps_ptr != px_ptr {
        return;
    }

    let a = [1, 2, 3];
    let end1 = &a[3];
    let end2 = a.as_ptr().add(3);
    if end1 != end2 {
        return;
    }
}