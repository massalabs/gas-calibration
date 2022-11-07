use std::{collections::HashMap, time::Duration};
use linregress::{FormulaRegressionBuilder, RegressionDataBuilder, assert_almost_eq};

pub fn mock_results() -> HashMap<String, u64> {
    let mut results = HashMap::new();
    results.insert("foo".to_string(), 1);
    results.insert("bar".to_string(), 2);
    results
}

pub fn calculate_times(results: Vec<(HashMap<String, u64>, Duration)>) -> Vec<Duration> {
    let y = vec![3., 6. , 9., 12.];
    let x = vec![1., 2., 3., 4.];
    let x2 = vec![1., 2., 3., 4.];
    let data = vec![("Y", y), ("X", x), ("X2", x2)];
    let data = RegressionDataBuilder::new().build_from(data).unwrap();
    let model = FormulaRegressionBuilder::new().data(&data).formula("Y ~ X + X2").fit().unwrap();
    let parameters: Vec<_> = model.iter_parameter_pairs().collect();
    println!("params {:#?}", parameters);
    Vec::new()
}