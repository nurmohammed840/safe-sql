use std::ops;

use crate::*;

#[cfg(test)]
pub mod test {
    #[macro_export]
    macro_rules! syntex {
        ($($t:tt)*) => { syn::parse2(<proc_macro2::TokenStream as std::str::FromStr>::from_str(stringify!($($t)*)).unwrap()) };
    }

    pub(crate) use syntex;
}

#[derive(Debug)]
pub struct SeparatedByComma<T> {
    pub values: Vec<T>,
}

impl<T: Parse> Parse for SeparatedByComma<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut values = vec![];
        while !input.cursor().eof() {
            values.push(input.parse()?);
            if !input.peek(Token![,]) {
                break;
            }
            input.parse::<Token![,]>()?;
        }
        Ok(SeparatedByComma { values })
    }
}

impl<T: ops::Deref> ops::Deref for SeparatedByComma<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

pub fn suggest<I>(input: &str, values: I) -> String
where
    I: Iterator,
    I::Item: fmt::Display,
{
    let mut tree: Vec<_> = values
        .map(|v| {
            let val = v.to_string();
            (levenshtein_distance(input, &val), val)
        })
        .collect();

    tree.sort();
    let mut values = tree.iter().map(|(_, b)| b).take(7);
    let mut msg = String::new();

    if let Some(v) = values.next() {
        msg += "`";
        msg += v;
        msg += "`";
    }
    for v in values {
        msg += ", `";
        msg += v;
        msg += "`";
    }
    msg
}

pub fn parse_kw_if_matched(input: ParseStream, kw: &str) -> Result<Ident> {
    let err = || input.error(format!("expected keyword: `{kw}`"));
    input.step(|c| {
        let (keyword, rest) = c.ident().ok_or_else(err)?;
        if !keyword.to_string().eq_ignore_ascii_case(kw) {
            return Err(err());
        }
        Ok((keyword, rest))
    })
}
pub fn parse_keywords_if_matched(input: ParseStream, kws: &[&str]) -> Result<Ident> {
    let err = || input.error(format!("expected keywords: `{}`", kws.join(", ")));
    input.step(|c| {
        let (keyword, rest) = c.ident().ok_or_else(err)?;
        let kw = keyword.to_string();
        for expected_kw in kws {
            if !kw.eq_ignore_ascii_case(expected_kw) {
                return Err(err());
            }
        }
        Ok((keyword, rest))
    })
}

/// See:
/// - https://youtu.be/d-Eq6x1yssU?si=YW94NY_vSc_KXrSb
/// - https://en.wikipedia.org/wiki/Levenshtein_distance
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
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
