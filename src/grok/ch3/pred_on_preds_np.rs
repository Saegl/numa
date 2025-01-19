use ndarray::prelude::*;
use ndarray::stack;

fn neural_network(input: &Array1<f32>, weights: &Array3<f32>) -> Array1<f32> {
    let hid_weights = weights.index_axis(Axis(0), 0);
    let hid = input.dot(&hid_weights);

    let pred_weights = weights.index_axis(Axis(0), 1);
    let pred = hid.dot(&pred_weights);

    pred
}

fn main() {
    let toes = arr1(&[8.5, 9.5, 9.9, 9.0]);
    let wlrec = arr1(&[0.65, 0.8, 0.8, 0.9]);
    let nfans = arr1(&[1.2, 1.3, 0.5, 1.0]);
    let input = arr1(&[toes[0], wlrec[0], nfans[0]]);

    #[rustfmt::skip]
    let orig_ih_wgt = arr2(&[
        [0.1, 0.2, -0.1],
        [-0.1, 0.1, 0.9],
        [0.1, 0.4, 0.1],
    ]);

    let ih_wgt = orig_ih_wgt.t();

    #[rustfmt::skip]
    let orig_hp_wgt = arr2(&[
        [0.3, 1.1, -0.3],
        [0.1, 0.2, 0.0],
        [0.0, 1.3, 0.1],
    ]);

    let hp_wgt = orig_hp_wgt.t();

    let weights = stack(Axis(0), &[ih_wgt.view(), hp_wgt.view()]).unwrap();

    let pred = neural_network(&input, &weights);
    println!("pred: {:?}", pred);
}
