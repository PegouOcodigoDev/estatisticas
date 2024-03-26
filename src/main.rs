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
    let squared_deviation = Statistics::squared_deviation(&nums);
    let variance = Statistics::variance(&nums);
    let standart_deviation = Statistics::standart_deviation(&nums);
    let frequency_map = Statistics::frequency(&nums);
    

    println!("\n\nDado o vetor:{:?}...\n\nA media dos valores é: {:.2}\nA mediana dos valores é: {:.2}\n\nA moda dos valores é: {}\n\nO desvio dos valores é: \n{:?}\n\nO desvio dos valores ao quadrado é: \n{:?}\n\nA variancia é: {:.2}\n\nO desvio padrão é {:.2}\n\nFrequência dos valores é: \n{:?}" , &nums[0..30], &mean, &median, &mode,&deviation, &squared_deviation, &variance, &standart_deviation,&frequency_map);
}
