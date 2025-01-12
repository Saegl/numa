fn neural_network(input: f64, weight: f64) -> f64 {
    let pred = input * weight;
    pred
}

fn main() {
    let weight = 0.1;

    let number_of_toes = vec![8.5];
    let win_or_lose_binary = vec![1.0];

    let input = number_of_toes[0];
    let is_true = win_or_lose_binary[0];

    let pred = neural_network(input, weight);

    let error = (pred - is_true).powf(2.0);
    println!("Original error {}", error);

    // Make prediction with higher weight
    let lr = 0.01;
    let p_up = neural_network(input, weight + lr);
    let e_up = (p_up - is_true).powf(2.0);
    println!("Higher weight error {}", e_up);

    // Make prediction with lower weight
    let lr = 0.01;
    let p_dn = neural_network(input, weight - lr);
    let e_dn = (p_dn - is_true).powf(2.0);
    println!("Lower weight error {}", e_dn);
}
