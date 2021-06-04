/// from (inclusive), to (exclusive)
pub fn sigma<T>(a: T, from: usize, to: usize) -> f64
where
    T: Fn(usize) -> f64,
{
    let mut sum = 0.0;
    for k in from..to {
        sum = sum + a(k);
    }
    sum
}

#[cfg(test)]
mod test_sigma {
    use crate::sigma::sigma;
    #[test]
    fn test_sigma() {
        assert_eq!(sigma(|k| k as f64, 1, 10), 45.0);
        assert_eq!(sigma(|k| k as f64, 3, 5), 7.0);
        assert_eq!(sigma(|k| 2.0 * k as f64, 1, 4), 12.0);
    }
}

/// from (inclusive), to (exclusive)
pub fn sigma_vec<T>(a: T, from: usize, to: usize) -> (f64, f64)
where
    T: Fn(usize) -> (f64, f64),
{
    let mut sum = (0.0, 0.0);
    for k in from..to {
        let ak = a(k);
        sum = (sum.0 + ak.0, sum.1 + ak.1);
    }
    sum
}
