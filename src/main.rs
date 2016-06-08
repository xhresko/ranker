use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use std::ops::Rem;

fn split_targets(features_vec: &Vec<Vec<f32>>, targets_vec: &Vec<f32>, feature: i32, threshold: f32)
    -> (Vec<f32>, Vec<f32>) {
    let bigger: Vec<f32> = targets_vec.iter().zip(features_vec)
        .filter(|e| e.1[feature as usize] > threshold)
        .map(|f| *f.0)
        .collect();
    let lower: Vec<f32> = targets_vec.iter().zip(features_vec)
        .filter(|e| e.1[feature as usize] <= threshold)
        .map(|f| *f.0)
        .collect();
    (bigger, lower)
}

fn create_thresholds(feature_vec: &Vec<f32>, granularity: u32) -> Vec<f32>{
    let mut uniques: Vec<f32> = vec![];
    let mut list = feature_vec.to_vec();
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut y = 0;
    let mut last = 0.0f32 / 0.0f32; // NAN

    for num in list {
        if num != last {
            y += 1;
            last = num;
            uniques.push(num);
        }
    }

    let mod_used = if (uniques.len() as u32) < (granularity + 1) {
        1
    }
    else {
        ((uniques.len() as u32) / (granularity + 1))
    };

    let tholds: Vec<f32> = uniques.iter().enumerate()
    .filter(|x| (x.0 as i32).rem(mod_used as i32) == 0)
    .map(|x| *x.1)
    .skip(1)
    .collect();
    tholds
}

fn parse_value(elem: &str) -> f32 {
    elem.split(':').last().unwrap().parse::<f32>().unwrap()
}

fn parse_features(line: &str) -> Vec<f32> {
    let f_part = line.split('#').nth(0).unwrap().trim();
    let feature_vec: Vec<f32> = f_part.split(' ').
    map(|l| parse_value(l)).collect();
    feature_vec
}

fn rmse_score(list: &Vec<&Vec<f32>>) -> f32 {
    list.iter()
    .map(|&x| total(x).powi(2) / x.iter().count() as f32)
    .fold(0.0, |sum, x| sum + x)
}

fn mean(list: &Vec<f32>) -> f32 {
    list.iter()
    .fold(0.0, |acc, &x| acc + x) / list.iter().count() as f32
}


fn main() {
    let input = env::args().nth(1).unwrap();
    let file = File::open(input).unwrap();

    let mut all_features: Vec<Vec<f32>> = vec![];
    let mut targets: Vec<f32> = vec![];

    for (num, line) in BufReader::new(file).lines().enumerate() {
        let p_line = line.unwrap();
        let features = parse_features(&p_line);
        targets.push(features[0]);
        all_features.push(features);
        if num == 40 {break};
    }
    let (bigger, lower) = split_targets(&all_features, &targets, 0, 2.0);
    println!("{:?} {:?}", all_features, targets);
    println!("{:?} {:?}", bigger, lower);

}
