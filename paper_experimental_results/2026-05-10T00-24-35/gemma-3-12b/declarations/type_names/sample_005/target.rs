fn main() {
    // Check sizeof((int){0}) != sizeof(int)
    assert!(std::mem::size_of::<i32>() == std::mem::size_of::<i32>());

    // Check *(int *)&g_arr[0] != 1
    let g_arr: [i32; 3] = [1, 2, 3];
    assert!(g_arr[0] == 1);

    // Check *q != 3
    let g_arr: [i32; 3] = [1, 2, 3];
    let q = &g_arr[2];
    assert!(*q == 3);

    // Check (*pa)[1] != 2
    let g_arr: [i32; 3] = [1, 2, 3];
    let pa: &[[i32; 3]] = &g_arr;
    assert!((*pa)[1] == 2);

    // Check use_vla(n, (int (*)[n]) & g_arr) != 13
    let g_arr: [i32; 3] = [1, 2, 3];
    let n = 3;
    let r = g_arr[0] * 10 + g_arr[n - 1];
    assert!(r == 13);

    // Check *pf() != 2
    let g_arr: [i32; 3] = [1, 2, 3];
    let pf = || &g_arr[1];
    assert!(*pf() == 2);

    // Check pf() != 7
    let pf = || 7;
    assert!(pf() == 7);

    // Check af[0](4u, 0) != 7
    let pf1 = |x: u32| (x + 3) as i32;
    let af: &[(u32) -> i32] = &[pf1, pf1];
    assert!(af[0](4) == 7);

    // Check af[1](10u, 1, 2, 3) != 13
    let pf2 = |x: u32, a: i32, b: i32, c: i32| (x + 3) as i32;
    let af: &[(u32, i32, i32, i32) -> i32] = &[pf2, pf2];
    assert!(af[1](10, 1, 2, 3) == 13);
}