fn w_sum(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    assert!(a.len() == b.len());
    let mut output = 0.0;
    for i in 0..a.len() {
        output += a[i] * b[i];
    }
    output
}

fn vect_mat_mul(vect: &Vec<f64>, matrix: &Vec<Vec<f64>>) -> Vec<f64> {
    let mut output = vec![];
    for i in 0..vect.len() {
        output.push(w_sum(vect, &matrix[i]));
    }
    output
}

fn zero_vec(b: usize) -> Vec<f64> {
    let mut output = vec![];
    for _ in 0..b {
        output.push(0.0);
    }
    output
}

fn zero_matrix(a: usize, b: usize) -> Vec<Vec<f64>> {
    let mut output = vec![];
    for _ in 0..a {
        output.push(zero_vec(b));
    }
    output
}

fn outer_prod(a: &Vec<f64>, b: &Vec<f64>) -> Vec<Vec<f64>> {
    let mut output = zero_matrix(a.len(), b.len());

    for i in 0..a.len() {
        for j in 0..b.len() {
            output[i][j] = a[i] * b[j];
        }
    }
    output
}

fn neural_network(input: &Vec<f64>, weights: &Vec<Vec<f64>>) -> Vec<f64> {
    let pred = vect_mat_mul(input, weights);
    pred
}

fn main() {
    let mut weights = vec![
        vec![0.1, 0.1, -0.3],
        vec![0.1, 0.2, 0.0],
        vec![0.0, 1.3, 0.1],
    ];

    let toes = vec![8.5, 9.5, 10.0, 9.0];
    let wlrec = vec![0.65, 0.8, 0.8, 0.9];
    let nfans = vec![1.2, 1.3, 0.5, 1.0];

    let hurt = vec![0.1, 0.0, 0.0, 0.1];
    let win = vec![1.0, 1.0, 0.0, 1.0];
    let sad = vec![0.1, 0.0, 0.1, 0.2];

    let alpha = 0.01;

    let input = vec![toes[0], wlrec[0], nfans[0]];
    let is_true = vec![hurt[0], win[0], sad[0]];

    for itr in 0..5 {
        let pred = neural_network(&input, &weights);

        let mut error = vec![0.0, 0.0, 0.0];
        let mut delta = vec![0.0, 0.0, 0.0];

        for i in 0..is_true.len() {
            error[i] = (pred[i] - is_true[i]).powf(2.0);
            delta[i] = pred[i] - is_true[i];
        }

        let weight_deltas = outer_prod(&input, &delta);

        for i in 0..weights.len() {
            for j in 0..weights[i].len() {
                weights[i][j] -= alpha * weight_deltas[i][j];
            }
        }

        let mut error_sum = 0.0;
        for e in &error {
            error_sum += e;
        }

        println!("\n\n\n---------------{itr}----------------");
        println!("Pred {pred:?}");
        println!("Error {error:?}");
        println!("Error sum {error_sum}");
        println!("Weight deltas {weight_deltas:?}");
    }
}
