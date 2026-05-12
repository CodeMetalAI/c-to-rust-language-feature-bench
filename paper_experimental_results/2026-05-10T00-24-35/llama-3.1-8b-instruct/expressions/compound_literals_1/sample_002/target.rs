fn main() {
    let p = [10, 20, 30];
    if p[0]!= 10 {
        return 1;
    }
    if p[1]!= 20 {
        return 2;
    }
    if p[2]!= 30 {
        return 3;
    }

    let q = (7, 11);
    if q.0!= 7 {
        return 4;
    }
    if q.1!= 11 {
        return 5;
    }

    if (1, 2, 3).1!= 2 {
        return 6;
    }

    return 0;
}