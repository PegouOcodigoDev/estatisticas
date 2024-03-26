use std::collections::HashMap;

pub struct Statistics;

impl Statistics {
    pub fn frequency(nums: &Vec<i32>) -> HashMap<i32, i32> {
        let mut frequency_map: HashMap<i32, i32> = HashMap::new();
        
        for &num in nums.iter() {
            *frequency_map.entry(num).or_insert(0) += 1;   
        }

        frequency_map
    }

    pub fn mean(nums: &Vec<i32>) -> f64 {
        nums.iter().sum::<i32>() as f64 / nums.len() as f64
    }

    pub fn meanf(nums: &Vec<f64>) -> f64 {
        nums.iter().sum::<f64>() / nums.len() as f64
    }

    pub fn median(nums: &Vec<i32>) -> f64 {
        let len: usize = nums.len();
        let even: bool = len % 2 == 0;
        let middle: usize = len / 2;
        let mut nums_copy = nums.clone();

        nums_copy.sort();

        if even {
            let middle_left: usize = middle - 1;
            let middle_elements = vec![nums_copy[middle], nums_copy[middle_left]];
            return Statistics::mean(&middle_elements);
        }
            
        nums_copy[middle] as f64
    }

    pub fn mode(nums: &Vec<i32>) -> i32 {
        let frequency_map = Statistics::frequency(&nums);

        let max = frequency_map.values().cloned().max().unwrap();

        let mode: Vec<_> = frequency_map
            .into_iter()
            .filter(|&(_,value)| value == max )
            .map(|(key,_)| key)
            .collect();

        mode[0]
    }

    pub fn deviation(nums: &Vec<i32>) -> Vec<f64> {
        let mean = Statistics::mean(&nums);
        nums.iter().map(|v| *v as f64 - mean).collect()
    }

    pub fn squared_deviation(nums: &Vec<i32>) -> Vec<f64>{
        let deviation = Statistics::deviation(&nums);
        deviation.iter().map(|v| v.powf(2.0)).collect()
    }

    pub fn variance(nums: &Vec<i32>) -> f64 {
        let squared_deviation = Statistics::squared_deviation(&nums);
        Statistics::meanf(&squared_deviation)
    }

    pub fn standart_deviation(nums: &Vec<i32>) -> f64{
        let variance = Statistics::variance(&nums);
        variance.sqrt()
    }
}