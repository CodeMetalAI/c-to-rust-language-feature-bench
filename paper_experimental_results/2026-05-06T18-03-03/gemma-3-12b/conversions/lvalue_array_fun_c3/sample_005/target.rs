fn main() {
    let a = [10, 20, 30];

    if type_id(&a) != 2 {
        return 1;
    }

    if type_id(&a as *const [i32]) != 2 {
        return 2;
    }

    if a[0] != 10 {
        return 3;
    }
    if a[1] != 20 {
        return 4;
    }
    if a[2] != 30 {
        return 5;
    }

    if std::mem::size_of(&a) != 3 * std::mem::size_of(i32) {
        return 6;
    }
    if std::mem::align_of(&a) != std::mem::align_of(i32) {
        return 7;
    }

    let s = "abc".as_bytes();
    if type_id(s as *const u8) != 3 {
        return 8;
    }
    if s[0] != b'a' {
        return 9;
    }
    if s[1] != b'b' {
        return 10;
    }
    if s[2] != b'c' {
        return 11;
    }
    if s[3] != 0 {
        return 12;
    }

    0
}

#[allow(dead_code)]
fn type_id(x: *const u8) -> i32 {
    if x.is_null() {
        return 99;
    }
    let ptr = unsafe { &*x };
    if let Some(arr) = ptr.as_ref::<[i32]>() {
        if arr.len() == 3 {
            return 2;
        }
    }
    if let Some(str) = ptr.as_ref::<&str>() {
        if str.len() == 3 {
            return 3;
        }
    }
    if let Some(str_arr) = ptr.as_ref::<&[u8]>(){
        if str_arr.len() == 3 {
            return 3;
        }
    }
    if let Some(arr_ptr) = ptr.as_ref::<*const [i32]>() {
        return 1;
    }

    99
}