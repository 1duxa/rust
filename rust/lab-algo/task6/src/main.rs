use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};

pub fn init_map() -> HashMap<String, (DateTime<Utc>, DateTime<Utc>)> {
    let mut activities: HashMap<String, (DateTime<Utc>, DateTime<Utc>)> = HashMap::new();
    activities.insert(
        "Math".to_string(),
        (Utc::now(), Utc::now() + Duration::minutes(20)),
    );
    activities.insert(
        "Physics".to_string(),
        (
            Utc::now() - Duration::minutes(30),
            Utc::now() + Duration::minutes(30),
        ),
    );
    activities.insert(
        "Singing".to_string(),
        (
            Utc::now() + Duration::hours(1),
            Utc::now() + Duration::hours(2),
        ),
    );
    activities.insert(
        "Helloing".to_string(),
        (
            Utc::now() + Duration::hours(3),
            Utc::now() + Duration::hours(4),
        ),
    );
    activities.insert(
        "Ma".to_string(),
        (
            Utc::now() + Duration::hours(4),
            Utc::now() + Duration::minutes(30),
        ),
    );
    activities.insert(
        "Sigma".to_string(),
        (
            Utc::now() + Duration::hours(5),
            Utc::now() + Duration::hours(6),
        ),
    );
    activities.insert(
        "Skibidi".to_string(),
        (
            Utc::now() + Duration::hours(2),
            Utc::now() + Duration::hours(3),
        ),
    );
    activities.insert(
        "Zari".to_string(),
        (
            Utc::now() + Duration::hours(6),
            Utc::now() + Duration::hours(7),
        ),
    );
    activities
}
pub fn display_subjects(subjects: HashMap<String, (DateTime<Utc>, DateTime<Utc>)>) {
    for subject in subjects {
        println!(
            "SBJ: {}, S: {} | E: {}",
            subject.0,
            subject.1 .0.time(),
            subject.1 .1.time()
        );
    }
}
pub fn sort(subj: HashMap<String, (DateTime<Utc>, DateTime<Utc>)>) {}
fn main() {
    let map = init_map();
    display_subjects(map);
}
