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
    
    let seed_ranges = simplify_overlaps(seed_ranges(seeds));

    dbg!(seed_ranges.clone());

    let mut min_location: u64 = u64::MAX;
    let mut prev_min_location = u64::MAX;

    for seed in seed_ranges.iter().flat_map(|range| range.start..range.end).collect::<Vec<u64>>() {
        let mut traverser = seed;

        for map in &almanac {
            traverser = map.map(traverser);
        }

        min_location = min_location.min(traverser);
        
        if min_location < prev_min_location {
            dbg!(min_location.clone());
            prev_min_location = min_location;
        }

    }

    // // process this one range at a time
    // for range in seed_ranges {
    //     let mut traverser: Vec<Range<u64>> = vec![range.clone()];

    //     for list_mapping in &almanac {
    //         let mut output: Vec<Range<u64>> = Vec::new(); 
            
    //         for range in traverser.iter() {
    //             output.extend(list_mapping.map_range(&range));
    //         }
    
    //         // update the whole range after going through a map
    //         traverser = output;

    //         dbg!(traverser.clone());
    //     }

    //     dbg!(min_start(traverser.clone()));

    //     min_location = min_location.min(min_start(traverser));

    //     println!("{}", format!("{:?} done", range));
    // } 

    println!("{}", min_location);
}

fn seed_ranges(v: Vec<u64>) -> Vec<Range<u64>> {
    v.chunks_exact(2)
        .into_iter()
        .map(|x| x[0]..(x[0] + x[1]))
        .collect::<Vec<Range<u64>>>()
}

fn min_start(v: Vec<Range<u64>>) -> u64 {
    v.iter().map(|range| range.start).min().unwrap()
}

fn simplify_overlaps(mut v: Vec<Range<u64>>) -> Vec<Range<u64>> {
    v.sort_by(|a, b| a.start.cmp(&b.start));

    let mut result = Vec::new();

    let mut current_range = v[0].clone();

    for range in v.into_iter().skip(1) {
        if range.start <= current_range.end {
            // Ranges overlap, so merge them
            current_range.end = current_range.end.max(range.end);
        } else {
            // Ranges don't overlap, so add the current range to the result and start a new one
            result.push(current_range);
            current_range = range;
        }
    }

    // Add the last range to the result
    result.push(current_range);

    result
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
    fn map_range(&self, r: &Range<u64>) -> Vec<Range<u64>> {
        let mut output: Vec<Range<u64>> = Vec::new();

        for map in &self.value {
            output.extend(map.map_range(r));
        }

        output.sort_by(|a, b| a.start.cmp(&b.start));
        output.dedup();
        simplify_overlaps(output)
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

    fn map(&self, seed: u64) -> u64 {
        if self.source_range.contains(&seed) {
            self.destination_range.start + (seed - self.source_range.start)
        } else {
            seed
        }
    }

    fn map_range(&self, r: &Range<u64>) -> Vec<Range<u64>> {
        let (i_start, i_end, j_start, j_end) = (
            r.start,
            r.end,
            self.source_range.start,
            self.source_range.end,
        );
    
        if self.source_range.contains(&i_start) && self.source_range.contains(&i_end) {
            vec![self.map(i_start)..self.map(i_end)]
        } else if i_end <= j_start || i_start >= j_end {
            vec![r.clone()]
        } else {
            let overlap_start = i_start.max(j_start);
            let overlap_end = i_end.min(j_end);
            let mut result = Vec::new();
    
            if i_start < overlap_start {
                result.push(i_start..overlap_start);
            }
    
            result.push(self.map(overlap_start)..self.map(overlap_end));
    
            if i_end > overlap_end {
                result.push(overlap_end..i_end);
            }
    
            result.sort_by(|a, b| a.start.cmp(&b.start));
            result.dedup();
            simplify_overlaps(result)
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
