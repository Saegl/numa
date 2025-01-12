fn main() {
    let knob_weight: f32 = 0.5;
    let input = 0.5;
    let goal_pred = 0.8;

    let pred = input * knob_weight;
    let error = (pred - goal_pred).powf(2.0);
    println!("{}", error);
}
