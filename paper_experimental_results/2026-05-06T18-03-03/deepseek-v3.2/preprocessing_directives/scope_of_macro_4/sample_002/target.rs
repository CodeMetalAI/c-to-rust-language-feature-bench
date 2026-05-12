fn main() {
    let x1: i32 = 7;
    let x2: [char; 3] = ['h', 'i', '\0'];

    let fmt: [char; 15] = [
        'x', '1', '=', ' ', '%', 'd', ',', ' ', 'x', '2', '=', ' ', '%', 's', '\0',
    ];
    let inc: [char; 8] = ['v', 'e', 'r', 's', '2', '.', 'h', '\0'];
    let a: [char; 6] = ['h', 'e', 'l', 'l', 'o', '\0'];
    let b: [char; 13] = [
        'h', 'e', 'l', 'l', 'o', ',', ' ', 'w', 'o', 'r', 'l', 'd', '\0',
    ];

    if x1 != 7 {
        std::process::exit(1);
    }
    if x2[0] != 'h' {
        std::process::exit(2);
    }
    if x2[1] != 'i' {
        std::process::exit(3);
    }
    if x2[2] != '\0' {
        std::process::exit(4);
    }

    if fmt.len() != 15 {
        std::process::exit(5);
    }
    if fmt[0] != 'x' {
        std::process::exit(6);
    }
    if fmt[1] != '1' {
        std::process::exit(7);
    }
    if fmt[2] != '=' {
        std::process::exit(8);
    }
    if fmt[3] != ' ' {
        std::process::exit(9);
    }
    if fmt[4] != '%' {
        std::process::exit(10);
    }
    if fmt[5] != 'd' {
        std::process::exit(11);
    }
    if fmt[6] != ',' {
        std::process::exit(12);
    }
    if fmt[7] != ' ' {
        std::process::exit(13);
    }
    if fmt[8] != 'x' {
        std::process::exit(14);
    }
    if fmt[9] != '2' {
        std::process::exit(15);
    }
    if fmt[10] != '=' {
        std::process::exit(16);
    }
    if fmt[11] != ' ' {
        std::process::exit(17);
    }
    if fmt[12] != '%' {
        std::process::exit(18);
    }
    if fmt[13] != 's' {
        std::process::exit(19);
    }
    if fmt[14] != '\0' {
        std::process::exit(20);
    }

    if inc.len() != 8 {
        std::process::exit(21);
    }
    if inc[0] != 'v' {
        std::process::exit(22);
    }
    if inc[1] != 'e' {
        std::process::exit(23);
    }
    if inc[2] != 'r' {
        std::process::exit(24);
    }
    if inc[3] != 's' {
        std::process::exit(25);
    }
    if inc[4] != '2' {
        std::process::exit(26);
    }
    if inc[5] != '.' {
        std::process::exit(27);
    }
    if inc[6] != 'h' {
        std::process::exit(28);
    }
    if inc[7] != '\0' {
        std::process::exit(29);
    }

    if a.len() != 6 {
        std::process::exit(30);
    }
    if a[0] != 'h' {
        std::process::exit(31);
    }
    if a[1] != 'e' {
        std::process::exit(32);
    }
    if a[2] != 'l' {
        std::process::exit(33);
    }
    if a[3] != 'l' {
        std::process::exit(34);
    }
    if a[4] != 'o' {
        std::process::exit(35);
    }
    if a[5] != '\0' {
        std::process::exit(36);
    }

    if b.len() != 13 {
        std::process::exit(37);
    }
    if b[0] != 'h' {
        std::process::exit(38);
    }
    if b[1] != 'e' {
        std::process::exit(39);
    }
    if b[2] != 'l' {
        std::process::exit(40);
    }
    if b[3] != 'l' {
        std::process::exit(41);
    }
    if b[4] != 'o' {
        std::process::exit(42);
    }
    if b[5] != ',' {
        std::process::exit(43);
    }
    if b[6] != ' ' {
        std::process::exit(44);
    }
    if b[7] != 'w' {
        std::process::exit(45);
    }
    if b[8] != 'o' {
        std::process::exit(46);
    }
    if b[9] != 'r' {
        std::process::exit(47);
    }
    if b[10] != 'l' {
        std::process::exit(48);
    }
    if b[11] != 'd' {
        std::process::exit(49);
    }
    if b[12] != '\0' {
        std::process::exit(50);
    }

    std::process::exit(0);
}