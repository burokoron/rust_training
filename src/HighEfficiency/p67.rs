fn main() {
    for i in 1926..2027 {
        if i < 1989 {
            if i == 1926 {
                println!("{}年は昭和元年", i);
            } else {
                println!("{}年は昭和{}年", i, i - 1925);
            }
        } else if i < 2019 {
            if i == 1989 {
                println!("{}年は平成元年", i);
            } else {
                println!("{}年は平成{}年", i, i - 1988);
            }
        } else if i == 2019 {
            println!("{}年は令和元年", i);
        } else {
            println!("{}年は令和{}年", i, i - 2018);
        }
    }
}
