use crate::util::vectors;

pub fn d01(lines: &Vec<String>) -> Result<(String, String), String> {
    let nums = vectors::from_strs::<u32>(lines);
    if let Err(e) = nums {
        return Err(format!("could not convert all input into nums {}", e));
    }
    let nums = nums.unwrap();

    // Part 1: Larger than previous
    let mut ans1 = 0;
    for ii in 1..nums.len() {
        if nums[ii] > nums[ii - 1] {
            ans1 += 1;
        }
    }

    // Part 2: Comparing nums[x] with nums[x+3]
    let mut ans2 = 0;
    for ii in 3..nums.len() {
        if nums[ii] > nums[ii - 3] {
            ans2 += 1;
        }
    }

    Ok((ans1.to_string(), ans2.to_string()))
}
