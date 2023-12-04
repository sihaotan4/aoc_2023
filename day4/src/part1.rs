use std::fs;

fn main() {
    let input = read_file("data.txt").unwrap();

    let cards = input
        .lines()
        .map(|s| Card::from_string(s.to_string()))
        .collect::<Vec<Card>>();

    let result = cards
        .iter()
        .map(|card| card.calculate_points())
        .sum::<i32>();

    println!("{}",result);

}

struct Card {
    winning_nums: Vec<u32>,
    card_nums: Vec<u32>,
}

impl Card {
    fn from_string(s: String) -> Card {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            unimplemented!()
        }
    
        // split into winning nums and own nums
        let nums: Vec<&str> = parts[1].split("|").collect();
        if nums.len() != 2 {
            unimplemented!()
        }
    
        let winning_nums = nums[0]
            .trim()
            .replace("  ", " ")
            .split(" ")
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
    
        let card_nums = nums[1]
            .trim()
            .replace("  ", " ")
            .split(" ")
            .map(|c| c.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
    
        Card{winning_nums, card_nums}
    }

    fn calculate_points(&self) -> i32 {
        let mut hits = 0_usize;

        for x in &self.card_nums {
            if self.winning_nums.contains(x) {
                hits += 1;
            }
        }

        match hits {
            0 => { 0 },
            x => {2_i32.pow((x-1) as u32)},
        }
    }
}

fn read_file(filepath: &str) -> std::io::Result<String> {
    fs::read_to_string(filepath)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_string() {
        let s = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11 ".to_string();
        let card = Card::from_string(s);

        assert_eq!(card.winning_nums, vec![31, 18, 13, 56, 72]);
        assert_eq!(card.card_nums, vec![74, 77, 10, 23, 35, 67, 36, 11]);
    }

    #[test]
    fn test_calculate_points() {
        let card = Card {
            winning_nums: vec![31, 18, 13, 56, 72],
            card_nums: vec![74, 77, 10, 23, 35, 67, 36, 11],
        };

        assert_eq!(card.calculate_points(), 0);

        let card = Card {
            winning_nums: vec![31, 18, 13, 56, 72],
            card_nums: vec![31, 18, 13, 56, 72],
        };

        assert_eq!(card.calculate_points(), 16);
    }

}