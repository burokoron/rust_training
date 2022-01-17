use proconio::input;

fn main() {
    input! {
        _: usize,
        mut s: proconio::marker::Chars,
    }
 
    loop {
        let mut count = 0;
        for &i in &s {
            match i {
                '(' => count += 1,
                ')' => count -= 1,
                _ => unreachable!(),
            }
            if count < 0 {
                break;
            }
        }
        match count {
            0 => break,
            -1 => s.insert(0, '('),
            _ => s.push(')'),
        }
    }
 
    let ans: String = s.into_iter().collect();
    println!("{}", ans);
}
