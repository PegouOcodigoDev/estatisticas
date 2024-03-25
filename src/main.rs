use rand::{self, Rng};
mod statistics;
use statistics::statistics::Statistics;

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums = vec![0;1_000];

    for value in  &mut nums{
        *value = rng.gen_range(0..100);
    }

    let mean = Statistics::mean(&nums);
    let median = Statistics::median(&nums);
    let mode = Statistics::mode(&nums);
    let deviation = Statistics::deviation(&nums);
    let frequency_map = Statistics::frequency(&nums);
    

    println!("\n\nDado o vetor:{:?}\n\nA media dos valores é: {}\n\nA mediana dos valores é: {}\n\nA moda dos valores é: {}\n\nO desvio dos valores é: \n{}\n\nFrequência dos valores é: \n{:?}" , &nums[500..520], &mean, &median, &mode,&deviation,&frequency_map);
}
