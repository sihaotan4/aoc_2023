use std::fs;
use std::collections::HashMap;

fn main() {
    let input = read_file("data.txt").unwrap();

    let cards = input
        .lines()
        .enumerate()
        .map(|(i,s)| Card::build_from(i+1, s.to_string()))
        .collect::<Vec<Card>>();

    let result = process(cards);

    
    println!("{:?}", result.len());

}

fn process(cards: Vec<Card>) -> Vec<usize> {
    let last_id = cards.len();

    // store running tally card ids
    let mut tally:Vec<usize> = Vec::new(); 

    // hashmap for lookup
    let mut cards_map: HashMap<usize, Card> = cards
        .into_iter()
        .map(|card| (card.id, card))
        .collect();

    // iterate through all the cards but in reverse
    for i in (1..=last_id).rev() {
        let mut also_wins:Vec<usize> = Vec::new();

        let card = cards_map.get(&i).unwrap();

        let lookup_cards = (card.id + 1)..=((card.score as usize + card.id));

        // look up cards that are immediately won
        for k in lookup_cards {
            if cards_map.contains_key(&k) {
                // push the card won
                also_wins.push(k);
                // also push all the cards that this won card has also won
                also_wins.extend(
                    cards_map
                        .get(&k)
                        .unwrap() // should be unreachable
                        .also_wins
                        .as_ref()
                        .unwrap() // should be unreachable
                );
            }
        }
        // get mut to update this cards also_win state
        let card = cards_map.get_mut(&i).unwrap();
        card.also_wins = Some(also_wins.clone());

        // update tally of num cards
        tally.push(i);
        tally.extend(also_wins);
    }

    tally
}

#[derive(Clone, Debug)]
struct Card {
    id: usize,
    winning_nums: Vec<u32>,
    card_nums: Vec<u32>,
    score: usize,
    also_wins: Option<Vec<usize>> 
}

impl Card {
    fn new(id: usize) -> Self {
        Self {
            id,
            winning_nums: Vec::new(),
            card_nums: Vec::new(),
            score: 0,
            also_wins:None,
        }
    }

    fn build_from(id: usize, s: String) -> Card {
        let mut card = Card::new(id);
        
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
    
        card.winning_nums = winning_nums;
        card.card_nums = card_nums;
        card.score = card.calculate_score();
        card
    }

    fn calculate_score(&self) -> usize {
        let mut hits = 0_usize;

        for x in &self.card_nums {
            if self.winning_nums.contains(x) {
                hits += 1;
            }
        }
        hits
    }
}


fn read_file(filepath: &str) -> std::io::Result<String> {
    fs::read_to_string(filepath)
}
