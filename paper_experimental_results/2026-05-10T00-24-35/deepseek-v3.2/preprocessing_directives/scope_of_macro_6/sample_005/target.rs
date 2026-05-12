/* scope_of_macro_6.c */
const OBJ_LIKE: i32 = 1 - 1;
const _OBJ_LIKE_DUP: i32 = 1 - 1; // duplicate definition

fn func_like(a: i32) -> i32 {
    a
}
fn _func_like_dup(a: i32) -> i32 {
    a
}

#[cfg(NEGATIVE)]
const OBJ_LIKE_NEG: i32 = 0;
#[cfg(NEGATIVE)]
const OBJ_LIKE_NEG2: i32 = 1e - 1;
#[cfg(NEGATIVE)]
fn func_like_neg(b: i32) -> i32 {
    a // This would be a compile error in Rust due to undefined 'a'
}
#[cfg(NEGATIVE)]
fn func_like_neg2(b: i32) -> i32 {
    b
}

fn main() {
    let x = OBJ_LIKE;
    let y = func_like(3);

    if x != 0 {
        std::process::exit(1);
    }
    if y != 3 {
        std::process::exit(2);
    }

    std::process::exit(0);
}