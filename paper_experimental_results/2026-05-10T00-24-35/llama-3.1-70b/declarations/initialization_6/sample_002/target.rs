fn main() {
    let q = [
        [[1, 0], [0, 0], [0, 0]],
        [[2, 3], [0, 0], [0, 0]],
        [[4, 5], [6, 0], [0, 0]],
        [[0, 0], [0, 0], [0, 0]],
    ];

    assert_eq!(q[0][0][0], 1, "return 1");
    assert_eq!(q[0][0][1], 0, "return 2");
    assert_eq!(q[0][1][0], 0, "return 3");
    assert_eq!(q[0][2][1], 0, "return 4");

    assert_eq!(q[1][0][0], 2, "return 5");
    assert_eq!(q[1][0][1], 3, "return 6");
    assert_eq!(q[1][1][0], 0, "return 7");

    assert_eq!(q[2][0][0], 4, "return 8");
    assert_eq!(q[2][0][1], 5, "return 9");
    assert_eq!(q[2][1][0], 6, "return 10");
    assert_eq!(q[2][1][1], 0, "return 11");

    assert_eq!(q[3][0][0], 0, "return 12");
    assert_eq!(q[3][2][1], 0, "return 13");

    let p = &q[0][0][0] as *const _ as usize;
    assert_eq!(unsafe { *(p as *const i16) }, 1, "return 14");
    assert_eq!(unsafe { *(p + 6 as usize) as *const i16 }, 2, "return 15");
    assert_eq!(unsafe { *(p + 7 as usize) as *const i16 }, 3, "return 16");
    assert_eq!(unsafe { *(p + 12 as usize) as *const i16 }, 4, "return 17");
    assert_eq!(unsafe { *(p + 13 as usize) as *const i16 }, 5, "return 18");
    assert_eq!(unsafe { *(p + 14 as usize) as *const i16 }, 6, "return 19");
}