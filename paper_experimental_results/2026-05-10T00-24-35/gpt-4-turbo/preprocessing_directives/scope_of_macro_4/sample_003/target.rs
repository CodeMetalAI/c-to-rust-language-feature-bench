fn main() {
    let x1 = 7;
    let x2 = "hi";

    let fmt = "x1= %d, x2= %s";
    let inc = "vers2.h";
    let a = "hello";
    let b = "hello, world";

    if x1 != 7 {
        std::process::exit(1);
    }
    if x2.as_bytes()[0] != b'h' {
        std::process::exit(2);
    }
    if x2.as_bytes()[1] != b'i' {
        std::process::exit(3);
    }
    if x2.len() != 2 {
        std::process::exit(4);
    }

    if fmt.len() != 14 {
        std::process::exit(5);
    }
    assert_char_sequence(fmt, &[
        ('x', 6), ('1', 7), ('=', 8), (' ', 9), ('%', 10), ('d', 11), (',', 12), (' ', 13), ('x', 14),
        ('2', 15), ('=', 16), (' ', 17), ('%', 18), ('s', 19)
    ], 6);

    if inc.len() != 7 {
        std::process::exit(21);
    }
    assert_char_sequence(inc, &[
        ('v', 22), ('e', 23), ('r', 24), ('s', 25), ('2', 26), ('.', 27), ('h', 28)
    ], 22);

    if a.len() != 5 {
        std::process::exit(30);
    }
    assert_char_sequence(a, &[
        ('h', 31), ('e', 32), ('l', 33), ('l', 34), ('o', 35)
    ], 31);

    if b.len() != 12 {
        std::process::exit(37);
    }
    assert_char_sequence(b, &[
        ('h', 38), ('e', 39), ('l', 40), ('l', 41), ('o', 42), (',', 43), (' ', 44), ('w', 45),
        ('o', 46), ('r', 47), ('l', 48), ('d', 49)
    ], 38);
}

fn assert_char_sequence(s: &str, checks: &[(char, u32)], base_exit_code: u32) {
    for (i, &(ch, exit_code)) in checks.iter().enumerate() {
        if s.as_bytes()[i] as char != ch {
            std::process::exit(exit_code);
        }
    }
    if s.len() != checks.len() {
        std::process::exit(base_exit_code + checks.len() as u32);
    }
}