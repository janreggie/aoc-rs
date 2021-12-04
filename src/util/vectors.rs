use std::any;
use std::fmt;
use std::str::FromStr;

pub fn from_strs<T>(strs: Vec<String>) -> Result<Vec<T>, String>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    let mut result = Vec::new();
    for str in strs {
        let item: Result<T, _> = str.parse();
        if let Err(_) = item {
            return Err(format!(
                "could not format `{}` as type `{}`",
                str,
                any::type_name::<T>()
            ));
        }
        result.push(item.unwrap());
    }
    Ok(result)
}

/// Groups together strings which aren't empty.
/// For example:
///
///     group(["a", "b", "", "c", "d"]) -> [["a","b"], ["c","d"]]
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
