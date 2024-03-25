#[cfg(test)]
mod tests {
    use estatisticas::statistics::{Statistics, StatisticsCollections, StatisticsResult};
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

    #[test]
    fn test_deviation_float(){
        let nums = vec![5, 3, 9, 14, 26, 12];
        let mean = Statistics::mean(&nums);
        let deviation = match mean {
            StatisticsResult::Integer(_value) => Statistics::deviation(&nums),
            StatisticsResult::Float(_value) => Statistics::deviation(&nums),
        };
        let diff_nums = StatisticsCollections::Float(vec![-6.5, -8.5, -2.5, 2.5, 14.5, 0.5]);

        assert_eq!(deviation, diff_nums);     
        
    }

    #[test]
    fn test_deviation_integer(){
        let nums = vec![2, 4, 6, 8, 12, 8, 2];
        let mean = Statistics::mean(&nums);
        let deviation = match mean {
            StatisticsResult::Integer(_value) => Statistics::deviation(&nums),
            StatisticsResult::Float(_value) => Statistics::deviation(&nums),
        };
        let diff_nums = StatisticsCollections::Integer(vec![-4, -2, 0, 2, 6, 2,-4]);

        assert_eq!(deviation, diff_nums);     
        
    }

    #[test]
    fn test_squared_deviation_float(){
        let nums = vec![5, 3, 9, 14, 26, 12];
        let squared_deviation = Statistics::squared_deviation(&nums);
        let diff_nums = StatisticsCollections::Float(vec![42.25, 72.25, 6.25, 6.25, 210.25, 0.25]);

        assert_eq!(squared_deviation, diff_nums);     
        
    }

    #[test]
    fn test_squared_deviation_integer(){
        let nums = vec![2, 4, 6, 8, 12, 8, 2];
        let squared_deviation = Statistics::squared_deviation(&nums);
        let diff_nums = StatisticsCollections::Integer(vec![16, 4, 0, 4, 36, 4, 16]);

        assert_eq!(squared_deviation, diff_nums);     
        
    }

    #[test]
    fn test_variance(){
        let nums = vec![2, 4, 6, 8, 12, 8, 2];
        let variance = Statistics::variance(&nums);
        let result = StatisticsResult::Float(11.428571428571429);

        assert_eq!(variance, result);     
    }

}