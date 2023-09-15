use super::common_value::CONSTANT;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn empty() -> Interval {
        Interval {
            min: CONSTANT.INFINITY,
            max: -CONSTANT.INFINITY,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        return self.min <= x && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return self.min < x && x < self.max;
    }
}

pub const EMPTY_INTERVAL: Interval = Interval {
    min: CONSTANT.INFINITY,
    max: -CONSTANT.INFINITY,
};
pub const UNIVERSE_INTERVAL: Interval = Interval {
    min: -CONSTANT.INFINITY,
    max: CONSTANT.INFINITY,
};
