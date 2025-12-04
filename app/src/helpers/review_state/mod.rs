use chrono::{DateTime, Duration, Utc};

#[cfg(test)]
mod review_state_test;

#[derive(Debug, Clone)]
pub struct ReviewState {
    pub interval: u32,
    pub ease_factor: f32,
    pub repetitions: u32,
}

impl ReviewState {
    pub fn new() -> Self {
        Self {
            interval: 1,
            ease_factor: 2.5,
            repetitions: 0,
        }
    }

    pub fn from(interval: u32, ease_factor: f32, repetitions: u32) -> Self {
        Self {
            interval: interval.max(1),
            ease_factor: ease_factor.max(1.3),
            repetitions,
        }
    }

    pub fn review(mut self, quality: u8) -> Self {
        if quality < 2 {
            self.repetitions = 0;
            self.interval = 1;
            self.ease_factor = (self.ease_factor - 0.2).max(1.3);
            return self;
        }

        self.repetitions += 1;

        if self.repetitions == 1 {
            self.interval = 1;
        } else if self.repetitions == 2 {
            self.interval = 2;
        } else if self.repetitions == 3 {
            self.interval = 4;
        } else {
            self.interval = (self.interval as f32 * self.ease_factor).round() as u32;
        }

        let ef = self.ease_factor
            + (0.1 - (3.0 - quality as f32) * (0.08 + (3.0 - quality as f32) * 0.02));

        self.ease_factor = ef.max(1.3);

        self
    }

    pub fn rounded(&self) -> Self {
        Self {
            interval: self.interval,
            repetitions: self.repetitions,
            ease_factor: (self.ease_factor * 1000.0).round() / 1000.0,
        }
    }

    pub fn next_review_date_from(&self, from: DateTime<Utc>) -> DateTime<Utc> {
        from + Duration::days(self.interval as i64)
    }
}
