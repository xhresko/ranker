use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;

fn parse_value(elem: &str) -> f32 {
    elem.split(':').last().unwrap().parse::<f32>().unwrap()
}

fn parse_features(line: &str) -> Vec<f32> {
    let f_part = line.split('#').nth(0).unwrap().trim();
    let feature_vec: Vec<f32> = f_part.split(' ').
    map(|l| parse_value(l)).collect();
    feature_vec
}

fn total(list: &Vec<f32>) -> f32 {
    list.iter()
    .fold(0.0, |sum, x| sum + x)
}

fn rmse_score(list: &Vec<&Vec<f32>>) -> f32 {
    list.iter()
    .map(|&x| total(x).powi(2) / x.iter().count() as f32)
    .fold(0.0, |sum, x| sum + x)
}


fn main() {
    let input = env::args().nth(1).unwrap();
    let file = File::open(input).unwrap();

    for (num, line) in BufReader::new(file).lines().enumerate() {
        let p_line = line.unwrap();
        let features = parse_features(&p_line);
        let mean_f = mean(&features);
        println!("{}: {:?} {:?}", num, features, mean_f);
    }
}
