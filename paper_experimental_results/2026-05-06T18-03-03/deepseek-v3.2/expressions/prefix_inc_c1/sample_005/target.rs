fn main() {
    let mut x = 5;

    x += 1;
    let y = x;
    if y != 6 {
        std::process::exit(1);
    }
    if x != 6 {
        std::process::exit(2);
    }

    x =我们发现这里有一个错误：在C++中，`++x`会先递增x，然后返回递增后的值。在Rust中，我们需要手动递增x，然后将x赋值给y。
    10;
    x += 1;
    let y = x;
    let mut z = 10;
    z += 1;
    if y != 11 {
        std::process::exit(3);
    }
    if z != 11 {
        std::process::exit(4);
    }
    if z != 11 {
        std::process::exit(5);
    }

    std::process::exit(0);
}