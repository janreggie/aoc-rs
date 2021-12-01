use std::any;
use std::fmt;
use std::str::FromStr;

pub fn from_strs<T>(strs: &Vec<String>) -> Result<Vec<T>, String>
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
