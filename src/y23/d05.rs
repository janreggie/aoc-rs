use std::{collections::VecDeque, vec};

use anyhow::{bail, Context, Result};

use crate::util::vectors::group;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    start: u64,
    length: u64,
}

/// Combine range_1 and range_2 if they intersect.
/// None if they don't.
fn combine(range_1: Range, range_2: Range) -> Option<Range> {
    // Make sure range_1 is at least before range_2
    if range_1.start > range_2.start {
        return combine(range_2, range_1);
    }

    // Check for disjointness
    if range_1.start + range_1.length <= range_2.start {
        return None;
    }

    // Finally, check if range_2 is completely inside range_1
    if range_2.start + range_2.length <= range_1.start + range_1.length {
        Some(range_1)
    } else {
        Some(Range {
            start: range_1.start,
            length: range_2.start - range_1.start + range_2.length,
        })
    }
}

#[test]
fn test_combine() {
    assert_eq!(
        combine(Range { start: 1, length: 2 }, Range { start: 3, length: 8 }),
        None,
    );
    assert_eq!(
        combine(Range { start: 1, length: 3 }, Range { start: 3, length: 8 }),
        Some(Range { start: 1, length: 10 })
    );
    assert_eq!(
        combine(Range { start: 1, length: 5 }, Range { start: 3, length: 8 }),
        Some(Range { start: 1, length: 10 })
    );
    assert_eq!(
        combine(Range { start: 1, length: 16 }, Range { start: 3, length: 8 }),
        Some(Range { start: 1, length: 16 })
    );
    assert_eq!(
        combine(Range { start: 7, length: 3 }, Range { start: 8, length: 3 }),
        Some(Range { start: 7, length: 4 })
    );
}

fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
    if ranges.len() <= 1 {
        return ranges;
    }

    let mut ranges = ranges;
    ranges.sort_by_key(|range| range.start);
    let mut ranges = VecDeque::from(ranges);
    let mut result = vec![];
    let mut current = ranges.pop_front().unwrap();

    while ranges.len() > 0 {
        let front = ranges.pop_front().unwrap();
        if let Some(combined) = combine(current, front) {
            current = combined;
        } else {
            result.push(current);
            current = front;
        }
    }
    result.push(current);
    result
}

#[test]
fn test_merge_ranges() {
    let ranges = vec![
        Range { start: 1, length: 3 },
        Range { start: 2, length: 4 },
        Range { start: 7, length: 3 },
        Range { start: 8, length: 3 },
    ];
    assert_eq!(
        merge_ranges(ranges),
        vec![Range { start: 1, length: 5 }, Range { start: 7, length: 4 }]
    );
}

#[derive(Debug, PartialEq, Eq)]
struct MapRow {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

/// MapRowMapping represents a Range being either successfully mapped via MapRow, or unmapped.
#[derive(Debug, PartialEq, Eq)]
enum MapRowMapping {
    Mapped(Range),
    Unmapped(Range),
}

impl MapRow {
    /// Takes in an input e.g., "45 77 23"
    fn new(input: &str) -> Result<MapRow> {
        let numbers = input
            .split(" ")
            .map(|v| v.parse::<u64>().ok())
            .collect::<Option<Vec<u64>>>()
            .context("could not parse numbers to u64")?;
        if numbers.len() != 3 {
            bail!(
                "there should be three numbers, got {} instead",
                numbers.len()
            )
        }
        Ok(MapRow {
            destination_range_start: numbers[0],
            source_range_start: numbers[1],
            range_length: numbers[2],
        })
    }

    /// Returns the destination given a source. Returns None if outside of range
    fn source_to_dest(&self, source: u64) -> Option<u64> {
        if source >= self.source_range_start
            && source < self.source_range_start + self.range_length
        {
            Some(
                self.destination_range_start + source - self.source_range_start,
            )
        } else {
            None
        }
    }

    /// Returns a series of MapRowMappings depending on the range.
    /// Can return at most three.
    fn feed_range(&self, range: Range) -> Vec<MapRowMapping> {
        if range.start < self.source_range_start {
            // The start of range will be unmapped
            let unmapped_length = self.source_range_start - range.start;
            if unmapped_length >= range.length {
                return vec![MapRowMapping::Unmapped(range)];
            }

            // `starting` is unmapped. There will be mapped numbers.
            let starting = MapRowMapping::Unmapped(Range {
                start: range.start,
                length: unmapped_length,
            });
            let remaining_start = self.source_range_start;
            let remaining_length = range.length - unmapped_length;
            if remaining_length <= self.range_length {
                return vec![
                    starting,
                    MapRowMapping::Mapped(Range {
                        start: self.source_to_dest(remaining_start).unwrap(),
                        length: remaining_length,
                    }),
                ];
            }

            // `middle` is mapped. There will still be unmapped numbers
            let middle = MapRowMapping::Mapped(Range {
                start: self.source_to_dest(remaining_start).unwrap(),
                length: self.range_length,
            });
            let ending_length = remaining_length - self.range_length;
            let ending_start = remaining_start + self.range_length;
            return vec![
                starting,
                middle,
                MapRowMapping::Unmapped(Range {
                    start: ending_start,
                    length: ending_length,
                }),
            ];
        } else if range.start >= self.source_range_start + self.range_length {
            // There won't be any mapping at all
            return vec![MapRowMapping::Unmapped(range)];
        } else {
            // The start of range will be mapped
            let unmapped_length = range.start - self.source_range_start; // to the left of range
            let self_remaining_length = self.range_length - unmapped_length;
            if self_remaining_length >= range.length {
                // All of range will be mapped
                return vec![MapRowMapping::Mapped(Range {
                    start: self.source_to_dest(range.start).unwrap(),
                    length: range.length,
                })];
            }

            //
            let mapped = MapRowMapping::Mapped(Range {
                start: self.source_to_dest(range.start).unwrap(),
                length: self_remaining_length,
            });
            let unmapped_length = range.length - self_remaining_length;
            let unmapped_start = range.start + self_remaining_length;
            return vec![
                mapped,
                MapRowMapping::Unmapped(Range {
                    start: unmapped_start,
                    length: unmapped_length,
                }),
            ];
        }
    }
}

#[test]
fn test_map_row() {
    let map_row = MapRow::new("50 98 2").unwrap();
    assert_eq!(
        map_row,
        MapRow {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2
        }
    );
    assert_eq!(map_row.source_to_dest(97), None);
    assert_eq!(map_row.source_to_dest(98), Some(50));
    assert_eq!(map_row.source_to_dest(99), Some(51));
    assert_eq!(map_row.source_to_dest(100), None);

    // source_range is from 97 to 100
    let source_range = Range { start: 97, length: 4 };
    assert_eq!(
        map_row.feed_range(source_range),
        vec![
            MapRowMapping::Unmapped(Range { start: 97, length: 1 }),
            MapRowMapping::Mapped(Range { start: 50, length: 2 }),
            MapRowMapping::Unmapped(Range { start: 100, length: 1 }),
        ]
    );
    assert_eq!(
        map_row.feed_range(Range { start: 100, length: 2 }),
        vec![MapRowMapping::Unmapped(Range { start: 100, length: 2 })]
    )
}

struct Map {
    rows: Vec<MapRow>,
}

impl Map {
    fn new(lines: &Vec<String>) -> Result<Map> {
        let rows = lines
            .into_iter()
            .map(|line| {
                MapRow::new(line).with_context(|| {
                    format!("could not parse line `{}` properly", line)
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Map { rows })
    }

    /// Returns the destination given a source. Returns None if it can't be found in the ranges.
    fn source_to_dest(&self, source: u64) -> Option<u64> {
        self.rows.iter().find_map(|row| row.source_to_dest(source))
    }

    /// Returns the destination given a source. Returns the same number if it can't be found in the ranges.
    fn source_to_dest_or_same(&self, source: u64) -> u64 {
        self.source_to_dest(source).unwrap_or(source)
    }

    /// Feed it with a single range naively. Returns a list of ranges.
    fn feed_range(&self, range: Range) -> Vec<Range> {
        let mut unmapped_ranges = vec![range];
        let mut mapped_ranges = vec![];
        for row in &self.rows {
            let unmapped_ranges_now = unmapped_ranges;
            unmapped_ranges = vec![];
            for range in unmapped_ranges_now {
                for result in row.feed_range(range) {
                    match result {
                        MapRowMapping::Mapped(r) => mapped_ranges.push(r),
                        MapRowMapping::Unmapped(r) => unmapped_ranges.push(r),
                    }
                }
            }
        }

        unmapped_ranges.append(&mut mapped_ranges);
        merge_ranges(unmapped_ranges)
    }

    /// Feed it with a bunch of ranges. Returns a list of ranges.
    fn feed_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        ranges
            .into_iter()
            .map(|range| self.feed_range(range))
            .flatten()
            .collect()
    }
}

#[test]
fn test_map() {
    let map =
        Map::new(&vec!["50 98 2".to_string(), "52 50 48".to_string()]).unwrap();
    let expects = [
        (0, 0),
        (1, 1),
        (48, 48),
        (49, 49),
        (50, 52),
        (51, 53),
        (96, 98),
        (97, 99),
        (98, 50),
        (99, 51),
        (100, 100),
    ];
    for (source, dest) in expects {
        assert_eq!(map.source_to_dest_or_same(source), dest)
    }
}

struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Almanac {
    fn new(lines: Vec<String>) -> Result<Almanac> {
        let groups = group(lines);
        if groups.len() != 8 {
            bail!("input does not have 8 groups, got {}", groups.len());
        }

        let seeds = &groups[0];
        if seeds.len() != 1 {
            bail!("seed group should be of length 1")
        }
        let seeds = seeds[0]
            .strip_prefix("seeds: ")
            .context("could not find prefix 'seeds'")?
            .split(' ')
            .map(|seed| seed.parse::<u64>().ok())
            .collect::<Option<Vec<u64>>>()
            .context("could not parse seeds as u64")?;

        let groups = groups
            .into_iter()
            .skip(1)
            .map(|group| {
                group
                    .into_iter()
                    .skip(1) // Skip the row that says "seed-to-soil map:"
                    .collect::<Vec<String>>()
            })
            .map(|group| Map::new(&group))
            .collect::<Result<Vec<_>>>()
            .context("could not parse groups properly")?;
        let mut groups = groups.into_iter();

        Ok(Almanac {
            seeds,
            seed_to_soil: groups.next().unwrap(),
            soil_to_fertilizer: groups.next().unwrap(),
            fertilizer_to_water: groups.next().unwrap(),
            water_to_light: groups.next().unwrap(),
            light_to_temperature: groups.next().unwrap(),
            temperature_to_humidity: groups.next().unwrap(),
            humidity_to_location: groups.next().unwrap(),
        })
    }

    /// Using the seeds, return all locations
    fn get_locations(&self) -> Vec<u64> {
        self.seeds
            .iter()
            .map(|seed| self.seed_to_soil.source_to_dest_or_same(*seed))
            .map(|soil| self.soil_to_fertilizer.source_to_dest_or_same(soil))
            .map(|fertilizer| {
                self.fertilizer_to_water.source_to_dest_or_same(fertilizer)
            })
            .map(|water| self.water_to_light.source_to_dest_or_same(water))
            .map(|light| {
                self.light_to_temperature.source_to_dest_or_same(light)
            })
            .map(|temperature| {
                self.temperature_to_humidity.source_to_dest_or_same(temperature)
            })
            .map(|humidity| {
                self.humidity_to_location.source_to_dest_or_same(humidity)
            })
            .collect()
    }

    /// Using a list of ranges, return all ranges, or an empty list if seeds.len() isn't even
    fn get_locations_ranges(&self) -> Vec<Range> {
        let initial_ranges = self
            .seeds
            .chunks(2)
            .map(|pair| {
                if pair.len() != 2 {
                    None
                } else {
                    Some(Range { start: pair[0], length: pair[1] })
                }
            })
            .collect::<Option<Vec<_>>>();
        if initial_ranges.is_none() {
            return vec![];
        }
        let seed_ranges = initial_ranges.unwrap();
        let soil_ranges = self.seed_to_soil.feed_ranges(seed_ranges);
        let fertilizer_ranges =
            self.soil_to_fertilizer.feed_ranges(soil_ranges);
        let water_ranges =
            self.fertilizer_to_water.feed_ranges(fertilizer_ranges);
        let light_ranges = self.water_to_light.feed_ranges(water_ranges);
        let temperature_ranges =
            self.light_to_temperature.feed_ranges(light_ranges);
        let humidity_ranges =
            self.temperature_to_humidity.feed_ranges(temperature_ranges);
        self.humidity_to_location.feed_ranges(humidity_ranges)
    }
}

fn solve_part_1(almanac: &Almanac) -> Result<String> {
    Ok(almanac
        .get_locations()
        .iter()
        .min()
        .context("locations is empty")?
        .to_string())
}

fn solve_part_2(almanac: &Almanac) -> Result<String> {
    Ok(almanac
        .get_locations_ranges()
        .iter()
        .min_by_key(|range| range.start)
        .and_then(|range| Some(range.start.to_string()))
        .context("not even number of seeds")?)
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let almanac = Almanac::new(lines).context("could not create almanac")?;
    Ok((solve_part_1(&almanac), solve_part_2(&almanac)))
}
