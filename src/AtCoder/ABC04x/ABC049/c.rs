use proconio::input;

fn main() {
    fn check(a: &Vec<char>, b: &Vec<char>) -> bool {
        assert_eq!(a.len(), b.len());
 
        for i in 0..a.len() {
            if a[i] != b[i] {
                return false;
            }
        }
        return true;
    }
 
    input! {
        s: proconio::marker::Chars,
    }
 
    let dream: Vec<char> = "dream".chars().collect();
    let dreamer: Vec<char> = "dreamer".chars().collect();
    let dreamera: Vec<char> = "dreamera".chars().collect();
    let erase: Vec<char> = "erase".chars().collect();
    let eraser: Vec<char> = "eraser".chars().collect();
 
    let mut i = 0;
    while i < s.len() {
        if s.len() - i >= 10 {
            if check(&s[i..i+8].to_vec(), &dreamera) {
                i += 5;
            } else if check(&s[i..i+7].to_vec(), &dreamer) {
                i += 7;
            } else if check(&s[i..i+6].to_vec(), &eraser) {
                i += 6;
            } else if check(&s[i..i+5].to_vec(), &dream) || check(&s[i..i+5].to_vec(), &erase) {
                i += 5;
            } else {
                println!("NO");
                return;
            }
        } else if s.len() - i == 7 {
            if !check(&s[i..i+7].to_vec(), &dreamer) {
                println!("NO");
                return;
            }
            i += 7
        } else if s.len() - i == 6 {
            if !check(&s[i..i+6].to_vec(), &eraser) {
                println!("NO");
                return;
            }
            i += 6;
        } else if s.len() - i == 5 {
            if !check(&s[i..i+5].to_vec(), &dream) && !check(&s[i..i+5].to_vec(), &erase) {
                println!("NO");
                return;
            }
            i += 5;
        } else {
            println!("NO");
            return;
        }
    }
 
    println!("YES");
}
