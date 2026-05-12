const X1: i32 = 7;
const X2: &str = "hi";

const FMT: &str = "x1= %d, x2= %s";
const INC: &str = "vers2.h";
const A: &str = "hello";
const B: &str = "hello, world";

fn main() {
    if X1 != 7 {
        std::process::exit(1);
    }
    if X2.chars().nth(0) != Some('h') {
        std::process::exit(2);
    }
    if X2.chars().nth(1) != Some('i') {
        std::process::exit(3);
    }
    if X2.chars().nth(2) != None {
        std::process::exit(4);
    }

    if FMT.len() != 15 {
        std::process::exit(5);
    }
    if FMT.chars().nth(0) != Some('x') {
        std::process::exit(6);
    }
    if FMT.chars().nth(1) != Some('1') {
        std::process::exit(7);
    }
    if FMT.chars().nth(2) != Some('=') {
        std::process::exit(8);
    }
    if FMT.chars().nth(3) != Some(' ') {
        std::process::exit(9);
    }
    if FMT.chars().nth(4) != Some('%') {
        std::process::exit(10);
    }
    if FMT.chars().nth(5) != Some('d') {
        std::process::exit(11);
    }
    if FMT.chars().nth(6) != Some(',') {
        std::process::exit(12);
    }
    if FMT.chars().nth(7) != Some(' ') {
        std::process::exit(13);
    }
    if FMT.chars().nth(8) != Some('x') {
        std::process::exit(14);
    }
    if FMT.chars().nth(9) != Some('2') {
        std::process::exit(15);
    }
    if FMT.chars().nth(10) != Some('=') {
        std::process::exit(16);
    }
    if FMT.chars().nth(11) != Some(' ') {
        std::process::exit(17);
    }
    if FMT.chars().nth(12) != Some('%') {
        std::process::exit(18);
    }
    if FMT.chars().nth(13) != Some('s') {
        std::process::exit(19);
    }
    if FMT.chars().nth(14) != None {
        std::process::exit(20);
    }

    if INC.len() != 7 {
        std::process::exit(21);
    }
    if INC.chars().nth(0) != Some('v') {
        std::process::exit(22);
    }
    if INC.chars().nth(1) != Some('e') {
        std::process::exit(23);
    }
    if INC.chars().nth(2) != Some('r') {
        std::process::exit(24);
    }
    if INC.chars().nth(3) != Some('s') {
        std::process::exit(25);
    }
    if INC.chars().nth(4) != Some('2') {
        std::process::exit(26);
    }
    if INC.chars().nth(5) != Some('.') {
        std::process::exit(27);
    }
    if INC.chars().nth(6) != Some('h') {
        std::process::exit(28);
    }
    if INC.chars().nth(7) != None {
        std::process::exit(29);
    }

    if A.len() != 5 {
        std::process::exit(30);
    }
    if A.chars().nth(0) != Some('h') {
        std::process::exit(31);
    }
    if A.chars().nth(1) != Some('e') {
        std::process::exit(32);
    }
    if A.chars().nth(2) != Some('l') {
        std::process::exit(33);
    }
    if A.chars().nth(3) != Some('l') {
        std::process::exit(34);
    }
    if A.chars().nth(4) != Some('o') {
        std::process::exit(35);
    }
    if A.chars().nth(5) != None {
        std::process::exit(36);
    }

    if B.len() != 12 {
        std::process::exit(37);
    }
    if B.chars().nth(0) != Some('h') {
        std::process::exit(38);
    }
    if B.chars().nth(1) != Some('e') {
        std::process::exit(39);
    }
    if B.chars().nth(2) != Some('l') {
        std::process::exit(40);
    }
    if B.chars().nth(3) != Some('l') {
        std::process::exit(41);
    }
    if B.chars().nth(4) != Some('o') {
        std::process::exit(42);
    }
    if B.chars().nth(5) != Some(',') {
        std::process::exit(43);
    }
    if B.chars().nth(6) != Some(' ') {
        std::process::exit(44);
    }
    if B.chars().nth(7) != Some('w') {
        std::process::exit(45);
    }
    if B.chars().nth(8) != Some('o') {
        std::process::exit(46);
    }
    if B.chars().nth(9) != Some('r') {
        std::process::exit(47);
    }
    if B.chars().nth(10) != Some('l') {
        std::process::exit(48);
    }
    if B.chars().nth(11) != Some('d') {
        std::process::exit(49);
    }
    if B.chars().nth(12) != None {
        std::process::exit(50);
    }
}