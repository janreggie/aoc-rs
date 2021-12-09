use anyhow::{bail, Result};
use std::any;
use std::fmt;
use std::str::FromStr;

pub fn from_strs<T>(strs: &Vec<String>) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let mut result = Vec::new();
    for str in strs {
        let item: Result<T, _> = str.parse();
        match item {
            Err(_) => bail!(
                "could not format `{}` as type `{}`",
                str,
                any::type_name::<T>()
            ),
            Ok(o) => result.push(o),
        }
    }
    Ok(result)
}

/// Splits str into chunks by delim.
/// Multiple delim's between values (e.g., spaces) will be removed.
///
/// ```none
/// split_and_trim(" a b  c  d   "); // returns vec!["a","b","c","d"]
/// ```
///
pub fn split_and_trim(str: &str, delim: char) -> Vec<String>
where
{
    str.trim_matches(delim)
        .split(delim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Groups together strings which aren't empty.
/// For example:
///
/// ```none
/// group(["a", "b", "", "c", "d"]); // returns vec![vec!["a","b"], vec!["c","d"]]
/// ```
///
pub fn group(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    result.push(Vec::new());

    for str in strs {
        if str == "" {
            result.push(Vec::new());
        } else {
            result.last_mut().unwrap().push(str);
        }
    }

    result
}
