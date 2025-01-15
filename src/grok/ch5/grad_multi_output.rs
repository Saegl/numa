fn ele_mul(number: f64, vector: &Vec<f64>) -> Vec<f64> {
    let mut output = vec![];
    for v in vector {
        output.push(v * number);
    }
    output
}

fn neural_network(input: f64, weights: &Vec<f64>) -> Vec<f64> {
    let pred = ele_mul(input, weights);
    pred
}

fn main() {
    let mut weights = vec![0.3, 0.2, 0.9];

    let wlrec = vec![0.65, 0.8, 0.8, 0.9];

    let hurt = vec![0.1, 0.0, 0.0, 0.1];
    let win = vec![1.0, 1.0, 0.0, 1.0];
    let sad = vec![0.1, 0.0, 0.1, 0.2];

    let input = wlrec[0];
    let is_true = vec![hurt[0], win[0], sad[0]];

    let pred = neural_network(input, &weights);

    let mut error = vec![0.0, 0.0, 0.0];
    let mut delta = vec![0.0, 0.0, 0.0];

    for i in 0..is_true.len() {
        error[i] = (pred[i] - is_true[i]).powf(2.0);
        delta[i] = pred[i] - is_true[i];
    }

    let weight_deltas = ele_mul(input, &weights);
    let alpha = 0.1;

    for i in 0..weights.len() {
        weights[i] = weight_deltas[i] * alpha;
    }

    println!("Weights: {weights:?}");
    println!("Weight Deltas: {weight_deltas:?}");
}
