fn generate_sequences(n: i32, k: i32, start: i32, current_sequence: String) {
    if current_sequence.matches(',').count() as i32 == k - 1 {
        println!("{}", current_sequence);
        return;
    }

    for i in start..=n {
        let new_sequence = if current_sequence.is_empty() {
            i.to_string()
        } else {
            format!("{},{}", current_sequence, i)
        };
        generate_sequences(n, k, i + 1, new_sequence);
    }
}

fn print_increasing_sequences(n: i32, k: i32) {
    generate_sequences(n, k, 1, String::new());
}

fn main() {
    print_increasing_sequences(5, 4);
}
