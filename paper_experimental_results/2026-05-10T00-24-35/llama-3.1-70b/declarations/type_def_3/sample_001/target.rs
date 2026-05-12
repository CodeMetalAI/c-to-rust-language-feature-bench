fn main() {
    #[repr(C)]
    struct Tag {
        t: u8,
        _pad1: [u8; 1],
        r: i8,
        _pad2: [u8; 2],
    }

    fn id(x: i32) -> i32 { x }

    fn call_cb(cb: fn(i32) -> i32, x: i32) -> i32 { cb(x) }

    fn use_bits(s: Tag) -> i32 {
        let a = s.t as i32;
        let b = s.r as i32;
        a * 100 + b
    }

    let mut s = Tag { t: 15, _pad1: [0], r: 31, _pad2: [0; 2] };

    if use_bits(s)!= 1517 {
        return;
    }

    let t: i64 = 1234;

    if t!= 1234 {
        return;
    }

    if call_cb(id, 7)!= 7 {
        return;
    }

    if f(id)!= 9 {
        return;
    }

    fn f(pf: fn(i32) -> i32) -> i32 { call_cb(pf, 9) }
}