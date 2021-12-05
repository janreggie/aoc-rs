use anyhow::{bail, Context, Result};

#[derive(Debug, Copy, Clone)]
struct Number(u32);

#[derive(Debug)]
struct Report {
    nums: Vec<Number>,

    /// Maximum number of bits in each of nums.
    bitsize: usize,
}

impl Number {
    fn new(input: &str) -> Option<Number> {
        let result = u32::from_str_radix(input, 2);
        match result {
            Err(_) => None,
            Ok(num) => Some(Number(num)),
        }
    }

    /// Checks if the number has the `pos`'th bit set to 1.
    /// Remember that the `0`th bit of this number is the least significant.
    fn is_set(self: &Number, pos: usize) -> bool {
        self.0 & (1 << pos) != 0
    }
}

impl Report {
    fn new(lines: &Vec<String>) -> Result<Report> {
        if lines.len() == 0 {
            bail!("empty lines");
        }
        let mut result = Report {
            nums: Vec::new(),
            bitsize: lines[0].len(),
        };
        for line in lines {
            if line.len() != result.bitsize {
                bail!(
                    "line `{}` expected to be of length {}, got {}",
                    line,
                    result.bitsize,
                    line.len()
                );
            }
            let number = Number::new(line);
            match number {
                Some(x) => result.nums.push(x),
                None => bail!("invalid line `{}`", line),
            }
        }

        Ok(result)
    }

    fn power_consumption(self: &Report) -> u32 {
        // counts[x] == how many times was the `x`th bit set to `1`.
        // Note that the `0`th bit is the "least significant" one.
        let mut counts: Vec<usize> = vec![0; self.bitsize];
        for num in &self.nums {
            for ii in 0..self.bitsize {
                if num.is_set(ii) {
                    counts[ii] += 1
                }
            }
        }

        // Finally, using counts, evaluate our gamma rate
        let mut gamma_rate = 0;
        for ii in 0..self.bitsize {
            if counts[ii] > (self.nums.len() / 2) {
                gamma_rate += 1 << ii;
            }
        }
        let epsilon_rate = ((1 << self.bitsize) - 1) - gamma_rate;
        gamma_rate * epsilon_rate
    }

    fn life_support_rating(self: &Report) -> u32 {
        Report::oxygen_generator_rating_iter(self, self.bitsize - 1)
            * Report::co2_scrubber_rating_iter(self, self.bitsize - 1)
    }

    // Iterates through self.nums such that all self.nums's bits more significant than `ind` is the same.
    fn oxygen_generator_rating_iter(old: &Report, ind: usize) -> u32 {
        if old.nums.len() == 1 {
            return old.nums[0].0;
        }

        let bit_set_as_one_count = old.nums.iter().filter(|num| num.is_set(ind)).count();
        let choose_one = bit_set_as_one_count > ((old.nums.len() - 1) / 2);

        let new_nums: Vec<Number> = old
            .nums
            .clone()
            .into_iter()
            .filter(|num| num.is_set(ind) == choose_one)
            .collect();

        if ind == 0 {
            new_nums[0].0
        } else {
            Report::oxygen_generator_rating_iter(
                &Report {
                    nums: new_nums,
                    bitsize: old.bitsize,
                },
                ind - 1,
            )
        }
    }

    // Iterates through self.nums such that all self.nums's bits more significant than `ind` is the same.
    fn co2_scrubber_rating_iter(old: &Report, ind: usize) -> u32 {
        if old.nums.len() == 1 {
            return old.nums[0].0;
        }

        let bit_set_as_zero_count = old.nums.iter().filter(|num| !num.is_set(ind)).count();
        let choose_zero = bit_set_as_zero_count <= (old.nums.len() / 2);

        let new_nums: Vec<Number> = old
            .nums
            .clone()
            .into_iter()
            .filter(|num| num.is_set(ind) != choose_zero)
            .collect();

        if ind == 0 {
            new_nums[0].0
        } else {
            Report::co2_scrubber_rating_iter(
                &Report {
                    nums: new_nums,
                    bitsize: old.bitsize,
                },
                ind - 1,
            )
        }
    }
}

pub fn d03(lines: Vec<String>) -> Result<(String, String)> {
    let report = Report::new(&lines).context("could not read input data")?;

    let ans1 = report.power_consumption();
    let ans2 = report.life_support_rating();

    Ok((ans1.to_string(), ans2.to_string()))
}
