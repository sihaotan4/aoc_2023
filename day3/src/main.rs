use itertools::iproduct;
use std::collections::HashSet;
use std::fs;

fn main() {
    let matrix = Matrix::build_from_file("data.txt");

    let result = get_all_gear_ratios(&matrix);

    println!("{}", result);
}

fn get_all_gear_ratios(matrix: &Matrix) -> u32 {
    let mut gear_ratios: Vec<u32> = Vec::new();

    for gear_coordinates in &matrix.symbol_coordinates {
        match check_gear_ratio(matrix, gear_coordinates) {
            Some(gear_ratio) => {
                gear_ratios.push(gear_ratio);
            },
            None => {},
        }
    }

    gear_ratios.iter().sum::<u32>()
}

fn check_gear_ratio(matrix: &Matrix, gear_coordinates: &(usize,usize)) -> Option<u32> {
    let mut adjacent_nums:Vec<u32> = Vec::new();

    for num in matrix.numbers.iter() {
        if num.perimeter.contains(&gear_coordinates) {
            adjacent_nums.push(num.number());
        }
    }

    // return Some(gear_ratio) if bordered by exactly two part numbers
    if adjacent_nums.len() != 2 {
        None
    } else {
        Some(adjacent_nums[0]*adjacent_nums[1])
    }
}

#[derive(Clone, Debug)]
struct Number {
    data: Vec<char>,
    coordinates: HashSet<(usize, usize)>,
    perimeter: HashSet<(usize, usize)>,
}

impl Number {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            coordinates: HashSet::new(),
            perimeter: HashSet::new(),
        }
    }

    fn reset(&mut self) {
        self.data.clear();
        self.coordinates.clear();
        self.perimeter.clear();
    }

    fn number(&self) -> u32 {
        let data_string: String = self.data.iter().collect();
        data_string.parse::<u32>().unwrap()
    }

    fn derive_perimeter(&mut self, i_last_index: usize, j_last_index: usize) {
        let mut perimeter: HashSet<(usize, usize)> = HashSet::new();

        // iterate through each digit, taking it's individual coordinates
        for (i, j) in &self.coordinates {
            let i_range = (i.saturating_sub(1))..=(usize::min(i + 1, i_last_index));
            let j_range = (j.saturating_sub(1))..=(usize::min(j + 1, j_last_index));

            // itertools macro to create an iterator cycling through all (i,j) tuples
            iproduct!(i_range, j_range).for_each(|x| {
                // don't insert if it's already a self coordinate
                if !self.coordinates.contains(&x) {
                    perimeter.insert(x);
                }
            })
        }
        self.perimeter = perimeter;
    }
}

#[derive(Clone, Debug)]
struct Matrix {
    data: Vec<Vec<char>>,
    numbers: Vec<Number>,
    symbol_coordinates: HashSet<(usize, usize)>,
}

impl Matrix {
    fn new() -> Matrix {
        Matrix {
            data: Vec::new(),
            numbers: Vec::new(),
            symbol_coordinates: HashSet::new(),
        }
    }

    // convenience constructor to fully populate each field from local file
    fn build_from_file(filepath: &str) -> Matrix {
        let mut matrix = Matrix::new();

        // populate data
        matrix.populate_data_from_file(filepath);

        // populate symbols
        matrix.populate_gear_coordinates();

        // populate numbers
        matrix.populate_numbers();

        matrix
    }

    fn populate_data_from_file(&mut self, filepath: &str) {
        let input_string = fs::read_to_string(filepath).expect("unable to read data from file");

        let data = input_string
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        self.data = data;
    }

    fn populate_gear_coordinates(&mut self) {
        let mut result: HashSet<(usize, usize)> = HashSet::new();

        for (i, line) in self.data.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if c == &'*' {
                    result.insert((i, j));
                }
            }
        }
        self.symbol_coordinates = result;
    }

    fn populate_numbers(&mut self) {
        let mut all_numbers: Vec<Number> = Vec::new();

        for (i, line) in self.data.iter().enumerate() {
            all_numbers.extend(Matrix::get_numbers_in_line(i, line).into_iter())
        }

        // matrix dimensions
        let j_last_index = &self.data.len() - 1;
        let i_last_index = &self.data[0].len() - 1;

        // populate perimeter data for each number
        all_numbers
            .iter_mut()
            .for_each(|x| x.derive_perimeter(i_last_index, j_last_index));

        self.numbers = all_numbers;
    }

    //Example input: "467..114.." and ".#.35..633"
    fn get_numbers_in_line(line_number: usize, line: &Vec<char>) -> Vec<Number> {
        let mut current_number: Number = Number::new();
        let mut all_numbers: Vec<Number> = Vec::new();
        let mut collecting: bool = false;

        for (j, &c) in line.iter().enumerate() {
            if c.is_numeric() {
                // start collecting
                if !collecting {
                    collecting = true;
                }
                // update current number data
                current_number.data.push(c);
                match current_number.coordinates.insert((line_number, j)) {
                    true => {}
                    false => {
                        panic!()
                    }
                }
            } else {
                if collecting {
                    // a number was being formed but is now completed
                    collecting = false;
                    all_numbers.push(current_number.clone());
                    current_number.reset();
                }
            }
        }

        //special case, last character in line is numeric
        if collecting {
            all_numbers.push(current_number.clone());
        }

        all_numbers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_data_from_file() {
        let expected_data = vec![
            vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
            vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
            vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'],
            vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
            vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'],
            vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
        ];

        let matrix = Matrix::build_from_file("test_data.txt");
        assert_eq!(matrix.data, expected_data);
    }

    #[test]
    fn test_matrix_get_numbers_in_line_type1() {
        let line_number = 0;
        let line = vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'];
        let numbers = Matrix::get_numbers_in_line(line_number, &line);

        assert_eq!(numbers.len(), 2);

        assert_eq!(numbers[0].data, vec!['4', '6', '7']);
        assert_eq!(
            numbers[0].coordinates,
            [(0, 0), (0, 1), (0, 2)]
                .iter()
                .cloned()
                .collect::<HashSet<_>>()
        );

        assert_eq!(numbers[1].data, vec!['1', '1', '4']);
        assert_eq!(
            numbers[1].coordinates,
            [(0, 5), (0, 6), (0, 7)]
                .iter()
                .cloned()
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_matrix_get_numbers_in_line_type2() {
        let line_number = 0;
        let line = vec!['.', '.', '.', '3', '5', '.', '.', '6', '3', '3'];
        let numbers = Matrix::get_numbers_in_line(line_number, &line);

        assert_eq!(numbers.len(), 2);

        assert_eq!(numbers[0].data, vec!['3', '5']);
        assert_eq!(
            numbers[0].coordinates,
            [(0, 3), (0, 4)].iter().cloned().collect::<HashSet<_>>()
        );

        assert_eq!(numbers[1].data, vec!['6', '3', '3']);
        assert_eq!(
            numbers[1].coordinates,
            [(0, 7), (0, 8), (0, 9)]
                .iter()
                .cloned()
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_matrix_populate_numbers() {
        let mut matrix = Matrix::new();
        matrix.populate_data_from_file("test_data.txt");

        matrix.populate_numbers();

        assert_eq!(matrix.numbers.len(), 10);

        assert_eq!(matrix.numbers[0].data, vec!['4', '6', '7']);
        assert_eq!(
            matrix.numbers[0].coordinates,
            [(0, 0), (0, 1), (0, 2)]
                .iter()
                .cloned()
                .collect::<HashSet<_>>()
        );

        assert_eq!(matrix.numbers[9].data, vec!['5', '9', '8']);
        assert_eq!(
            matrix.numbers[9].coordinates,
            [(9, 5), (9, 6), (9, 7)]
                .iter()
                .cloned()
                .collect::<HashSet<_>>()
        );
    }

    #[test]
    fn test_matrix_populate_gear_coordinates() {
        let mut matrix = Matrix::new();
        matrix.populate_data_from_file("test_data.txt");

        matrix.populate_gear_coordinates();

        assert_eq!(matrix.symbol_coordinates.len(), 3);

        let expected = [(1,3),(4,3),(8,5)]
            .iter()
            .cloned()
            .collect::<HashSet<_>>();
        assert_eq!(matrix.symbol_coordinates, expected);
    }

    #[test]
    fn test_number_data_to_number() {
        let number: Number = Number {
            data: vec!['4', '6', '7'],
            coordinates: HashSet::new(),
            perimeter: HashSet::new(),
        };
        assert_eq!(number.number(), 467);
    }

    #[test]
    fn derive_perimeter_limit_case() {
        let mut number = Number {
            data: Vec::new(),
            coordinates: vec![(1, 1), (1, 2)].into_iter().collect(),
            perimeter: HashSet::new(),
        };

        number.derive_perimeter(3, 2);

        let expected_perimeter: HashSet<_> =
            vec![(0, 0), (0, 1), (0, 2), (1, 0), (2, 0), (2, 1), (2, 2)]
                .into_iter()
                .collect();

        assert_eq!(number.perimeter, expected_perimeter);
    }

    #[test]
    fn derive_perimeter_edge_case() {
        let mut number = Number {
            data: Vec::new(),
            coordinates: vec![(1, 0), (2, 0)].into_iter().collect(),
            perimeter: HashSet::new(),
        };

        number.derive_perimeter(3, 3);

        let expected_perimeter: HashSet<_> = vec![(0, 0), (0, 1), (1, 1), (2, 1), (3, 0), (3, 1)]
            .into_iter()
            .collect();

        assert_eq!(number.perimeter, expected_perimeter);
    }
}
