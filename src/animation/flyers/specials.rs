use rand::{Rng, RngExt};

pub const DEFAULT_SPECIALS: u32 = 12;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Specials {
    every: u32,
    left: Option<u32>,
}

impl Specials {
    pub fn new(every: u32) -> Self {
        Self { every, left: None }
    }

    pub fn draw(every: u32, rng: &mut (impl Rng + ?Sized)) -> u32 {
        let low = ((every as f32) * 0.75).round().max(1.0) as u32;
        let high = ((every as f32) * 1.25).round().max(1.0) as u32;
        rng.random_range(low..=high)
    }

    pub fn due(&mut self, rng: &mut (impl Rng + ?Sized)) -> bool {
        if self.every == 0 {
            return false;
        }
        let every = self.every;
        *self.left.get_or_insert_with(|| Self::draw(every, rng)) == 1
    }

    pub fn aircraft_flew(&mut self) {
        if let Some(left) = self.left.as_mut().filter(|left| **left > 1) {
            *left -= 1;
        }
    }

    pub fn character_flew(&mut self, rng: &mut (impl Rng + ?Sized)) {
        self.left = Some(Self::draw(self.every, rng));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn twelve_draws_nine_to_fifteen() {
        let mut rng = StdRng::seed_from_u64(1);
        let draws: Vec<u32> = (0..500).map(|_| Specials::draw(12, &mut rng)).collect();
        assert_eq!(draws.iter().min(), Some(&9));
        assert_eq!(draws.iter().max(), Some(&15));
        assert_eq!(Specials::draw(1, &mut rng), 1);
    }

    #[test]
    fn the_nth_flight_is_the_character() {
        let mut rng = StdRng::seed_from_u64(2);
        let mut specials = Specials::new(12);
        let mut flights = 1;
        while !specials.due(&mut rng) {
            specials.aircraft_flew();
            flights += 1;
        }
        assert!((9..=15).contains(&flights), "{flights}");
        specials.aircraft_flew();
        assert!(
            specials.due(&mut rng),
            "a special stays due until a character flies"
        );
        specials.character_flew(&mut rng);
        assert!(!specials.due(&mut rng));
    }

    #[test]
    fn one_makes_every_flight_a_character_and_zero_none() {
        let mut rng = StdRng::seed_from_u64(3);
        let mut every = Specials::new(1);
        for _ in 0..5 {
            assert!(every.due(&mut rng));
            every.character_flew(&mut rng);
        }
        let mut never = Specials::new(0);
        for _ in 0..50 {
            assert!(!never.due(&mut rng));
            never.aircraft_flew();
        }
    }
}
