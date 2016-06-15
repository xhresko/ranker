use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
use std::ops::Rem;
use std::f32;



/// The `Option` type. See [the module level documentation](index.html) for more.
fn split_targets(features_vec: &Vec<Vec<f32>>,
                 targets_vec: &Vec<f32>,
                 feature: i32,
                 threshold: f32)
                 -> (Vec<f32>, Vec<f32>) {
    let bigger: Vec<f32> = targets_vec.iter()
        .zip(features_vec)
        .filter(|e| e.1[feature as usize] > threshold)
        .map(|f| *f.0)
        .collect();
    let smaller: Vec<f32> = targets_vec.iter()
        .zip(features_vec)
        .filter(|e| e.1[feature as usize] <= threshold)
        .map(|f| *f.0)
        .collect();
    (bigger, smaller)
}


fn split_features(features_vec: &Vec<Vec<f32>>,
                  feature: i32,
                  threshold: f32)
                  -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let bigger: Vec<Vec<f32>> = features_vec.to_vec()
        .iter()
        .filter(|e| e[feature as usize] > threshold)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<f32>>>();
    let smaller: Vec<Vec<f32>> = features_vec.iter()
        .filter(|e| e[feature as usize] <= threshold)
        .map(|x| x.to_vec())
        .collect::<Vec<Vec<f32>>>();
    (bigger, smaller)
}



fn create_thresholds(feature_vec: &Vec<f32>, granularity: u32) -> Vec<f32> {
    let mut uniques: Vec<f32> = vec![];
    let mut list = feature_vec.to_vec();
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut y = 0;
    let mut last = f32::NAN;

    for num in list {
        if num != last {
            y += 1;
            last = num;
            uniques.push(num);
        }
    }

    let mod_used = if (uniques.len() as u32) < (granularity + 1) {
        1
    } else {
        ((uniques.len() as u32) / (granularity + 1))
    };

    let tholds: Vec<f32> = uniques.iter()
        .enumerate()
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
    let feature_vec: Vec<f32> = f_part.split(' ')
        .map(|l| parse_value(l))
        .collect();
    feature_vec
}

fn total(list: &Vec<f32>) -> f32 {
    list.iter()
        .fold(0.0, |sum, x| sum + x)
}


fn rmse_score(list: &Vec<Vec<f32>>) -> f32 {
    let score = list.iter()
        .map(|x| total(x).powi(2) / x.iter().count() as f32)
        .fold(0.0, |sum, x| sum + x);
    match score.is_nan() {
        true => 0.0,
        false => score,
    }
}

fn mean(list: &Vec<f32>) -> f32 {
    list.iter()
        .fold(0.0, |acc, &x| acc + x) / list.iter().count() as f32
}

fn read_data(input_file_name: &str) -> (Vec<Vec<f32>>, Vec<f32>) {
    let file = File::open(input_file_name).unwrap();

    let mut all_features: Vec<Vec<f32>> = vec![];
    let mut targets: Vec<f32> = vec![];

    for (num, line) in BufReader::new(file).lines().enumerate() {
        let p_line = line.unwrap();
        let features = parse_features(&p_line);
        targets.push(features[0]);
        all_features.push(features);
    }
    (all_features, targets)
}

fn get_all_thresholds(all_features: &Vec<Vec<f32>>, granularity: u32) -> Vec<Vec<f32>> {
    let mut thresholds: Vec<Vec<f32>> = vec![];
    let feat_num = all_features[0].len();

    for f in 0..feat_num {
        let f_vec: Vec<f32> = all_features.iter().map(|x| x[f]).collect();
        let t_tholds: Vec<f32> = create_thresholds(&f_vec, granularity);
        thresholds.push(t_tholds);
    }
    thresholds
}

fn find_oblivious_split(ftr_bins: &Vec<Vec<Vec<f32>>>,
                        tar_bins: &Vec<Vec<f32>>,
                        thresholds: &Vec<Vec<f32>>)
                        -> (usize, f32, f32) {
    let mut best_th = (0, 0.0, -1.0);
    let feat_num = ftr_bins[0][0].len();

    for f in 2..feat_num {
        for t in &thresholds[f] {
            // println!("Testing feature {:?} on threshold {:?}", f, t);
            let mut bins: Vec<Vec<f32>> = vec![];
            let tuples: Vec<(Vec<f32>, Vec<f32>)> = ftr_bins.iter()
                .zip(tar_bins.iter())
                .map(|x| split_targets(x.0, x.1, f as i32, *t as f32))
                .collect();

            for (mi, ma) in tuples {
                bins.push(mi);
                bins.push(ma);
            }

            let mses = rmse_score(&bins);
            best_th = if mses >= best_th.2 {
                (f, *t as f32, mses)
            } else {
                best_th
            }
        }
    }
    best_th

}

fn split_all_features(all_features_vec: &Vec<Vec<Vec<f32>>>,
                      all_targets_vec: &Vec<Vec<f32>>,
                      feature: i32,
                      threshold: f32)
                      -> (Vec<Vec<Vec<f32>>>, Vec<Vec<f32>>) {
    let mut result_ftrs: Vec<Vec<Vec<f32>>> = vec![];
    let mut result_tars: Vec<Vec<f32>> = vec![];

    for (fv, tv) in all_features_vec.iter().zip(all_targets_vec) {
        let (bigger, smaller) = split_features(fv, feature, threshold);
        let (bigger_t, smaller_t) = split_targets(fv, tv, feature, threshold);
        result_ftrs.push(bigger);
        result_ftrs.push(smaller);
        result_tars.push(bigger_t);
        result_tars.push(smaller_t);
    }
    (result_ftrs, result_tars)

}



fn main() {
    let nan: f32 = f32::NAN;
    let input = env::args().nth(1).unwrap();
    let granularity = 100;
    let (all_features, targets) = read_data(&input);

    let samples_num = all_features.len() as i32;
    if samples_num == 0 {
        println!("No data in given file!");
        return;
    }


    let feat_num = all_features[0].len();
    let thresholds = get_all_thresholds(&all_features, granularity);

    println!("Datafile {} contains {} samples and {} features.",
             &input,
             &samples_num,
             &feat_num);
    // println!("{:?}", thresholds);


    let mut bin_features = vec![all_features];
    let mut bin_targets = vec![targets];

    for level in 0..3 {

        let best_th = find_oblivious_split(&bin_features, &bin_targets, &thresholds);
        println!("Found best threshold on feature {} on value {} with score {}",
                 best_th.0,
                 best_th.1,
                 best_th.2);
        let (new_f, new_t) =
            split_all_features(&bin_features, &bin_targets, best_th.0 as i32, best_th.1);
        bin_features = new_f;
        bin_targets = new_t;
        //for bt in &bin_targets {
        //    let mn = mean(bt);
        //    println!("{:?} {:?}", mn, bt);
        //}


    }


}
