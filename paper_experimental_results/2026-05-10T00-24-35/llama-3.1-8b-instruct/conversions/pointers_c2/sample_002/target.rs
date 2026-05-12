fn main() {
    let x: i32 = 17;
    let p = &x;

    let pc = p;
    let pv: *const i32 = p;
    let pcv: *const i32 = pv;

    if pc != p {
        panic!("pc != p");
    }
    if pv != p {
        panic!("pv != p");
    }
    if pcv != p {
        panic!("pcv != p");
    }

    if *pc != 17 {
        panic!("*pc != 17");
    }
    if *pv != 17 {
        panic!("*pv != 17");
    }
    if *pcv != 17 {
        panic!("*pcv != 17");
    }

    assert_eq!(pc as *const _, p as *const _);
    assert_eq!(pv as *const _, p as *const _);
    assert_eq!(pcv as *const _, p as *const _);
    assert_eq!(*pc, 17);
    assert_eq!(*pv, 17);
    assert_eq!(*pcv, 17);
}