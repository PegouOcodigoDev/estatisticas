use core::fmt;

pub struct Statistics;

pub enum StatisticsResult {
    Integer(i32),
    Float(f64),
}

impl fmt::Display for StatisticsResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatisticsResult::Integer(value) => write!(f,"{}", value),
            StatisticsResult::Float(value) => writeln!(f, "{:.2}", value)
        }
    }
}

impl Statistics {
   pub fn mean<T: Into<f64> + Copy>(nums: &[T]) -> StatisticsResult{
        let sum: f64 = nums.iter().map(|&x| x.into()).sum();
        let mean:f64 = sum / nums.len() as f64;

        if mean.fract() == 0.0 {
            return StatisticsResult::Integer(mean as i32);
        }

        StatisticsResult::Float(mean)
    }
}