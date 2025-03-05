use std::fmt::Debug;

use sort_rust::sort;
use rand::Rng;

fn test<F, T>(size: usize, offset: i32, generator: F) 
where 
    F: Fn(&Vec<T>) -> Vec<T>,
    T: PartialOrd + Clone + Debug + From<i32>, 
{
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    let vals: Vec<T> = (0..size).map(|_| T::from(rng.random_range(-offset..offset))).collect();
    let sorted: Vec<T> = generator(&vals);
    println!("{:?}", vals);
    println!("{:?}\n", sorted);
}

fn main() {
    test(20, 1000, sort::merge_sort::<i32>);
}
