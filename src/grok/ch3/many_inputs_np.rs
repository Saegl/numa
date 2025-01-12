use ndarray::prelude::*;

fn neural_network(input: Array1<f32>, weights: Array1<f32>) -> f32 {
    let prediction = input.dot(&weights);
    prediction
}

fn main() {
    let toes = arr1(&[8.5, 9.5, 10.0, 9.0]);
    let wlrec = arr1(&[0.65, 0.8, 0.8, 0.9]);
    let nfans = arr1(&[1.2, 1.3, 0.5, 1.0]);

    let input = arr1(&[toes[0], wlrec[0], nfans[0]]);
    let weights = arr1(&[0.1, 0.2, 0.0]);

    let pred = neural_network(input, weights);
    println!("pred: {}", pred);
}
