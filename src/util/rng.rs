#[derive(Debug, Clone)]
pub struct SeededRng {
    state: u64,
}

impl SeededRng {
    pub fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0x9E37_79B9_7F4A_7C15,
        }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);

        let xorshifted = (((self.state >> 18) ^ self.state) >> 27) as u32;
        let rot = (self.state >> 59) as u32;
        xorshifted.rotate_right(rot)
    }

    pub fn next_usize_bounded(&mut self, upper_exclusive: usize) -> usize {
        if upper_exclusive <= 1 {
            return 0;
        }

        let upper = upper_exclusive as u32;
        let threshold = u32::MAX - (u32::MAX % upper);

        loop {
            let v = self.next_u32();
            if v < threshold {
                return (v % upper) as usize;
            }
        }
    }

    pub fn next_i32_inclusive(&mut self, min: i32, max: i32) -> i32 {
        if max <= min {
            return min;
        }

        let width = (max - min + 1) as usize;
        min + self.next_usize_bounded(width) as i32
    }

    pub fn next_bool_ratio(&mut self, numer: u32, denom: u32) -> bool {
        if denom == 0 {
            return false;
        }

        let cap = self.next_u32() % denom;
        cap < numer.min(denom)
    }

    pub fn shuffle<T>(&mut self, values: &mut [T]) {
        if values.len() < 2 {
            return;
        }

        for i in (1..values.len()).rev() {
            let j = self.next_usize_bounded(i + 1);
            values.swap(i, j);
        }
    }
}
