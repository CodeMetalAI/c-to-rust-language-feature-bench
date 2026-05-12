fn main() {
    let x = 10;
    let y = 20;

    let px = &x;
    let py = &y;

    let px_end = &x + 1;
    let py_end = &y + 1;

    if px!= &x {
        println!("Return code: 1");
        return;
    }

    if px_end <= px {
        println!("Return code: 2");
        return;
    }

    if px_end!= &x + 1 {
        println!("Return code: 3");
        return;
    }

    if py_end <= py {
        println!("Return code: 4");
        return;
    }

    println!("Return code: 0");
}