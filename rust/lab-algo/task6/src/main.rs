use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone)]
pub struct Activity {
    name: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    priority: i32,
}

pub fn init_map() -> Vec<Activity> {
    vec![
        Activity {
            name: "Math".to_string(),
            start: Utc::now(),
            end: Utc::now() + Duration::minutes(20),
            priority: 1,
        },
        Activity {
            name: "Physics".to_string(),
            start: Utc::now() - Duration::minutes(30),
            end: Utc::now() + Duration::minutes(30),
            priority: 2,
        },
        Activity {
            name: "Singing".to_string(),
            start: Utc::now() + Duration::hours(1),
            end: Utc::now() + Duration::hours(2),
            priority: 1,
        },
        Activity {
            name: "ASD".to_string(),
            start: Utc::now() - Duration::hours(3),
            end: Utc::now() - Duration::hours(2),
            priority: 1,
        },
        Activity {
            name: "GCP".to_string(),
            start: Utc::now() - Duration::minutes(30),
            end: Utc::now() + Duration::minutes(30),
            priority: 2,
        },
    ]
}

pub fn select_priority_activities(mut activities: Vec<Activity>) -> Vec<Activity> {
    activities.sort_by(|a, b| a.end.cmp(&b.end).then(b.priority.cmp(&a.priority)));

    let mut selected_activities: Vec<Activity> = Vec::new();
    let mut last_end_time = Utc::now() - Duration::hours(24);

    for activity in activities {
        if activity.start >= last_end_time {
            selected_activities.push(activity.clone());
            last_end_time = activity.end;
        }
    }

    selected_activities
}

pub fn display_activities(activities: Vec<Activity>) {
    for activity in activities {
        println!(
            "Activity: {}, Start: {}, End: {}, Priority: {}",
            activity.name, activity.start, activity.end, activity.priority
        );
    }
}

fn main() {
    let activities = init_map();
    let selected_activities = select_priority_activities(activities);
    display_activities(selected_activities);
}
