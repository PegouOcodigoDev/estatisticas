use core::fmt;
use std::collections::HashMap;

pub struct Statistics;

#[derive(Debug, PartialEq)]
pub enum StatisticsResult {
    Integer(i32),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub enum StatisticsCollections {
    Integer(Vec<i32>),
    Float(Vec<f64>),
}

impl fmt::Display for StatisticsResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatisticsResult::Integer(value) => write!(f,"{}", value),
            StatisticsResult::Float(value) => writeln!(f, "{:.2}", value)
        }
    }
}

impl fmt::Display for StatisticsCollections {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatisticsCollections::Integer(value) =>{ 
                write!(f,"[{:?}]", value)
            },
            StatisticsCollections::Float(value) => {
                let formated_vetor = value.iter().map(|&v| format!("{:.2}", v)).collect::<Vec<String>>().join(",");
                writeln!(f, "[{:?}]", formated_vetor)
            }
        }
    }
}

impl Statistics {
    
    pub fn frequency(nums: &Vec<i32>) -> HashMap<i32, i32>{
        let mut frequency_map: HashMap<i32, i32> = HashMap::new();
        
        for &num in nums.iter() {
        *frequency_map.entry(num).or_insert(0) += 1;   
        }

        frequency_map
    }

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

            if even{
                let middle_left: usize = middle - 1;
                let middle_elements = vec![nums_copy[middle], nums_copy[middle_left]];
                return  Statistics::mean(&middle_elements);
            }
            
            StatisticsResult::Integer(nums_copy[middle])
    }

    pub fn mode(nums: &Vec<i32>) -> i32{
        
        let frequency_map = Statistics::frequency(&nums);

        let max = frequency_map.values().cloned().max().unwrap();

        let mode: Vec<_> = frequency_map
        .into_iter()
        .filter(|&(_,value)| value == max )
        .map(|(key,_)| key)
        .collect();

        mode[0]
        }

    pub fn deviation(nums: &Vec<i32>) -> StatisticsCollections{
        let mean = Statistics::mean(&nums);
        match mean {
            StatisticsResult::Integer(value) => {
                let diff_of_nums = nums.iter().map(|n| *n - value).collect::<Vec<_>>();
                StatisticsCollections::Integer(diff_of_nums)
            },
            StatisticsResult::Float(value) => {
                let diff_of_nums = nums.iter().map(|v| *v as f64).map(|v| v - value).collect::<Vec<_>>();
                StatisticsCollections::Float(diff_of_nums)
            }
        }
    }
}