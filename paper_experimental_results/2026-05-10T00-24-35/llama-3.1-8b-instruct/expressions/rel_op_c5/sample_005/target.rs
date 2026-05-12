fn main() {
    let mut x: i32 = 0;
    let px1 = &mut x as *mut i32;
    let px2 = &mut x as *mut i32;
    if px1 != px2 {
        panic!("Expected px1 to be equal to px2, but they are not");
    }

    let mut arr = [1, 2, 3];
    let p_end1 = arr.as_ptr().offset(3 * std::mem::size_of::<i32>()) as *const i32;
    let p_end2 = arr.as_ptr().add(3);
    if p_end1 != p_end2 {
        panic!("Expected p_end1 to be equal to p_end2, but they are not");
    }

    let p0 = arr.as_ptr();
    let p2 = arr.as_ptr().offset(2 * std::mem::size_of::<i32>());
    if !(p2 > p0) {
        panic!("Expected p2 to be greater than p0, but it is not");
    }
    if !(p0 < p2) {
        panic!("Expected p0 to be less than p2, but it is not");
    }

    let q_last = arr.as_ptr().offset(2 * std::mem::size_of::<i32>());
    let q1 = q_last.offset(1 * std::mem::size_of::<i32>());
    if !(q1 > p0) {
        panic!("Expected q1 to be greater than p0, but it is not");
    }

    let s = (0, 0);
    let sa = s.0 as *const i32 as *const u8;
    let sb = s.1 as *const i32 as *const u8;
    if !(sb > sa) {
        panic!("Expected sb to be greater than sa, but it is not");
    }

    let u = 0;
    let ui = &mut u as *mut i32 as *const u8;
    let ud = &mut u as *mut i32 as *const u8;
    if ui != ud {
        panic!("Expected ui to be equal to ud, but they are not");
    }
}