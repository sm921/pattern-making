pub fn binomial(n: usize, k: usize) -> u32 {
    let mut pascal_triangle: Vec<Vec<u32>> = vec![vec![1; n + 1]; n + 1];
    for i in 0..n + 1 {
        for j in 0..i + 1 {
            pascal_triangle[i][j] = if j == 0 || j == i {
                1
            } else {
                pascal_triangle[i - 1][j - 1] + pascal_triangle[i - 1][j]
            };
        }
    }
    pascal_triangle[n][k]
}

#[cfg(test)]
mod test {
    use crate::binomial::binomial;
    #[test]
    fn test_binomial() {
        assert_eq!(binomial(3, 1), 3);
        assert_eq!(binomial(1, 1), 1);
        assert_eq!(binomial(3, 2), 3);
        assert_eq!(binomial(5, 2), 10);
        assert_eq!(binomial(8, 4), 70);
        assert_eq!(binomial(4, 4), 1);
        assert_eq!(binomial(7, 2), 21);
    }
}
