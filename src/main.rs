use rand::{self, Rng};
mod statistics;
use statistics::statistics::Statistics;

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums = vec![0;1_000_000];

    for value in  &mut nums{
        *value = rng.gen_range(0..100);
    }

    
    let mean = Statistics::mean(&nums);
    let median = Statistics::median(&nums);
    let mode = Statistics::mode(&nums);

    println!("A media dos valores é {}\nA mediana dos valores é {}\nA moda dos valores é {}",mean, median, mode);
}
