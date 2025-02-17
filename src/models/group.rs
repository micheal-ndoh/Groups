use crate::{application::Helper, models::{students::Students, topics::Topic}};

// use super::{students::Students, topic::Topic};
#[derive(Debug)]
pub struct Group {
    id: u32,
    label: String,
    topic: Topic,
    students: Vec<Students>,
    generated_at: u64
}

impl Group {
    pub fn from(label: String, topic: Topic) -> Self {
        Self {
            id: 0,
            label,
            topic,
            students: Vec::new(),
            generated_at: Helper::now_in_secs(),
        }
    }
}
