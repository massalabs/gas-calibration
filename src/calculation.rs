use crate::sc_generation::abis;
use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};
use std::{collections::HashMap, time::Duration};

fn initialize_data() -> Vec<(String, Vec<f64>)> {
    let mut data = vec![
        (String::from("Time"), vec![]),
        (String::from("Size"), vec![]),
        (String::from("NbLaunch"), vec![]),
        ];
    let abis = abis::get_abis_full_name();
    for abi in abis {
        data.push((format!("Abi:call:massa.{}", abi.as_str()), vec![]));
    }
    data
}

pub fn calculate_times(results: Vec<(HashMap<String, u64>, Duration)>) -> Vec<Duration> {
    let mut data = initialize_data();
    for (stats, time) in results {
        data[0].1.push(time.as_millis() as f64);
        for (key, value) in data.iter_mut().skip(1) {
            value.push(*stats.get(key).unwrap_or(&0) as f64);
        }
    }
    let mut formula = String::from("Time ~");
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
    println!("params {:#?}", model.iter_parameter_pairs().collect::<Vec<(&str, f64)>>());
    Vec::new()
}
