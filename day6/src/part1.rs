fn main() {
    let time_data:Vec<u32> = vec![48, 93, 84, 66];
    let dist_data:Vec<u32> = vec![261, 1192, 1019, 1063];

    let races = std::iter::zip(time_data, dist_data)
        .map(|(t,d)| {
            Race::init(t, d)
        })
        .collect::<Vec<Race>>();

    let result: u32 = races
        .iter()
        .map(|race| race.num_beat_record())
        .product();

    println!("{}", result);

}

struct Race {
    time: u32,
    dist: u32,
}

impl Race {
    fn init(t:u32, d:u32) -> Race {
        Race { time: t, dist: d }
    }

    fn all_distances(&self) -> Vec<u32> {
        let mut out: Vec<u32> = Vec::new();

        for i in 0..=self.time {
            out.push(i*self.time - (i*i))
        }
        out
    }

    fn num_beat_record(&self) -> u32 {
        self
            .all_distances()
            .into_iter()
            .filter(|x| x > &self.dist)
            .count() as u32
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