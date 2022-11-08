use crate::sc_generation::abis;
use linregress::{assert_almost_eq, FormulaRegressionBuilder, RegressionDataBuilder};
use std::{collections::HashMap, time::Duration};

pub fn mock_results() -> HashMap<String, u64> {
    let mut results = HashMap::new();
    results.insert("Abi:call:get".to_string(), 1);
    results.insert("Abi:call:set".to_string(), 2);
    results
}

fn initialize_data() -> Vec<(String, Vec<f64>)> {
    let mut data = vec![(String::from("time"), vec![])];
    let abis = abis::get_abis();
    for abi in abis {
        data.push((format!("Abi:call:{}", abi[0].as_str()), vec![]));
    }
    data
}

pub fn calculate_times(results: Vec<(HashMap<String, u64>, Duration)>) -> Vec<Duration> {
    let mut data = initialize_data();
    for (stats, time) in results {
        data[0].1.push(time.as_nanos() as f64);
        for (key, value) in data.iter_mut().skip(1) {
            value.push(*stats.get(key).unwrap_or(&0) as f64);
        }
    }
    let mut formula = String::from("time ~");
    for (key, _) in data.iter().skip(1) {
        formula.push_str(&format!(" {} +", key));
    }
    formula.pop();
    println!("{:#?}", data);
    println!("{}", formula);
    let data = RegressionDataBuilder::new().build_from(data).unwrap();
    let model = FormulaRegressionBuilder::new()
        .data(&data)
        .formula(formula)
        .fit()
        .unwrap();
    let parameters: Vec<_> = model.iter_parameter_pairs().collect();
    println!("params {:#?}", parameters);
    Vec::new()
}
