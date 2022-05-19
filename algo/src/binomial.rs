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

pub fn catalan_dp(n: usize) -> i32 {
    let mut catalans = vec![0; n + 1];
    catalans[0] = 1;
    catalans[1] = 1;
    for i in 2..n + 1 {
        for j in 0..i {
            catalans[i] = catalans[i] + catalans[i - j - 1] * catalans[j]
        }
    }

    catalans[n]
}
