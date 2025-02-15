use std::rc::Rc;

use crate::{data_collection::Collect, enums::labelling::Labelling, models::{group::Group, topics::Topic}};

struct Application {
    state: AppState,
}

struct AppState {
    date: u64,
    groups: Rc<Vec<Group>>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            state: AppState {
                date: 0,
                groups: Rc::new(Vec::new()),
            },
        }
    }
    pub fn run() {
        println!("Enter topis and type 'done' when done");
        loop{
            let topic = Topic::collect();

        }
    }
    fn label_gen(&self) -> String {

        match self.state.labelling {
            Labelling::Numeric => label.gen_num_label(self),
            Labelling::Alphabetic => todo!()
            Labelling::Alphanumeric => todo!()
        } 
    

        fn num_label_gen(cursor: u32) -> String {
            (self.state.group.len() + 1).to_string()

        }
        
    }

}