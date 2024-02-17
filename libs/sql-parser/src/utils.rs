#[cfg(test)]
pub mod test {
    #[macro_export]
    macro_rules! syntex {
        ($($t:tt)*) => { syn::parse2(<proc_macro2::TokenStream as std::str::FromStr>::from_str(stringify!($($t)*)).unwrap()) };
    }

    pub(crate) use syntex;
}

/// See:
/// - https://youtu.be/d-Eq6x1yssU?si=YW94NY_vSc_KXrSb
/// - https://en.wikipedia.org/wiki/Levenshtein_distance
pub fn _levenshtein_distance(a: &str, b: &str) -> usize {
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..m + 1 {
        for j in 0..n + 1 {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1].min(dp[i - 1][j - 1]));
            }
        }
    }
    dp[m][n]
}

