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

/// Get the index of the first odd one out.
/// The odd one out is the element in a vector which is "different" from the rest.
/// If elems is length 0 or 1, return None.
/// If elems if length 2, return Some(0) or None.
///
/// ```none
/// odd_one_out_index(vec![4, 7, 4, 4]);  // returns Some(1)
/// odd_one_out_index(vec![4, 4, 7, 8]);  // returns Some(2) since vec[3] comes after vec[2]
/// odd_one_out_index(vec![4, 4, 4, 4]);  // returns None
/// ```
///
pub fn odd_one_out_index<T>(elems: &Vec<T>) -> Option<usize>
where
    T: PartialEq,
{
    if elems.len() < 2 {
        return None;
    }
    if elems.len() == 2 {
        return if elems[0] != elems[1] { Some(0) } else { None };
    }

    if elems[0] != elems[1] {
        if elems[1] == elems[2] {
            return Some(0);
        }
        return Some(1);
    }

    let first = &elems[0];
    for ii in 1..elems.len() {
        if &elems[ii] != first {
            return Some(ii);
        }
    }

    None
}

pub fn odd_one_out<T>(elems: &Vec<T>) -> Option<&T>
where
    T: PartialEq,
{
    if let Some(ii) = odd_one_out_index(elems) {
        Some(&elems[ii])
    } else {
        None
    }
}

/// Splits str into chunks by delim.
/// Multiple delim's between values (e.g., spaces) will be removed.
///
/// ```none
/// split_and_trim(" a b  c  d   "); // returns vec!["a","b","c","d"]
/// ```
///
pub fn split_and_trim(str: &str, delim: char) -> Vec<String> {
    str.trim_matches(delim)
        .split(delim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// split_and_trim but doesn't allocate new strings.
pub fn split_and_trim_borrowed(str: &str, delim: char) -> Vec<&str> {
    str.trim_matches(delim)
        .split(delim)
        .filter(|s| !s.is_empty())
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
