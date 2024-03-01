use core::f64;
use rand::random;
use std::iter::from_fn;

fn main() {
    println!("{}", triangle(3));
    // let num = rand_dist(1000);
    // println!("{:?}", num);
}

fn triangle(n: i32) -> i32 {
    // let mut num = 0;
    // for i in 1..n {
    //     num += i;
    // }
    // num
    // (1..=n).fold(0, |sum, item| sum + item)
    let v = vec!["zeb".to_string(), "yang".to_string()];
    let mut iterator = (&v).iter();
    while let Some(element) = iterator.next() {
        println!("{}", element);
    }
    (1..=n).fold(0, |sum, item| sum + item)
}

fn rand_dist(num: usize) -> Vec<f64> {
    from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(num)
        .collect()
}
