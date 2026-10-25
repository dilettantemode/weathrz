use rand::{Rng, RngExt};

#[derive(Debug, Default)]
pub struct ShuffleBag {
    queue: Vec<&'static str>,
    last: Option<&'static str>,
}

impl ShuffleBag {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn next(
        &mut self,
        all: &[&'static str],
        rng: &mut (impl Rng + ?Sized),
        eligible: impl Fn(&str) -> bool,
    ) -> Option<&'static str> {
        if !self.queue.iter().any(|id| eligible(id)) {
            self.top_up(all, rng);
        }
        let position = self.queue.iter().position(|id| eligible(id))?;
        let id = self.queue.remove(position);
        self.last = Some(id);
        Some(id)
    }

    fn top_up(&mut self, all: &[&'static str], rng: &mut (impl Rng + ?Sized)) {
        let mut missing: Vec<&'static str> = all
            .iter()
            .copied()
            .filter(|id| !self.queue.contains(id))
            .collect();
        for i in (1..missing.len()).rev() {
            let j = rng.random_range(0..=i);
            missing.swap(i, j);
        }
        if missing.len() > 1 && missing.first().copied() == self.last {
            let end = missing.len() - 1;
            missing.swap(0, end);
        }
        self.queue.extend(missing);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    const ALL: [&str; 5] = ["a", "b", "c", "d", "e"];

    #[test]
    fn every_id_flies_once_per_cycle() {
        let mut bag = ShuffleBag::new();
        let mut rng = StdRng::seed_from_u64(1);
        for _ in 0..4 {
            let mut cycle: Vec<&str> = (0..5)
                .map(|_| bag.next(&ALL, &mut rng, |_| true).unwrap())
                .collect();
            cycle.sort();
            assert_eq!(cycle, ALL);
        }
    }

    #[test]
    fn a_new_cycle_never_repeats_the_last_flyer() {
        for seed in 0..50 {
            let mut bag = ShuffleBag::new();
            let mut rng = StdRng::seed_from_u64(seed);
            let flights: Vec<&str> = (0..40)
                .map(|_| bag.next(&ALL, &mut rng, |_| true).unwrap())
                .collect();
            assert!(
                flights.windows(2).all(|pair| pair[0] != pair[1]),
                "seed {seed}: {flights:?}"
            );
        }
    }

    #[test]
    fn ineligible_flyers_wait_for_their_turn() {
        let mut bag = ShuffleBag::new();
        let mut rng = StdRng::seed_from_u64(2);
        let night: Vec<&str> = (0..8)
            .map(|_| bag.next(&ALL, &mut rng, |id| id != "c").unwrap())
            .collect();
        assert!(!night.contains(&"c"));
        assert!(bag.len() <= ALL.len());
        let day: Vec<&str> = (0..5)
            .map(|_| bag.next(&ALL, &mut rng, |_| true).unwrap())
            .collect();
        assert!(day.contains(&"c"), "{day:?}");
    }

    #[test]
    fn nothing_eligible_gives_none() {
        let mut bag = ShuffleBag::new();
        let mut rng = StdRng::seed_from_u64(3);
        assert_eq!(bag.next(&ALL, &mut rng, |_| false), None);
        assert!(bag.len() <= ALL.len());
        assert!(!bag.is_empty());
    }
}
