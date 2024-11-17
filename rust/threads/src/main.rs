use std::thread;

fn main() {
    let nums = Vec::from_iter(0..100);

    thread::scope(|s| {
        s.spawn(|| {
            for i in &nums {
                print!("|{i} {:?}|", thread::current().id());
            }
        });
        s.spawn(|| {
            for i in &nums {
                print!("|{i} {:?}|", thread::current().id());
            }
        });
    });
}
