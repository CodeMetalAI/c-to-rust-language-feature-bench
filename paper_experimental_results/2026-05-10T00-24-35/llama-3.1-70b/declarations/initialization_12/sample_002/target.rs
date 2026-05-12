fn main() {
    let a = [1, 3, 5, 7, 9, 0, 8, 6, 4, 2, 0, 0];

    if a[0]!= 1 {
        return;
    }
    if a[1]!= 3 {
        return;
    }
    if a[2]!= 5 {
        return;
    }
    if a[3]!= 7 {
        return;
    }
    if a[4]!= 9 {
        return;
    }

    if a[7]!= 8 {
        return;
    }
    if a[8]!= 6 {
        return;
    }
    if a[9]!= 4 {
        return;
    }
    if a[10]!= 2 {
        return;
    }
    if a[11]!= 0 {
        return;
    }

    {
        let mut i = 5;
        while i < 7 {
            if a[i]!= 0 {
                return;
            }
            i += 1;
        }
    }
}