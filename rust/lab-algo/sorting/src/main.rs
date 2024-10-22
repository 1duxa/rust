use rand::Rng;
use std::{env, usize};

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
    let mut Arr: Vec<f64> = vec![0.0; end - start + 1];
    let mut k = 0;

    while p <= mid && q <= end {
        if A[p] < A[q] {
            Arr[k] = A[p];
            p += 1;
        } else {
            Arr[k] = A[q];
            q += 1;
        }
        k += 1;
    }

    while p <= mid {
        Arr[k] = A[p];
        p += 1;
        k += 1;
    }

    while q <= end {
        Arr[k] = A[q];
        q += 1;
        k += 1;
    }

    for i in 0..Arr.len() {
        A[start + i] = Arr[i];
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
    env::set_var("RUST_BACKTRACE", "full");
    let mut rng = rand::thread_rng();
    let numbers: Vec<f64> = (0..=45).map(|_| rng.gen_range(0.0..=70.0)).collect();
    let mut numbers_copy = numbers.clone();
    let numbers_len = numbers_copy.len() - 1;

    merge_sort(&mut numbers_copy, 0, numbers_len);
    println!("{:#?}", numbers_copy);

    let res = split_vec_in_random_place(numbers);
    println!("Vec 1 {:#?}   \n\t\r   Vec 2 {:#?}", res.0, res.1);

    let mut part_1 = res.0;
    let part_1_len = part_1.len();
    merge_sort(&mut part_1, 0, part_1_len - 1);
    println!("{:#?}", part_1);

    let mut part_2 = res.1;
    let part_2_len = part_2.len();
    merge_sort(&mut part_2, 0, part_2_len - 1);
    println!("{:#?}", part_2);
}
