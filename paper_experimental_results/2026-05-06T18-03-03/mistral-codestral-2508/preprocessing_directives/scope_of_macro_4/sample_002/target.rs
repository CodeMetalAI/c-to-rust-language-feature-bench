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
    if x2.chars().nth(0) != Some('h') {
        std::process::exit(2);
    }
    if x2.chars().nth(1) != Some('i') {
        std::process::exit(3);
    }
    if x2.chars().nth(2) != Some('\0') {
        std::process::exit(4);
    }

    if fmt.len() != 15 {
        std::process::exit(5);
    }
    if fmt.chars().nth(0) != Some('x') {
        std::process::exit(6);
    }
    if fmt.chars().nth(1) != Some('1') {
        std::process::exit(7);
    }
    if fmt.chars().nth(2) != Some('=') {
        std::process::exit(8);
    }
    if fmt.chars().nth(3) != Some(' ') {
        std::process::exit(9);
    }
    if fmt.chars().nth(4) != Some('%') {
        std::process::exit(10);
    }
    if fmt.chars().nth(5) != Some('d') {
        std::process::exit(11);
    }
    if fmt.chars().nth(6) != Some(',') {
        std::process::exit(12);
    }
    if fmt.chars().nth(7) != Some(' ') {
        std::process::exit(13);
    }
    if fmt.chars().nth(8) != Some('x') {
        std::process::exit(14);
    }
    if fmt.chars().nth(9) != Some('2') {
        std::process::exit(15);
    }
    if fmt.chars().nth(10) != Some('=') {
        std::process::exit(16);
    }
    if fmt.chars().nth(11) != Some(' ') {
        std::process::exit(17);
    }
    if fmt.chars().nth(12) != Some('%') {
        std::process::exit(18);
    }
    if fmt.chars().nth(13) != Some('s') {
        std::process::exit(19);
    }
    if fmt.chars().nth(14) != Some('\0') {
        std::process::exit(20);
    }

    if inc.len() != 8 {
        std::process::exit(21);
    }
    if inc.chars().nth(0) != Some('v') {
        std::process::exit(22);
    }
    if inc.chars().nth(1) != Some('e') {
        std::process::exit(23);
    }
    if inc.chars().nth(2) != Some('r') {
        std::process::exit(24);
    }
    if inc.chars().nth(3) != Some('s') {
        std::process::exit(25);
    }
    if inc.chars().nth(4) != Some('2') {
        std::process::exit(26);
    }
    if inc.chars().nth(5) != Some('.') {
        std::process::exit(27);
    }
    if inc.chars().nth(6) != Some('h') {
        std::process::exit(28);
    }
    if inc.chars().nth(7) != Some('\0') {
        std::process::exit(29);
    }

    if a.len() != 6 {
        std::process::exit(30);
    }
    if a.chars().nth(0) != Some('h') {
        std::process::exit(31);
    }
    if a.chars().nth(1) != Some('e') {
        std::process::exit(32);
    }
    if a.chars().nth(2) != Some('l') {
        std::process::exit(33);
    }
    if a.chars().nth(3) != Some('l') {
        std::process::exit(34);
    }
    if a.chars().nth(4) != Some('o') {
        std::process::exit(35);
    }
    if a.chars().nth(5) != Some('\0') {
        std::process::exit(36);
    }

    if b.len() != 13 {
        std::process::exit(37);
    }
    if b.chars().nth(0) != Some('h') {
        std::process::exit(38);
    }
    if b.chars().nth(1) != Some('e') {
        std::process::exit(39);
    }
    if b.chars().nth(2) != Some('l') {
        std::process::exit(40);
    }
    if b.chars().nth(3) != Some('l') {
        std::process::exit(41);
    }
    if b.chars().nth(4) != Some('o') {
        std::process::exit(42);
    }
    if b.chars().nth(5) != Some(',') {
        std::process::exit(43);
    }
    if b.chars().nth(6) != Some(' ') {
        std::process::exit(44);
    }
    if b.chars().nth(7) != Some('w') {
        std::process::exit(45);
    }
    if b.chars().nth(8) != Some('o') {
        std::process::exit(46);
    }
    if b.chars().nth(9) != Some('r') {
        std::process::exit(47);
    }
    if b.chars().nth(10) != Some('l') {
        std::process::exit(48);
    }
    if b.chars().nth(11) != Some('d') {
        std::process::exit(49);
    }
    if b.chars().nth(12) != Some('\0') {
        std::process::exit(50);
    }
}