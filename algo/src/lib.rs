pub mod binomial;
#[cfg(test)]
mod tests {
    use crate::binomial;

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
}
