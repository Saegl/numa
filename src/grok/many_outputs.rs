fn ele_mul(number: f32, vec: Vec<f32>) -> Vec<f32> {
    let mut output = vec![0.0, 0.0, 0.0];
    assert!(vec.len() == output.len());

    for i in 0..(vec.len()) {
        output[i] = number * vec[i];
    }
    output
}

fn neural_network(input: f32, weights: Vec<f32>) -> Vec<f32> {
    let prediction = ele_mul(input, weights);
    prediction
}

fn main() {
    let wlrec = vec![0.65, 0.8, 0.8, 0.9];

    let input = wlrec[0];
    let weights = vec![0.3, 0.2, 0.9];

    let pred = neural_network(input, weights);
    println!("pred: {:?}", pred);
}
