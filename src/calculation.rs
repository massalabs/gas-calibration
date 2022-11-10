use crate::sc_generation::abis;
use nnls::nnls;
use ndarray::{Array1, Array2};
use std::{collections::HashMap, time::Duration};

fn initialize_data() -> Vec<(String, Vec<f64>)> {
    let mut data = vec![
        (String::from("Time"), vec![]),
        //(String::from("Size"), vec![]),
        (String::from("Launch"), vec![]),
        ];
    let abis = abis::get_abis_full_name();
    for abi in abis {
        if abi == "assembly_script_call" {
            continue;
        }
        data.push((format!("Abi:call:massa.{}", abi.as_str()), vec![]));
    }
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

pub fn calculate_times(results: Vec<(HashMap<String, u64>, Duration)>) -> Vec<Duration> {
    let mut data = initialize_data();
    for (stats, time) in results {
        data[0].1.push(time.as_nanos() as f64);
        for (key, value) in data.iter_mut().skip(1) {
            value.push(*stats.get(key).unwrap_or(&0) as f64);
        }
    }
    let values: Vec<Vec<f64>> = transpose(data[1..].iter().map(|elem| elem.1.clone()).collect());
    println!("nb batches {}", values.len());
    let arr = Array2::from_shape_vec((values.len(), values[0].len()), values.into_iter().flatten().collect()).unwrap();
    for i in 0..5 {
        let row = arr.row(i);
        println!("row {}: {:?}", i, row);
    }
    let times = Array1::from_shape_vec(data[0].1.len(), data[0].1.clone()).unwrap();
    for i in 0..5 {
        println!("time {}: {:?}", i, times[i]);
    }
    let (alphas, _residual) = nnls(arr.view(), times.view());
    let alphas = alphas.iter().enumerate().map(|elem| (data[elem.0 + 1].0.clone(), *elem.1 )).collect::<Vec<(String, f64)>>();
    println!("alphas2: {:?}", alphas);
    Vec::new()
}
