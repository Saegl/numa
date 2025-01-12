fn neural_network(input: f32, weight: f32) -> f32 {
    let prediction = input * weight;
    prediction
}

fn main() {
    let number_of_toes = vec![8.5, 9.5, 10.0, 9.0];
    let input = number_of_toes[0];
    let weight = 0.1;
    let pred = neural_network(input, weight);
    println!("pred: {}", pred);
}
