use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::{digit1, line_ending, multispace1, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use std::{fs, ops::Range};

fn main() {
    let input = read_file("input/data.txt").unwrap();
    let (_, (seeds, almanac)) = parse_input(&input).unwrap();

    // process seeds into a single range
    let seed_ranges = encompassing_range(seed_ranges(seeds));

    // get all location values to search
    let location_ranges = almanac.last()
        .unwrap()
        .value
        .iter()
        .map(|x| {
            x.destination_range.clone()
        })
        .collect::<Vec<Range<u64>>>();

    let location_ranges = encompassing_range(location_ranges);

    // backwards traversal until we hit
    almanac.clone().reverse();
    for i in location_ranges {
        let mut check_value = i.clone();

        for map in &almanac {
            check_value = map.reverse(check_value);
        }
        
        if seed_ranges.contains(&check_value) {
            println!("{}", i);
            break
        }
    }



}

fn seed_ranges(v: Vec<u64>) -> Vec<Range<u64>> {
    v.chunks_exact(2)
        .into_iter()
        .map(|x| x[0]..(x[0] + x[1]))
        .collect::<Vec<Range<u64>>>()
}

fn encompassing_range(v: Vec<Range<u64>>) -> Range<u64> {
    let min_start = v.iter().map(|range| range.start).min().unwrap();
    let max_end = v.iter().map(|range| range.end).max().unwrap();
    min_start..max_end
}

#[derive(Debug, Clone)]
struct ListMapping {
    value: Vec<Mapping>,
}

impl ListMapping {
    fn from(v: Vec<Vec<u64>>) -> ListMapping {
        let value = v
            .into_iter()
            .map(|v| Mapping::from(v))
            .collect::<Vec<Mapping>>();

        ListMapping { value }
    }

    fn map(&self, seed: u64) -> u64 {
        // set default to original seed value
        let mut mapped_value = seed.clone();

        for mapping in &self.value {
            if mapping.source_range.contains(&seed) {
                mapped_value =
                    mapping.destination_range.start + (&seed - mapping.source_range.start)
            }
        }
        mapped_value
    }

    fn reverse(&self, destination: u64) -> u64 {
        // set default to original destination value
        let mut reversed_value = destination;
    
        for mapping in &self.value {
            if mapping.destination_range.contains(&destination) {
                reversed_value =
                    mapping.source_range.start + (&destination - mapping.destination_range.start)
            }
        }
        reversed_value
    }

}

#[derive(Debug, Clone)]
struct Mapping {
    source_range: Range<u64>,
    destination_range: Range<u64>,
}

impl Mapping {
    fn from(v: Vec<u64>) -> Mapping {
        if v.len() == 3 {
            Mapping {
                source_range: v[1]..(v[1] + v[2]),
                destination_range: v[0]..(v[0] + v[2]),
            }
        } else {
            unimplemented!();
        }
    }
}

fn read_file(filepath: &str) -> std::io::Result<String> {
    fs::read_to_string(filepath)
}

fn parse_seeds(s: &str) -> IResult<&str, Vec<u64>> {
    let (input, _) = tag("seeds: ")(s)?;
    let (input, v) = separated_list1(space1, digit1)(input)?;
    let (input, _) = line_ending(input)?;

    let v: Vec<u64> = v
        .into_iter()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    Ok((input, v))
}

fn parse_map(s: &str) -> IResult<&str, Vec<Vec<u64>>> {
    let (input, _) = terminated(take_until("\n"), line_ending)(s)?;

    let parse_u64 = |input: &str| input.parse::<u64>();
    let parse_line = separated_list1(
        space1,
        map_res(take_while(|c: char| c.is_digit(10)), parse_u64),
    );

    separated_list1(terminated(take_until("\n"), line_ending), parse_line)(input)
}

fn parse_input(s: &str) -> IResult<&str, (Vec<u64>, Vec<ListMapping>)> {
    let (input, seeds) = parse_seeds(s)?;
    let (input, _) = terminated(take_until("\n"), line_ending)(input)?;

    let (input, v) = separated_list1(multispace1, parse_map)(input)?;

    let mut result_vec: Vec<ListMapping> = Vec::new();
    for entry in v {
        result_vec.push(ListMapping::from(entry));
    }

    Ok((input, (seeds, result_vec)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seeds_line() {
        let input = "seeds: 79 14 55 13\n";
        let expected_output = vec![79, 14, 55, 13];

        let result = parse_seeds(input).unwrap().1;

        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_parse_map() {
        let input = "seed-to-soil map:\n50 98 2\n52 50 48\n";
        let expected_output = vec![vec![50, 98, 2], vec![52, 50, 48]];

        let result = parse_map(input).unwrap().1;

        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_mapping_from() {
        let input = vec![50, 98, 2];
        let expected_output = Mapping {
            source_range: 98..100,
            destination_range: 50..52,
        };

        let result = Mapping::from(input);

        assert_eq!(result.source_range, expected_output.source_range);
        assert_eq!(result.destination_range, expected_output.destination_range);
    }

    #[test]
    fn test_list_mapping_map() {
        let list_mapping = ListMapping {
            value: vec![
                Mapping {
                    source_range: Range { start: 10, end: 20 },
                    destination_range: Range { start: 30, end: 40 },
                },
                Mapping {
                    source_range: Range { start: 20, end: 30 },
                    destination_range: Range { start: 40, end: 50 },
                },
            ],
        };

        let seed = 15;
        let expected_output = 35; // 15 is in the first mapping's source range, so it gets mapped to 35 in the destination range

        let result = list_mapping.map(seed);

        assert_eq!(result, expected_output);
    }
}
