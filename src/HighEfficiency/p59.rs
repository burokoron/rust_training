fn main() {
    for i in 1..51 {
        if i % 10 == 3 || i / 10 == 3 || i % 3 == 0 {
            println!("A");
        } else {
            println!("{}", i);
        }
    }
}
