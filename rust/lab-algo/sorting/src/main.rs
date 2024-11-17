use rand::Rng;
use std::env;

pub fn partition(arr: &mut Vec<f64>, low: usize, high: usize) -> usize {
    let pivot = arr[low];
    let mut i = low;
    let mut j = high;

    while i < j {
        while i <= high - 1 && arr[i] <= pivot {
            i += 1;
        }

        while j >= low + 1 && arr[j] > pivot {
            j -= 1;
        }

        if i < j {
            arr.swap(i, j);
        }
    }
    arr.swap(low, j);
    j
}

pub fn quick_sort(arr: &mut Vec<f64>, low: usize, high: usize) {
    if low < high {
        let pi = partition(arr, low, high);

        if pi > 0 {
            quick_sort(arr, low, pi - 1);
        }
        quick_sort(arr, pi + 1, high);
    }
}

pub fn merge_sort(A: &mut Vec<f64>, start: usize, end: usize) {
    if start < end {
        let mid = (start + end) / 2;
        merge_sort(A, start, mid);
        merge_sort(A, mid + 1, end);
        merge(A, start, mid, end);
    }
}

pub fn merge(A: &mut Vec<f64>, start: usize, mid: usize, end: usize) {
    let mut p = start;
    let mut q = mid + 1;
    let mut arr: Vec<f64> = vec![0.0; end - start + 1];
    let mut k = 0;

    while p <= mid && q <= end {
        if A[p] < A[q] {
            arr[k] = A[p];
            p += 1;
        } else {
            arr[k] = A[q];
            q += 1;
        }
        k += 1;
    }

    while p <= mid {
        arr[k] = A[p];
        p += 1;
        k += 1;
    }

    while q <= end {
        arr[k] = A[q];
        q += 1;
        k += 1;
    }

    for i in 0..arr.len() {
        A[start + i] = arr[i];
    }
}

pub fn split_vec_in_random_place(some_vec: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let min = 0;
    let max = 45;
    let mut rng = rand::thread_rng();
    let max = if max % 2 == 0 { max } else { max - 1 };
    let random_even: usize = rng.gen_range(min..=max / 2) * 2;
    let b = some_vec.split_at(random_even);
    (b.0.to_vec(), b.1.to_vec())
}

fn main() {
    unsafe {
        env::set_var("RUST_BACKTRACE", "full");
    }
    let mut rng = rand::thread_rng();
    let numbers: Vec<f64> = (0..=45).map(|_| rng.gen_range(0.0..=70.0)).collect();
    let numbers_copy = numbers.clone();
    let numbers_len = numbers_copy.len() - 1;

    //    merge_sort(&mut numbers_copy, 0, numbers_len);
    //    println!("Merge sort test: {:#?}", numbers_copy);

    let random_even_index = rng.gen_range(0..=numbers_len / 2) * 2;

    // SPLIT
    let (left_part, right_part) = numbers_copy.split_at(random_even_index);
    let mut left_part = left_part.to_vec();
    let mut right_part = right_part.to_vec();
    let left_part_len = left_part.len();
    let right_part_len = right_part.len();
    // ASC
    quick_sort(&mut left_part, 0, left_part_len - 1);

    //DESC
    quick_sort(&mut right_part, 0, right_part_len - 1);
    right_part.reverse();

    // LEFT SUM
    let left_sum: f64 = left_part.iter().sum();

    // AVG
    let right_mean: f64 = if !right_part.is_empty() {
        right_part.iter().sum::<f64>() / right_part.len() as f64
    } else {
        0.0
    };

    println!("Sorted array: {:#?}", numbers_copy);
    println!("Random even index: {}", random_even_index);
    println!("Left part: {:#?}", left_part);
    println!("Right part: {:#?}", right_part);
    println!("Sum of left part: {}", left_sum);
    println!("Arithmetic mean of right part: {}", right_mean);
}
