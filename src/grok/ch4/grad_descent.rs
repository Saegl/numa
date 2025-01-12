fn neural_network(input: f64, weight: f64) -> f64 {
    let pred = input * weight;
    pred
}

fn main() {
    let mut weight: f64 = 0.1;
    let alpha = 0.001;

    let number_of_toes = vec![8.5];
    let win_or_lose_binary = vec![1.0];

    let input = number_of_toes[0];
    let goal_pred = win_or_lose_binary[0];

    let pred = neural_network(input, weight);

    let error = (pred - goal_pred).powf(2.0);
    let delta = pred - goal_pred;

    let weight_delta = input * delta;
    weight -= weight_delta * alpha;
    println!(
        "Error {}, weight_delta {}, new_weight {}",
        error, weight_delta, weight
    );

    let mut weight = 0.0;
    let goal_pred = 0.8;
    let input = 0.5;

    // Few iterations
    for itr in 0..4 {
        let pred = neural_network(input, weight);
        let error = (pred - goal_pred).powf(2.0);
        let delta = pred - goal_pred;
        let weight_delta = delta * input;
        weight -= weight_delta;
        println!("{} Error: {}, Pred: {}", itr, error, pred);
    }

    // Watch closely

    let mut weight = 0.0;
    let goal_pred = 0.8;
    let input = 0.5;

    // Few iterations
    for itr in 0..4 {
        println!("-----\n{} - Weight: {}", itr, weight);
        let pred = neural_network(input, weight);
        let error = (pred - goal_pred).powf(2.0);
        let delta = pred - goal_pred;
        let weight_delta = delta * input;
        weight -= weight_delta;
        println!("Error: {}, Pred: {}", error, pred);
        println!("Delta: {}, Weight Delta: {}", delta, weight_delta);
    }
}
