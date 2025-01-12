fn neural_network(input: f64, weight: f64) -> f64 {
    let pred = input * weight;
    pred
}

fn main() {
    // Exercise to recreate chapter material from memory
    let mut weight = 1.0;
    let alpha = 1.0;
    let num_iterations = 50;

    let input = 0.9;
    let goal_pred = -0.4;

    for itrs in 0..num_iterations {
        let pred = neural_network(input, weight);
        let error = (pred - goal_pred).powf(2.0);

        let delta = pred - goal_pred;
        let weight_delta = delta * input;
        weight -= weight_delta * alpha;

        println!("------------------");
        println!("N: {}", itrs);
        println!("Pred {}, Error {}", pred, error);
        println!("Delta {}, Weight delta {}", delta, weight_delta);
        println!("New weight {}", weight);
    }
}
