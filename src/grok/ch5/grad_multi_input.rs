fn w_sum(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    assert!(a.len() == b.len());
    let mut output = 0.0;
    for i in 0..a.len() {
        output += a[i] * b[i];
    }
    output
}

fn ele_mul(number: f64, vector: &Vec<f64>) -> Vec<f64> {
    let mut output: Vec<f64> = vec![];
    for i in 0..vector.len() {
        output.push(number * vector[i]);
    }
    output
}

fn neural_network(input: &Vec<f64>, weights: &Vec<f64>) -> f64 {
    let pred = w_sum(input, weights);
    pred
}

fn main() {
    let toes = vec![8.5, 9.5, 10.0, 9.0];
    let wlrec = vec![0.65, 0.8, 0.8, 0.9];
    let nfans = vec![1.2, 1.3, 0.5, 1.0];

    let win_or_lose_binary = vec![1.0];

    let input = vec![toes[0], wlrec[0], nfans[0]];
    let is_true = win_or_lose_binary[0];

    let mut weights = vec![0.1, 0.2, -0.1];
    let pred = neural_network(&input, &weights);

    let error = (pred - is_true).powf(2.0);
    let delta = pred - is_true;

    let weight_deltas = ele_mul(delta, &input);
    let alpha = 0.001;

    for i in 0..weights.len() {
        weights[i] -= alpha * weight_deltas[i];
    }

    println!("Weights {weights:?}");
    println!("Weight deltas {weight_deltas:?}");

    // Several steps of learning
    for itr in 0..3 {
        let pred = neural_network(&input, &weights);
        let error = (pred - is_true).powf(2.0);
        let delta = pred - is_true;

        let weight_deltas = ele_mul(delta, &input);
        for i in 0..weights.len() {
            weights[i] -= alpha * weight_deltas[i];
        }

        println!("------------{itr}-------------");
        println!("Pred {pred}, Error {error}");
        println!("Delta {delta}");
        println!("Weights {weights:?}, Weight deltas {weight_deltas:?}");
    }

    // Freezing one weight
    for itr in 0..2000 {
        let pred = neural_network(&input, &weights);
        let error = (pred - is_true).powf(2.0);
        let delta = pred - is_true;

        let mut weight_deltas = ele_mul(delta, &input);
        weight_deltas[0] = 0.0;
        for i in 0..weights.len() {
            weights[i] -= alpha * weight_deltas[i];
        }

        println!("------------{itr}-------------");
        println!("Pred {pred}, Error {error}");
        println!("Delta {delta}");
        println!("Weights {weights:?}, Weight deltas {weight_deltas:?}");
    }
}
