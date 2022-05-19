pub mod binomial;
pub mod lis;
#[cfg(test)]
mod tests {
    use std::vec;

    use crate::binomial;
    use crate::lis;
    #[test]
    fn check_binomial_works() {
        assert_eq!(binomial::binomial_coefficiant(8, 2), 28);
    }

    #[test]
    fn check_catalan_works() {
        assert_eq!(binomial::catalan(1), 1);

        assert_eq!(binomial::catalan(2), 2);

        assert_eq!(binomial::catalan(3), 5);
    }

    #[test]
    fn check_catalan_dp_works() {
        println!("{}", binomial::catalan_dp(2));
        assert_eq!(binomial::catalan_dp(1), 1);

        assert_eq!(binomial::catalan_dp(2), 2);

        assert_eq!(binomial::catalan_dp(3), 5);
    }

    #[test]
    fn check_lis() {
        assert_eq!(binomial::catalan(1), 1);

        let c = vec![3, 10, 2, 1, 20];
        let res = lis::lis(&c);

        assert_eq!(res, [3, 10, 20]);
    }
}
