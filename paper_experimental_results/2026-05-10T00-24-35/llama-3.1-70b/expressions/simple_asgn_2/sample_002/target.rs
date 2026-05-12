fn main() {
    let mut c: u8 = 0;
    let mut i: i32 = 0;
    let mut l: i64 = 0;

    i = 100;

    l = (c = i as u8) as i64;

    if c as i32!= i as i32 {
        return;
    }
    if l as i64!= c as i64 {
        return;
    }
    if l as i64!= (i as u8) as i64 {
        return;
    }
}