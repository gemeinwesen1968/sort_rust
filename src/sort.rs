use std::fmt::Debug;

pub fn merge_sort<T: PartialOrd + Clone + Debug>(vec: &Vec<T>) -> Vec<T> {
    if vec.len() < 2 {
        vec.to_vec()
    } else {
        let size: usize = vec.len() / 2;
        let left: Vec<T> = merge_sort(&vec[0..size].to_vec());
        let right: Vec<T> = merge_sort(&vec[size..].to_vec());
        let merged: Vec<T> = merge(&left, &right);
        merged
    }
}

fn merge<T: PartialOrd + Clone>(left: &Vec<T>, right: &Vec<T>) -> Vec<T> {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut merged: Vec<T> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i].clone());
            i = i + 1;
        } else {
            merged.push(right[j].clone());
            j = j + 1;
        }
    }

    merged.extend(left[i..].iter().cloned());
    merged.extend(right[j..].iter().cloned());

    merged
}

pub fn selection_sort<T: PartialOrd + Clone + Debug>(vec: &Vec<T>) -> Vec<T> {
    let size: usize = vec.len();
    let mut v: Vec<T> = vec.clone();
    for i in 0..size {
        let mut min_index: usize = i;
        for j in i + 1..size {
            if v.get(j) < v.get(min_index) {
                min_index = j;
            }
        }
        v.swap(i, min_index);
    }
    v
} 
