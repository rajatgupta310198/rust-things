pub fn binomial_coefficiant(n: u64, mut k: u64) -> u64 {
    if k > n - k {
        k = n - k;
    }

    let mut res = 1;

    for i in 0..k {
        res = res * (n - i);
        res = res / (i + 1);
    }

    res
}

pub fn catalan(n: u64) -> u64 {
    let res = binomial_coefficiant(2 * n, n);

    res / (n + 1)
}
