// modを考慮した指数計算a^b
fn pow_mod(a: usize, b: usize, mod_number: usize) -> usize {
    let mut p = 1;
    let mut q = a;

    for i in 0..63 {
        if (b & (1 << i)) != 0 {
            p *= q;
            p %= mod_number;
        }
        q *= q;
        q %= mod_number;
    }

    return p;
}
