use sort_rust::sort;
use rand::Rng;

fn test<F>(size: usize, offset: i32, generator: F) where F: Fn(&Vec<i32>) -> Vec<i32> {
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let vals: Vec<i32> = (0..size).map(|_| rng.random_range(-offset..offset)).collect();
    let sorted: Vec<i32> = generator(&vals);
    println!("{:?}", vals);
    println!("{:?}\n", sorted);
}

fn main() {
    for _ in 0..5 {
        test(20, 1000, sort::merge_sort);
    }
}
