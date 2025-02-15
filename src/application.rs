use std::rc::Rc;

use crate::{
    data_collection::Collect,
    enums::labelling::Labelling,
    models::{group::Group, topics::Topic},
};

struct Application {
    state: AppState,
}

struct AppState {
    date: u64,
    groups: Rc<Vec<Group>>,
    labelling: Labelling, 
}

impl Application {
    pub fn new() -> Self {
        Self {
            state: AppState {
                date: 0,
                groups: Rc::new(Vec::new()),
                labelling: Labelling::Numeric,
            },
        }
    }
    pub fn run() {
        println!("Enter topics and type 'done' when done");
        loop {
            let topic = Topic::collect();
        }
    }
    fn label_gen(&self) -> String {
        match self.state.labelling {
            Labelling::Numeric => {
                let num_label_gen = || (self.state.groups.len() + 1).to_string();
                num_label_gen()
            },
            Labelling::Alphabetic => todo!(),
            Labelling::Alphanumeric => todo!(),
        }
    }
}