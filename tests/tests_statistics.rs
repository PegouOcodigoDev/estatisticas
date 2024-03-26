#[cfg(test)]
mod tests {
    use estatisticas::statistics::Statistics;
    use std::collections::HashMap;

    #[test]
    fn test_frequency() {
        let nums = vec![2, 2, 5, 5, 5, 5];
        let frequency = Statistics::frequency(&nums);
        let mut result = HashMap::new();

        result.insert(2, 2);
        result.insert(5, 4);

        assert_eq!(frequency, result);
    }

    #[test]
    fn test_mean() {
        let nums = vec![2, 5, 3, 9, 14, 26, 12];
        let mean = Statistics::mean(&nums);
        let result = 10.142857142857142;

        assert_eq!(mean, result);
    }

    #[test]
    fn test_median_not_even(){
        let nums = vec![2, 5, 3, 9, 14, 26, 12];
        let median = Statistics::median(&nums);
        let result = 9.0;

        assert_eq!(median, result);
    }

    #[test]
    fn test_median_even(){
        let nums = vec![2, 5, 3, 9, 14, 26, 12, 25];
        let median = Statistics::median(&nums);
        let result = 10.5;

        assert_eq!(median, result);
    }

    #[test]
    fn test_mode(){
        let nums = vec![2, 5, 3, 3, 9, 9, 14, 26, 12, 25, 25, 25];
        let mode = Statistics::mode(&nums);
        let result = 25;

        assert_eq!(mode, result);
    }

    #[test]
    fn test_deviation_float(){
        let nums = vec![5, 3, 9, 14, 26, 12];
        let deviation = Statistics::deviation(&nums);
        let diff_nums = vec![-6.5, -8.5, -2.5, 2.5, 14.5, 0.5];

        assert_eq!(deviation, diff_nums);     
        
    }

    #[test]
    fn test_deviation_integer(){
        let nums = vec![2, 4, 6, 8, 12, 8, 2];
        let deviation = Statistics::deviation(&nums);
        let diff_nums = vec![-4.0, -2.0, 0.0, 2.0, 6.0, 2.0,-4.0];

        assert_eq!(deviation, diff_nums);     
        
    }

    #[test]
    fn test_squared_deviation_float(){
        let nums = vec![5, 3, 9, 14, 26, 12];
        let squared_deviation = Statistics::squared_deviation(&nums);
        let diff_nums = vec![42.25, 72.25, 6.25, 6.25, 210.25, 0.25];

        assert_eq!(squared_deviation, diff_nums);     
        
    }

    #[test]
    fn test_squared_deviation_integer(){
        let nums = vec![2, 4, 6, 8, 12, 8, 2];
        let squared_deviation = Statistics::squared_deviation(&nums);
        let diff_nums = vec![16.0, 4.0, 0.0, 4.0, 36.0, 4.0, 16.0];

        assert_eq!(squared_deviation, diff_nums);     
        
    }

    #[test]
    fn test_variance(){
        let nums = vec![2, 4, 6, 8, 12, 8, 2];
        let variance = Statistics::variance(&nums);
        let result = 11.428571428571429;

        assert_eq!(variance, result);     
    }

    #[test]
    fn test_standard_deviation(){
        let nums = vec![2, 4, 6, 8, 12, 8, 2];
        let standard_deviation = Statistics::standart_deviation(&nums);
        let result = 3.3806170189140663;

        assert_eq!(standard_deviation, result);     
    }

    #[test]
    fn test_range() {
        let nums = vec![2, 4, 6, 8, 12, 8, 2];
        let range = Statistics::range(&nums);
        let result = 12 - 2;

        assert_eq!(range, result);
    }
}