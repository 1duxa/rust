pub fn generate_sequences(n: i32, k: i32, start: i32, current_sequence: &mut Vec<i32>) {
    if current_sequence.len() == (k as usize) {
        println!("{:?}", current_sequence);
        return;
    }

    for i in start..=n {
        current_sequence.insert(0, i);
        generate_sequences(n, k, i + 1, current_sequence);
        current_sequence.remove(0);
    }
}

pub fn print_increasing_sequences(n: i32, k: i32) {
    let mut current_sequence: Vec<i32> = Vec::new();
    generate_sequences(n, k, 1, &mut current_sequence);
}

fn main() {
    print_increasing_sequences(5, 3);
}
