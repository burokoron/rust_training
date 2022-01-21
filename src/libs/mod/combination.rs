struct Combination {
    max_number: usize, // 組み合わせnCrのNの最大値
    mod_number: i64, // modの数値
    factorial: Vec<i64>,
    factorial_inverse: Vec<i64>,
    inverse: Vec<i64>,
}

impl Combination {
    // 初期化関数(事前計算)
    fn initialize(v: &mut Combination) {
        v.factorial.push(1);
        v.factorial.push(1);
        v.factorial_inverse.push(1);
        v.factorial_inverse.push(1);
        v.inverse.push(-1); // 使用しない
        v.inverse.push(1);

        for i in 2..v.max_number {
            v.factorial.push(v.factorial[i-1] * i as i64 % v.mod_number);
            v.inverse.push(v.mod_number - v.inverse[v.mod_number as usize % i] * (v.mod_number / i as i64) % v.mod_number);
            v.factorial_inverse.push(v.factorial_inverse[i-1] * v.inverse[i] % v.mod_number);
        }
    }

    // 組み合わせ計算
    fn combination(&self, n: usize, k: usize) -> i64 {
        if n < k {
            return 0;
        }
        return self.factorial[n] * (self.factorial_inverse[k] * self.factorial_inverse[n-k] % self.mod_number) % self.mod_number;
    }
}

fn main() {
    let v = &mut Combination {
        max_number: 510000,
        mod_number: 1000000007,
        factorial: Vec::new(),
        factorial_inverse: Vec::new(),
        inverse: Vec::new(),
    };

    Combination::initialize(v);

    println!("{}", v.combination(2784, 1247));
}
