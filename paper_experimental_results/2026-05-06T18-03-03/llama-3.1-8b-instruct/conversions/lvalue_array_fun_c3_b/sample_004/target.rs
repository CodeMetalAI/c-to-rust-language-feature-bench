#![allow(unused_imports)]

use std::mem;

fn type_id<T>(x: &T) -> u32 {
    match x {
        _ => 99,
    }
}

fn main() {
    let a = [10, 20, 30];
    if type_id(&a) != 2 {
        return;
    }
    if type_id(&a as *const [i32]) != 2 {
        return;
    }

    if a[0] != 10 {
        return;
    }
    if a[1] != 20 {
        return;
    }
    if a[2] != 30 {
        return;
    }

    if a.len() as usize != 3 {
        return;
    }
    if mem::size_of_val(&a) != mem::size_of::<i32>() * 3 {
        return;
    }

    let s = "abc";
    if type_id(s) != 3 {
        return;
    }
    if s.chars().nth(0).unwrap() != 'a' {
        return;
    }
    if s.chars().nth(1).unwrap() != 'b' {
        return;
    }
    if s.chars().nth(2).unwrap() != 'c' {
        return;
    }
    if s.chars().nth(3).unwrap() != '\0' {
        return;
    }

    println!("Success");
    std::process::exit(0);
}