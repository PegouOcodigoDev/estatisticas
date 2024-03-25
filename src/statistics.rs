use core::fmt;
use std::collections::HashMap;

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
   pub fn mean(nums: &Vec<i32>) -> StatisticsResult{
        let sum: i32 = nums.iter().sum();
        let mean:f64 = sum as f64 / nums.len() as f64;

        if mean.fract() == 0.0 {
            return StatisticsResult::Integer(mean as i32);
        }

        StatisticsResult::Float(mean)
    }

    pub fn median(nums: &Vec<i32>) -> StatisticsResult{
        let len: usize = nums.len();
        let even: bool = len % 2 == 0;
        let middle: usize = len / 2;
        let mut nums_copy = nums.clone();

        nums_copy.sort();

        match even {
            true => {
                let middle_left: usize = middle - 1;
                let middle_elements = vec![nums_copy[middle], nums_copy[middle_left]];
                let mean = Statistics::mean(&middle_elements);
                match mean {
                  StatisticsResult::Integer(value) => StatisticsResult::Integer(value),
                  StatisticsResult::Float(value) => StatisticsResult::Float(value),
                }
                
            }
            false => return StatisticsResult::Integer(nums_copy[middle]),
        }
    }

    pub fn mode(nums: &Vec<i32>) -> i32{
        let mut frequency_map: HashMap<i32, i32> = HashMap::new();
        
        for &num in nums.iter() {
         *frequency_map.entry(num).or_insert(0) += 1;   
        }

        let max = frequency_map.values().cloned().max().unwrap();

        let mode: Vec<_> = frequency_map
        .into_iter()
        .filter(|&(_,value)| value == max )
        .map(|(key,_)| key)
        .collect();

        mode[0]
    }
}