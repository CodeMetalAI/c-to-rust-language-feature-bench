fn main() {
    let x1 = 7;
    let x2 = "hi";
    let fmt = format!("x1= %d, x2= %s", 1, 2);
    let inc = format!("vers2.h");
    let a = "hello";
    let b = "hello, world";

    if x1!= 7 {
        std::process::exit(1);
    }
    if x2.chars().next().unwrap()!= 'h' {
        std::process::exit(2);
    }
    if x2.chars().nth(1).unwrap()!= 'i' {
        std::process::exit(3);
    }
    if x2.chars().nth(2).is_some() {
        std::process::exit(4);
    }

    if fmt.len()!= 15 {
        std::process::exit(5);
    }
    if fmt.chars().nth(0).unwrap()!= 'x' {
        std::process::exit(6);
    }
    if fmt.chars().nth(1).unwrap()!= '1' {
        std::process::exit(7);
    }
    if fmt.chars().nth(2).unwrap()!= '=' {
        std::process::exit(8);
    }
    if fmt.chars().nth(3).unwrap()!= ' ' {
        std::process::exit(9);
    }
    if fmt.chars().nth(4).unwrap()!= '%' {
        std::process::exit(10);
    }
    if fmt.chars().nth(5).unwrap()!= 'd' {
        std::process::exit(11);
    }
    if fmt.chars().nth(6).unwrap()!= ',' {
        std::process::exit(12);
    }
    if fmt.chars().nth(7).unwrap()!= ' ' {
        std::process::exit(13);
    }
    if fmt.chars().nth(8).unwrap()!= 'x' {
        std::process::exit(14);
    }
    if fmt.chars().nth(9).unwrap()!= '2' {
        std::process::exit(15);
    }
    if fmt.chars().nth(10).unwrap()!= '=' {
        std::process::exit(16);
    }
    if fmt.chars().nth(11).unwrap()!= ' ' {
        std::process::exit(17);
    }
    if fmt.chars().nth(12).unwrap()!= '%' {
        std::process::exit(18);
    }
    if fmt.chars().nth(13).unwrap()!= 's' {
        std::process::exit(19);
    }
    if fmt.chars().nth(14).is_some() {
        std::process::exit(20);
    }

    if inc.len()!= 8 {
        std::process::exit(21);
    }
    if inc.chars().nth(0).unwrap()!= 'v' {
        std::process::exit(22);
    }
    if inc.chars().nth(1).unwrap()!= 'e' {
        std::process::exit(23);
    }
    if inc.chars().nth(2).unwrap()!= 'r' {
        std::process::exit(24);
    }
    if inc.chars().nth(3).unwrap()!= 's' {
        std::process::exit(25);
    }
    if inc.chars().nth(4).unwrap()!= '2' {
        std::process::exit(26);
    }
    if inc.chars().nth(5).unwrap()!= '.' {
        std::process::exit(27);
    }
    if inc.chars().nth(6).unwrap()!= 'h' {
        std::process::exit(28);
    }
    if inc.chars().nth(7).is_some() {
        std::process::exit(29);
    }

    if a.len()!= 5 {
        std::process::exit(30);
    }
    if a.chars().nth(0).unwrap()!= 'h' {
        std::process::exit(31);
    }
    if a.chars().nth(1).unwrap()!= 'e' {
        std::process::exit(32);
    }
    if a.chars().nth(2).unwrap()!= 'l' {
        std::process::exit(33);
    }
    if a.chars().nth(3).unwrap()!= 'l' {
        std::process::exit(34);
    }
    if a.chars().nth(4).unwrap()!= 'o' {
        std::process::exit(35);
    }

    if b.len()!= 12 {
        std::process::exit(36);
    }
    if b.chars().nth(0).unwrap()!= 'h' {
        std::process::exit(37);
    }
    if b.chars().nth(1).unwrap()!= 'e' {
        std::process::exit(38);
    }
    if b.chars().nth(2).unwrap()!= 'l' {
        std::process::exit(39);
    }
    if b.chars().nth(3).unwrap()!= 'l' {
        std::process::exit(40);
    }
    if b.chars().nth(4).unwrap()!= 'o' {
        std::process::exit(41);
    }
    if b.chars().nth(5).unwrap()!= ',' {
        std::process::exit(42);
    }
    if b.chars().nth(6).unwrap()!= ' ' {
        std::process::exit(43);
    }
    if b.chars().nth(7).unwrap()!= 'w' {
        std::process::exit(44);
    }
    if b.chars().nth(8).unwrap()!= 'o' {
        std::process::exit(45);
    }
    if b.chars().nth(9).unwrap()!= 'r' {
        std::process::exit(46);
    }
    if b.chars().nth(10).unwrap()!= 'l' {
        std::process::exit(47);
    }
    if b.chars().nth(11).unwrap()!= 'd' {
        std::process::exit(48);
    }

    std::process::exit(0);
}