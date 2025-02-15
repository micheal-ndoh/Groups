use crate::enums::difficulty::Difficulty;

use crate::data_collection::{Collect, DataCollection};

#[derive(Debug, Clone)]
pub struct Topic {
    id: u32,
    title: String,
    difficulty: Difficulty,
}

impl Topic {
    pub fn new() -> Self {
        Self {
            id: 0,
            title: String::from(""),
            difficulty: Difficulty::Easy,
        }
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_difficulty(&self) -> Difficulty {
        self.difficulty.clone()
    }
}

impl Collect for Topic {
    fn collect() -> Self {
        let mut topic = Self::new();
        // println!("Enter topic. \n type done when completed");
        let prompt = Some("Enter topic title".to_string());
        topic.title = DataCollection::input(prompt);

        let prompt: Option<String> =
            Some("Enter topic difficulty (easy, medium, hard)".to_string());
        let difficulty: String = DataCollection::input(prompt);
        topic.difficulty = Difficulty::from(difficulty.as_str());

        topic
    }
}
