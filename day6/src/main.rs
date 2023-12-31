fn main() {
    let time_data:Vec<u64> = vec![48938466];
    let dist_data:Vec<u64> = vec![261119210191063];

    let races = std::iter::zip(time_data, dist_data)
        .map(|(t,d)| {
            Race::init(t, d)
        })
        .collect::<Vec<Race>>();

    let result: u64 = races
        .iter()
        .map(|race| race.num_beat_record())
        .product();

    println!("{}", result);

}

struct Race {
    time: u64,
    dist: u64,
}

impl Race {
    fn init(t:u64, d:u64) -> Race {
        Race { time: t, dist: d }
    }

    fn all_distances(&self) -> Vec<u64> {
        let mut out: Vec<u64> = Vec::new();

        for i in 0..=self.time {
            out.push(i*self.time - (i*i))
        }
        out
    }

    fn num_beat_record(&self) -> u64 {
        self
            .all_distances()
            .into_iter()
            .filter(|x| x > &self.dist)
            .count() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_distances() {
        let race = Race::init(7, 9); // Initialize a Race instance
        let result = race.all_distances();
        let expected = vec![0, 6, 10, 12, 12, 10, 6, 0]; // Expected output based on the all_distances method logic
        assert_eq!(result, expected);
    }
}