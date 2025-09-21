use libafl::state::HasRand;
use libafl_bolts::rands::Rand;

mod inputs;
mod mutators;

// A simple fixed random number generator for deterministic testing
#[derive(Debug, Clone)]
struct FixedRand {
    fixed_values: Vec<u64>,
    current_idx: usize,
}

impl FixedRand {
    fn new(value: u64) -> Self {
        Self {
            fixed_values: vec![value],
            current_idx: 0,
        }
    }

    fn with_values(values: Vec<u64>) -> Self {
        Self {
            fixed_values: values,
            current_idx: 0,
        }
    }
}

impl Rand for FixedRand {
    fn set_seed(&mut self, _seed: u64) {}
    fn next(&mut self) -> u64 {
        let result = self.fixed_values[self.current_idx];
        self.current_idx = self.current_idx.wrapping_add(1) % self.fixed_values.len();
        result
    }

    fn below(&mut self, upper_bound_excl: std::num::NonZeroUsize) -> usize {
        if upper_bound_excl.get() == 0 {
            return 0;
        }
        (self.next() % upper_bound_excl.get() as u64) as usize
    }

    fn below_or_zero(&mut self, n: usize) -> usize {
        if n == 0 {
            0
        } else {
            (self.next() % n as u64) as usize
        }
    }
}

// Mock state object that implements HasRand with our FixedRand
#[derive(Debug)]
struct MockState {
    rand: FixedRand,
}

impl MockState {
    fn new(rand_value: u64) -> Self {
        Self {
            rand: FixedRand::new(rand_value),
        }
    }

    fn with_rand_values(values: Vec<u64>) -> Self {
        Self {
            rand: FixedRand::with_values(values),
        }
    }
}

impl HasRand for MockState {
    type Rand = FixedRand;

    fn rand(&self) -> &Self::Rand {
        &self.rand
    }

    fn rand_mut(&mut self) -> &mut Self::Rand {
        &mut self.rand
    }
}
