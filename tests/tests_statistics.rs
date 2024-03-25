#[cfg(test)]
mod tests {
    use estatisticas::statistics::{Statistics, StatisticsResult};
    use std::collections::HashMap;

    #[test]
    fn test_frequency() {
        let nums = vec![2, 2, 5, 5, 5, 5];
        let frequency = Statistics::frequency(&nums);
        let mut result = HashMap::new();

        result.entry(2).or_insert(1);
        *result.entry(2).or_insert(0) += 1;
        result.entry(5).or_insert(1);
        *result.entry(5).or_insert(0) += 1;
        *result.entry(5).or_insert(0) += 1;
        *result.entry(5).or_insert(0) += 1;

        assert_eq!(frequency, result);
    }

    #[test]
    fn test_mean() {
        let nums = vec![2, 5, 3, 9, 14, 26, 12];
        let mean = Statistics::mean(&nums);
        let result = StatisticsResult::Float(10.142857142857142);

        assert_eq!(mean, result);
    }

    #[test]
    fn test_median_not_even(){
        let nums = vec![2, 5, 3, 9, 14, 26, 12];
        let median = Statistics::median(&nums);
        let result = StatisticsResult::Integer(9);

        assert_eq!(median, result);
    }

    #[test]
    fn test_median_even(){
        let nums = vec![2, 5, 3, 9, 14, 26, 12, 25];
        let median = Statistics::median(&nums);
        let result = StatisticsResult::Float(10.5);

        assert_eq!(median, result);
    }

    #[test]
    fn test_mode(){
        let nums = vec![2, 5, 3, 3, 9, 9, 14, 26, 12, 25, 25, 25];
        let median = Statistics::mode(&nums);
        let result = 25;

        assert_eq!(median, result);
    }
}