mod statistics;

use statistics::Statistics;
use rand::{self, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let mut nums = vec![0;1_000_000];

    for value in  &mut nums{
        *value = rng.gen_range(0..100);
    }

    let mean = Statistics::mean(&nums);
    let median = Statistics::median(&nums);

    println!(
        "A media dos valores é {}\nA mediana dos valores é {}",mean, median);
}
