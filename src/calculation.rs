use ndarray::{Array1, Array2};
use nnls::nnls;
use std::collections::BTreeMap;
use std::io::Write;
use std::{collections::HashMap, fs::File, time::Duration};

use crate::sc_generation;

fn initialize_data(abi_index: usize) -> Vec<(String, Vec<f64>)> {
    let abis = sc_generation::abis::get_abis_full_name();
    let mut data = vec![
        (String::from("Time"), vec![]),
        (String::from("Size"), vec![]),
        (String::from("Launch"), vec![]),
    ];
    data.push((format!("Abi:call:massa.{}", abis[abi_index]), vec![]));
    data
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn mean(data: &[f64]) -> Option<f64> {
    let sum = data.iter().sum::<f64>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

fn std_deviation(data: &[f64]) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f64);

                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    }
}

pub fn compile_and_write_results(
    results: HashMap<String, Vec<f64>>,
    max_gas: u32,
    max_execution_time: Duration,
) -> BTreeMap<String, (f64, usize, f64)> {
    // Mean, number of element, standard deviation
    let mut final_results: BTreeMap<String, (f64, usize, f64)> = BTreeMap::new();
    let mut gas_costs: BTreeMap<String, u32> = BTreeMap::new();
    for (key, value) in results.iter() {
        final_results.insert(key.clone(), (value.iter().sum::<f64>() / value.len() as f64, value.len(), std_deviation(value).unwrap()));
    }
    let mut output = File::create("./results/results.json").unwrap();
    write!(output, "{}", serde_json::to_string(&final_results).unwrap()).unwrap();
    for (key, value) in final_results.iter() {
        gas_costs.insert(
            key.clone(),
            (max_gas as f64 / (max_execution_time.as_millis() as f64 / value.0)) as u32,
        );
    }
    let mut output = File::create("./results/gas_costs.json").unwrap();
    write!(output, "{}", serde_json::to_string(&gas_costs).unwrap()).unwrap();
    final_results
}

pub fn calculate_times(
    abi_index: usize,
    results: Vec<(HashMap<String, u64>, Duration)>,
) -> HashMap<String, f64> {
    let mut data = initialize_data(abi_index);
    for (stats, time) in results {
        data[0].1.push(time.as_nanos() as f64);
        for (key, value) in data.iter_mut().skip(1) {
            value.push(*stats.get(key).unwrap_or(&0) as f64);
        }
    }
    let values: Vec<Vec<f64>> = transpose(data[1..].iter().map(|elem| elem.1.clone()).collect());
    let arr = Array2::from_shape_vec(
        (values.len(), values[0].len()),
        values.into_iter().flatten().collect(),
    )
    .unwrap();
    let times = Array1::from_shape_vec(data[0].1.len(), data[0].1.clone()).unwrap();
    let (alphas, _residual) = nnls(arr.view(), times.view());
    let alphas = alphas
        .iter()
        .enumerate()
        .map(|elem| (data[elem.0 + 1].0.clone(), (*elem.1 / 1_000_000_000f64)))
        .collect::<Vec<(String, f64)>>();
    alphas.into_iter().collect()
}
