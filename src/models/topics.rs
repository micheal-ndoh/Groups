use crate::enums::difficulty::{Difficulty};


use crate::data_collection::{Collect, DataCollection};


#[derive(Debug)]
 pub struct Topic {
    id: u32,        
    title: String,
    difficulty: Difficulty,
    
}

impl Collect for Topic {
    fn collect() -> Self {
        println!("Enter topic. \n type done when completed");
        loop {
            let prompt = Some("Enter topic title".to_string());
            let topic = DataCollection::input(prompt);

            let prompt: Option<String> = Some("Enter topic difficulty (easy, medium, hard)".to_string());
            let difficulty: String = DataCollection::input(prompt);
            let difficulty = Difficulty::from(difficulty.as_str());   

                  
            };
        }
    }
    



