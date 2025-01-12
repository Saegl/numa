fn w_sum(a: Vec<f32>, b: Vec<f32>) -> f32 {
    assert!(a.len() == b.len());
    let mut output = 0.0;

    for i in 0..(a.len()) {
        output += a[i] * b[i];
    }
    output
}

fn neural_network(input: Vec<f32>, weights: Vec<f32>) -> f32 {
    let prediction = w_sum(input, weights);
    prediction
}

fn main() {
    let toes = vec![8.5, 9.5, 10.0, 9.0];
    let wlrec = vec![0.65, 0.8, 0.8, 0.9];
    let nfans = vec![1.2, 1.3, 0.5, 1.0];

    let input = vec![toes[0], wlrec[0], nfans[0]];
    let weights = vec![0.1, 0.2, 0.0];

    let pred = neural_network(input, weights);
    println!("pred: {}", pred);
}
