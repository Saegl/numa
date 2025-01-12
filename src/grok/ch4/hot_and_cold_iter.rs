fn main() {
    let mut weight: f32 = 0.5;
    let input = 0.5;
    let goal_pred = 0.8;

    let step_amount = 0.01;

    for iteration in 0..1101 {
        let pred = input * weight;
        let error = (pred - goal_pred).powf(2.0);

        println!("{}: Error: {}, Pred: {}", iteration, error, pred);

        let up_pred = input * (weight + step_amount);
        let up_error = (goal_pred - up_pred).powf(2.0);

        let down_pred = input * (weight - step_amount);
        let down_error = (goal_pred - down_pred).powf(2.0);

        if down_error < up_error {
            weight -= step_amount
        }

        if down_error > up_error {
            weight += step_amount
        }
    }
}
