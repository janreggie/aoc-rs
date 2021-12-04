use crate::util::vectors;

pub fn d04(lines: Vec<String>) -> Result<(String, String), String> {
    let groups = vectors::group(lines);
    if groups.len() < 2 {
        return Err(String::from(
            "input must have at least two groups of continguous lines",
        ));
    }

    Err(String::from("unimplemented"))
}
