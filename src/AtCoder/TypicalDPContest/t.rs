use proconio::input;

fn main() {
    // Kitamasa法、線形漸化式の第N項をO(K^2logN)で求める
    struct Kitamasa {
        first_term: Vec<usize>,  // 漸化式の初項
        initial_coefficient: Vec<usize>,  // 漸化式の係数
        mod_number: usize,  // mod数
    }
    
    impl Kitamasa {
        fn next(v: &Kitamasa, coefficient: &Vec<usize>) -> Vec<usize> {
            let mut next_coefficient: Vec<usize> = vec![0; v.first_term.len()];

            next_coefficient[0] = v.initial_coefficient[0] * coefficient[coefficient.len()-1];
            next_coefficient[0] %= v.mod_number;
            for i in 1..v.first_term.len() {
                next_coefficient[i] = coefficient[i-1] + v.initial_coefficient[i] * coefficient[coefficient.len()-1];
                next_coefficient[i] %= v.mod_number;
            }

            return next_coefficient;
        }

        fn double(v: &Kitamasa, coefficient: Vec<usize>) -> Vec<usize> {
            let mut double_coefficient: Vec<usize> = vec![0; v.first_term.len()];
            let mut cs: Vec<Vec<usize>> = Vec::new();
            cs.push(coefficient.clone());

            for _ in 0..v.first_term.len()-1 {
                cs.push(Kitamasa::next(v, &cs[cs.len()-1]));
            }

            for i in 0..cs.len() {
                for j in 0..v.first_term.len() {
                    double_coefficient[j] += cs[i][j] * coefficient[i];
                    double_coefficient[j] %= v.mod_number;
                }
            }

            return double_coefficient;
        }

        fn calculate(v: &Kitamasa, n: usize) -> usize {
            let mut coefficient: Vec<usize> = vec![0; v.first_term.len()];
            coefficient[1] = 1;
            let mut msb = 0;

            for i in (0..64).rev() {
                if (n & (1 << i)) > 0 {
                    msb = i;
                    break;
                }
            }

            for i in (0..msb).rev() {
                coefficient = Kitamasa::double(v, coefficient);
                if (n & (1 << i)) > 0 {
                    coefficient = Kitamasa::next(v, &coefficient);
                }
            }

            let mut ans = 0;
            for i in 0..v.first_term.len() {
                ans += coefficient[i] * v.first_term[i];
                ans %= v.mod_number;
            }

            return ans;
        }
    }

    input! {
        k: usize,
        n: usize,
    }

    let a: Vec<usize> = vec![1; k];
    let v = Kitamasa {
        first_term: a.clone(),
        initial_coefficient: a.clone(),
        mod_number: 1000000007, 
    };

    println!("{}", Kitamasa::calculate(&v, n-1));
}
