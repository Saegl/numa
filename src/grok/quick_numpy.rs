use ndarray::prelude::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

fn main() {
    // Part 1
    let a = arr1(&[0.0, 1.0, 2.0, 3.0]);
    let b = arr1(&[4.0, 5.0, 6.0, 7.0]);
    let c = arr2(&[[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]);

    let d: Array2<f32> = Array::zeros((2, 5));
    let e = Array::random((2, 5), Uniform::new(0.0, 10.0));

    println!("{}", a);
    println!("{:?}", a.shape());
    println!("{}", b);
    println!("{}", c);
    println!("{:?}", c.shape());
    println!("{}", d);
    println!("{}", e);

    println!("{}", a.clone() * 0.1);
    println!("{}", c.clone() * 0.2);
    println!("{}", a.clone() * b.clone());
    println!("{}", a.clone() * b.clone() * 0.2);
    println!("{}", a.clone() * c.clone());

    // Expected error: incompatible shape at runtime
    // println!("{}", a.clone() * e.clone());

    // Part 2

    let a: Array2<f32> = Array::zeros((1, 4));
    let b: Array2<f32> = Array::zeros((4, 3));

    let c = a.dot(&b);
    println!("{:?}", c.shape());

    // Part 3

    let a: Array2<f32> = Array::zeros((2, 4));
    let b: Array2<f32> = Array::zeros((4, 3));
    let c = a.dot(&b);
    println!("{:?}", c.shape());

    let e: Array2<f32> = Array::zeros((2, 1));
    let f: Array2<f32> = Array::zeros((1, 3));
    let g = e.dot(&f);
    println!("{:?}", g.shape());

    let h: Array2<f32> = Array::zeros((5, 4));
    let h = h.t();
    let i: Array2<f32> = Array::zeros((5, 6));
    let j = h.dot(&i);
    println!("{:?}", j.shape());

    // Expected error: incompatible shape at runtime
    // let h: Array2<f32> = Array::zeros((5, 4));
    // let i: Array2<f32> = Array::zeros((5, 6));
    // let j = h.dot(&i);
    // println!("{:?}", j.shape());
}
