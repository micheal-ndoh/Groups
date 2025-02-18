use crate::data_collection::DataCollection;
use crate::traits::gen_data_display::GenDataDisplay;
use crate::traits::gen_data_id::GenDataId;
use crate::traits::collect::Collect;
use crate::enums::difficulty::Difficulty;
use cli_table::{format::Justify, print_stdout, Cell, Color, Style, Table};

#[derive(Debug, Clone, Table)]
pub struct Topic {
    #[table(title = "ID", color= "Color::Red", justify = "Justify::Right" )]
    id: u32,
    #[table(title = "Title", color = "Color::Green", justify = "Justify::Center")]
    title: String,
    #[table(title = "Difficulty" )]
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

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_difficulty(&self) -> Difficulty {
        self.difficulty.clone()
    }
}

impl GenDataId<u32> for Topic {
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Collect for Topic {
    fn collect() -> Self {
        let mut topic = Self::new();
        topic.title = DataCollection::input("Enter topic title: ");

        let difficulty =  DataCollection::input("Enter difficulty (Easy, Medium, Hard): ");
        topic.difficulty = Difficulty::from(difficulty.as_str());
        topic
        //     "Medium" => Difficulty::Medium,
        //     "Hard" => Difficulty::Hard,
        //     _ => Difficulty::Easy,
        // };
        // Self {
        //     id: 0,
        //     title,
        //     difficulty,
        // }
    }
}

impl GenDataDisplay for Topic {
   fn display(){}
    
}