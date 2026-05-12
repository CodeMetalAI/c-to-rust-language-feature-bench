fn main() {
    let x1 = 7;
    let x2 = "hi";
    let fmt = format!("x1= %d, x2= %s", 1, 2);
    let inc = format!("vers2.h");
    let high = "hello";
    let low = format!("{}, world", high);
    let a = format!("{}{}", high, low);
    let b = format!("{}{}", high, low);

    if x1!= 7 {
        std::process::exit(1);
    }
    if &x2[0..1]!= "h" {
        std::process::exit(2);
    }
    if &x2[1..2]!= "i" {
        std::process::exit(3);
    }
    if x2.len()!= 2 {
        std::process::exit(4);
    }

    if fmt.len()!= 15 {
        std::process::exit(5);
    }
    if &fmt[0..1]!= "x" {
        std::process::exit(6);
    }
    if &fmt[1..2]!= "1" {
        std::process::exit(7);
    }
    if &fmt[2..3]!= "=" {
        std::process::exit(8);
    }
    if &fmt[3..4]!= " " {
        std::process::exit(9);
    }
    if &fmt[4..5]!= "%" {
        std::process::exit(10);
    }
    if &fmt[5..6]!= "d" {
        std::process::exit(11);
    }
    if &fmt[6..7]!= "," {
        std::process::exit(12);
    }
    if &fmt[7..8]!= " " {
        std::process::exit(13);
    }
    if &fmt[8..9]!= "x" {
        std::process::exit(14);
    }
    if &fmt[9..10]!= "2" {
        std::process::exit(15);
    }
    if &fmt[10..11]!= "=" {
        std::process::exit(16);
    }
    if &fmt[11..12]!= " " {
        std::process::exit(17);
    }
    if &fmt[12..13]!= "%" {
        std::process::exit(18);
    }
    if &fmt[13..14]!= "s" {
        std::process::exit(19);
    }
    if fmt.len()!= 14 {
        std::process::exit(20);
    }

    if inc.len()!= 7 {
        std::process::exit(21);
    }
    if &inc[0..1]!= "v" {
        std::process::exit(22);
    }
    if &inc[1..2]!= "e" {
        std::process::exit(23);
    }
    if &inc[2..3]!= "r" {
        std::process::exit(24);
    }
    if &inc[3..4]!= "s" {
        std::process::exit(25);
    }
    if &inc[4..5]!= "2" {
        std::process::exit(26);
    }
    if &inc[5..6]!= "." {
        std::process::exit(27);
    }
    if &inc[6..7]!= "h" {
        std::process::exit(28);
    }

    if a.len()!= 5 {
        std::process::exit(30);
    }
    if &a[0..1]!= "h" {
        std::process::exit(31);
    }
    if &a[1..2]!= "e" {
        std::process::exit(32);
    }
    if &a[2..3]!= "l" {
        std::process::exit(33);
    }
    if &a[3..4]!= "l" {
        std::process::exit(34);
    }
    if &a[4..5]!= "o" {
        std::process::exit(35);
    }

    if b.len()!= 12 {
        std::process::exit(37);
    }
    if &b[0..1]!= "h" {
        std::process::exit(38);
    }
    if &b[1..2]!= "e" {
        std::process::exit(39);
    }
    if &b[2..3]!= "l" {
        std::process::exit(40);
    }
    if &b[3..4]!= "l" {
        std::process::exit(41);
    }
    if &b[4..5]!= "o" {
        std::process::exit(42);
    }
    if &b[5..6]!= "," {
        std::process::exit(43);
    }
    if &b[6..7]!= " " {
        std::process::exit(44);
    }
    if &b[7..8]!= "w" {
        std::process::exit(45);
    }
    if &b[8..9]!= "o" {
        std::process::exit(46);
    }
    if &b[9..10]!= "r" {
        std::process::exit(47);
    }
    if &b[10..11]!= "l" {
        std::process::exit(48);
    }
    if &b[11..12]!= "d" {
        std::process::exit(49);
    }

    std::process::exit(0);
}