#[derive(Debug)]
struct Range {
    src_start: i64,
    dst_start: i64,
    length: i64,
}

#[derive(Debug)]
struct Map {
    src: String,
    dst: String,
    ranges: Vec<Range>,
}

impl Range {
    fn contains(&self, src: i64) -> bool {
        src >= self.src_start && src < self.src_start + self.length
    }
}

impl Map {
    fn map(&self, src: i64) -> i64 {
        let mut dst = src;
        for range in &self.ranges {
            if range.contains(src) {
                dst = range.dst_start + (src - range.src_start);
                break;
            }
        }
        dst
    }
}

fn find_location(seed: i64, mapping: &Vec<Map>) -> i64 {
    let mut location = seed;
    for map in mapping {
        location = map.map(location);
    }

    return location;
}

pub fn solve_part_1(almanac: &str) -> i64 {
    let seeds = read_seeds(almanac);
    let mapping: Vec<Map> = build_mapping(almanac);
    seeds.iter().map(|&seed| find_location(seed, &mapping)).min().unwrap()
}

fn read_seeds(almanac: &str) -> Vec<i64> {
    almanac.lines().into_iter()
        .nth(0).unwrap()
        .strip_prefix("seeds: ").unwrap()
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn build_mapping(almanac: &str) -> Vec<Map> {
    let mut mapping: Vec<Map> = Vec::new();

    let mut current_map = &mut Map {
        src: "".to_string(),
        dst: "".to_string(),
        ranges: Vec::new(),
    };
    for line in almanac.lines().into_iter().skip(1) {
        if line.is_empty() {
            continue;
        }

        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() == 2 {
            let names = parts[0].split_once("-to-").unwrap();
            mapping.push(Map {
                src: names.0.to_string(),
                dst: names.1.to_string(),
                ranges: Vec::new(),
            });
            current_map = mapping.last_mut().unwrap()
        } else {
            // the almanac ranges are defined in the order dst -> src -> length
            let dst_start = parts[0].parse::<i64>().unwrap();
            let src_start = parts[1].parse::<i64>().unwrap();
            let length = parts[2].parse::<i64>().unwrap();

            current_map.ranges.push(Range {
                src_start,
                dst_start,
                length,
            });
        }
    }

    return mapping;
}

pub fn solve_part_2(almanac: &str) -> i64 {
    let seeds: Vec<i64> = read_seeds(almanac);
    let seeds: Vec<&[i64]> = seeds.chunks_exact(2).collect();

    let mapping: Vec<Map> = build_mapping(almanac);

    seeds.iter()
        .map(|&seeds|
            {
                let seed = seeds[0];
                let count = seeds[1];
                (0..count)
                    .map(|offset| seed + offset)
                    .map(|s| find_location(s, &mapping))
                    .collect::<Vec<i64>>()
            }
        )
        .flatten()
        .min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_in_ranges() {
        let map = Map {
            src: "seed".to_string(),
            dst: "soil".to_string(),
            ranges: vec![
                Range {
                    src_start: 98,
                    dst_start: 50,
                    length: 2,
                },
                Range {
                    src_start: 50,
                    dst_start: 52,
                    length: 48,
                },
            ],
        };

        assert_eq!(map.map(49), 49);
        assert_eq!(map.map(50), 52);
        assert_eq!(map.map(97), 99);
        assert_eq!(map.map(98), 50);
        assert_eq!(map.map(99), 51);

        assert_eq!(map.map(79), 81);
        assert_eq!(map.map(14), 14);
        assert_eq!(map.map(55), 57);
        assert_eq!(map.map(13), 13);
    }
}
