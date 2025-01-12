fn main() {
    let mut weight: f64 = 0.5;
    let goal_pred = 0.8;
    let input = 0.5;

    for itr in 0..20 {
        let pred = input * weight;
        let error = (pred - goal_pred).powf(2.0);
        let dir_and_amount = (pred - goal_pred) * input;
        weight -= dir_and_amount;

        println!("{}: Error {}, Pred {}", itr, error, pred);
    }
}
